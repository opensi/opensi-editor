#[path = "../core/lib.rs"]
mod opensi;

use std::io;
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{
    CellLayoutExt, ContainerExt, GtkWindowExt, Inhibit, TreeStore, TreeView, TreeViewExt,
    WidgetExt, Window, WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

#[derive(Msg)]
enum Msg {
    ItemSelect,
    Quit,
}

struct Win {
    tree_view: TreeView,
    model: Model,
    window: Window,
}
struct Model {
    package: opensi::Package,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            // TODO: will be replaced with actual data in future
            package: opensi::Package::open("tests/data/slamjam2.siq").expect("cant't open package"),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::ItemSelect => {
                let selection = self.tree_view.get_selection();
                if let Some((list_model, iter)) = selection.get_selected() {
                    // TODO: add action here 
                }
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = gtk::Window::new(WindowType::Toplevel);
        let vbox = gtk::Box::new(Vertical, 0);
        let tree_view = gtk::TreeView::new();
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();

        window.set_title("Package editor");
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);

        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        tree_view.append_column(&column);

        let store_model = to_treestore(&model.package).expect("convert to TreeStore failed");
        tree_view.set_model(Some(&store_model));

        vbox.add(&tree_view);
        window.add(&vbox);

        window.show_all();

        connect!(relm, tree_view, connect_cursor_changed(_), Msg::ItemSelect);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            tree_view,
            model,
            window,
        }
    }
}

// ugly, need refactor
fn to_treestore(package: &opensi::Package) -> io::Result<TreeStore> {
    let store = gtk::TreeStore::new(&[String::static_type()]);

    package.rounds.rounds.iter().for_each(|round| {
        let round_parent = store.insert_with_values(None, None, &[0 as u32], &[&round.name]);

        round.themes.themes.iter().for_each(|theme| {
            let theme_parent =
                store.insert_with_values(Some(&round_parent), None, &[0 as u32], &[&theme.name]);

            theme.questions.questions.iter().for_each(|question| {
                let atom = &question
                    .scenario
                    .atoms
                    .first()
                    .expect("failed to extract atom");
                let title = atom.body.as_ref().expect("heh");
                let title = format!("{} ({})", title, question.price);

                let atom_store =
                    store.insert_with_values(Some(&theme_parent), None, &[0 as u32], &[&title]);

                let empty = String::from("");
                let answers = &question
                    .right
                    .answers
                    .iter()
                    .map(|answer| answer.body.as_ref().unwrap_or(&empty).clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                store.insert_with_values(Some(&atom_store), None, &[0 as u32], &[&answers]);
            })
        });
    });

    // add flag for printing only for debug version
    println!("{:?}", package);
    Ok(store)
}

fn main() {
    Win::run(()).expect("run failed");
}
