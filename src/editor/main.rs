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
    chunks: Vec<opensi::Chunk>,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model { chunks: Vec::new() }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::PackageSelect => {
                let filename = self.file_chooser.get_filename().unwrap();
                let package = opensi::Package::open(filename).expect("Failed to open file");

                self.model.chunks = Vec::new();
                let store_model = to_treestore(&package).expect("convert to TreeStore failed");

                self.tree_view.set_model(Some(&store_model));
            }
            Msg::ItemSelect => {
                let selection = self.tree_view.get_selection();
                if let Some((model, iter)) = selection.get_selected() {}
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
    let store = gtk::TreeStore::new(&[String::static_type(), u32::static_type()]);
    let columns = &[0u32, 1u32];
    let mut chunks = Vec::new();
    let mut i = 0u32;
    let empty = String::new();

    package.rounds.rounds.iter().for_each(|round| {
        let round_parent = store.insert_with_values(None, None, columns, &[&round.name, &i]);
        i += 1;
        chunks.push(opensi::Chunk::Round(round.clone()));

        round.themes.themes.iter().for_each(|theme| {
            let theme_parent =
                store.insert_with_values(Some(&round_parent), None, columns, &[&theme.name, &i]);

            i += 1;
            chunks.push(opensi::Chunk::Theme(theme.clone()));

            theme.questions.questions.iter().for_each(|question| {
                let question_parent = store.insert_with_values(
                    Some(&theme_parent),
                    None,
                    columns,
                    &[&question.price.to_string(), &i],
                );

                i += 1;
                chunks.push(opensi::Chunk::Question(question.clone()));

                let question_title_parent = store.insert_with_values(
                    Some(&question_parent),
                    None,
                    columns,
                    &[&"вопрос".to_owned(), &0u32],
                );

                question.scenario.atoms.iter().for_each(|atom| {
                    i += 1;
                    chunks.push(opensi::Chunk::Atom(atom.clone()));

                    let atom_store = store.insert_with_values(
                        Some(&question_title_parent),
                        None,
                        columns,
                        &[&atom.body.as_ref().unwrap(), &i],
                    );
                });

                let answer_title_parent = store.insert_with_values(
                    Some(&question_parent),
                    None,
                    columns,
                    &[&"ответ".to_owned(), &0u32],
                );


                question.right.answers.iter().for_each(|answer| {
                    i +=1; 
                    chunks.push(opensi::Chunk::Answer(answer.clone()));

                    let title = answer.body.as_ref().unwrap_or(&empty).clone();
                    
                    store.insert_with_values(
                        Some(&answer_title_parent),
                        None,
                        columns,
                        &[&title, &i],
                    );
                })
            })
        });
    });

    Ok(store)
}

fn main() {
    Win::run(()).expect("run failed");
}
