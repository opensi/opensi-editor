#[path = "../core/lib.rs"]
mod opensi;

use gtk::prelude::*;
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use std::io;

#[derive(Msg)]
enum Msg {
    PackageSelect,
    ItemSelect,
    Quit,
}

struct Win {
    window: gtk::Window,
    file_chooser: gtk::FileChooserButton,
    tree_view: gtk::TreeView,
    model: Model,
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
            Msg::PackageSelect => {
                let filename = self.file_chooser.get_filename().unwrap();
                let package = opensi::Package::open(filename).unwrap();

                let store_model = to_treestore(&package).expect("convert to TreeStore failed");
                self.tree_view.set_model(Some(&store_model));
            }
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
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let source = include_str!("editor.ui");
        let bulider = gtk::Builder::new_from_string(source);
        let window: gtk::Window = bulider.get_object("editor").unwrap();

        let tree_view: gtk::TreeView = bulider.get_object("tree").unwrap();
        let file_chooser: gtk::FileChooserButton = bulider.get_object("file-chooser").unwrap();

        // default model
        let store_model = to_treestore(&model.package).expect("convert to TreeStore failed");
        tree_view.set_model(Some(&store_model));

        window.show_all();

        connect!(relm, file_chooser, connect_file_set(_), Msg::PackageSelect);
        connect!(relm, tree_view, connect_cursor_changed(_), Msg::ItemSelect);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            window,
            file_chooser,
            tree_view,
            model,
        }
    }
}

// ugly, need refactor
fn to_treestore(package: &opensi::Package) -> io::Result<gtk::TreeStore> {
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
