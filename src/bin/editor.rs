use std::io;

use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{
    prelude::GtkListStoreExtManual, CellLayoutExt, ContainerExt, GtkWindowExt, Inhibit,
    TreeModelExt, TreeSelectionExt, TreeStore, TreeView, TreeViewExt, WidgetExt, Window,
    WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

use opensi;

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
            package: opensi::Package::open("tests/data/slamjam2.siq").expect("cant't open package"),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::ItemSelect => {
                let selection = self.tree_view.get_selection();
                if let Some((list_model, iter)) = selection.get_selected() {
                    // let round = list_model
                    //     .get_value(&iter, VALUE_COL)
                    //     .get::<String>()
                    //     .ok()
                    //     .and_then(|value| value)
                    //     .expect("operation failed");

                    // self.tree_view.set_model(Some(&list_model));
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

        let store_model = model
            .package
            .to_treestore()
            .expect("create_and_fill_model failed");
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
// These two constants stand for the columns of the listmodel and the listview
const VALUE_COL: i32 = 0;
const INDEX_COL: i32 = 1;

impl TreeStorable for opensi::Package {
    fn to_treestore(&self) -> io::Result<TreeStore> {
        let store = gtk::TreeStore::new(&[String::static_type(), i32::static_type()]);

        self.rounds.rounds.iter().enumerate().for_each(|(i, round)| {
            store.insert_with_values(
                None,
                None,
                &[VALUE_COL as u32, INDEX_COL as u32],
                &[&round.name, &(i as u32)],
            );
        });

        println!("{:?}", self);
        Ok(store)
    }
}

fn main() {
    Win::run(()).expect("run failed");
}

/// Intended to convert Package parts to gtk::TreeStore
trait TreeStorable {
    // let store = gtk::TreeStore::new(&[String::static_type(), bool::static_type()]);
    fn to_treestore(&self) -> io::Result<TreeStore>;
}
