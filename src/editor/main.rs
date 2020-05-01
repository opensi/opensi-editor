#[path = "../core/lib.rs"]
mod opensi;

use gtk::prelude::*;
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

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
    body_editor: gtk::Entry,
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

                let store = gtk::TreeStore::new(&[String::static_type(), u32::static_type()]);
                let columns = &[0u32, 1u32];
                self.model.chunks = Vec::new();
                let mut i = 0u32;
                let empty = String::new();

                package.rounds.rounds.iter().for_each(|round| {
                    let round_parent =
                        store.insert_with_values(None, None, columns, &[&round.name, &i]);
                    i += 1;
                    self.model.chunks.push(opensi::Chunk::Round(round.clone()));

                    round.themes.themes.iter().for_each(|theme| {
                        let theme_parent = store.insert_with_values(
                            Some(&round_parent),
                            None,
                            columns,
                            &[&theme.name, &i],
                        );

                        i += 1;
                        self.model.chunks.push(opensi::Chunk::Theme(theme.clone()));

                        theme.questions.questions.iter().for_each(|question| {
                            let question_parent = store.insert_with_values(
                                Some(&theme_parent),
                                None,
                                columns,
                                &[&question.price.to_string(), &i],
                            );

                            i += 1;
                            self.model
                                .chunks
                                .push(opensi::Chunk::Question(question.clone()));

                            let question_title_parent = store.insert_with_values(
                                Some(&question_parent),
                                None,
                                columns,
                                &[&"вопрос".to_owned(), &0u32],
                            );

                            question.scenario.atoms.iter().for_each(|atom| {
                                i += 1;
                                self.model.chunks.push(opensi::Chunk::Atom(atom.clone()));

                                store.insert_with_values(
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
                                i += 1;
                                self.model
                                    .chunks
                                    .push(opensi::Chunk::Answer(answer.clone()));

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

                self.tree_view.set_model(Some(&store));
            }
            Msg::ItemSelect => {
                let selection = self.tree_view.get_selection();
                if let Some((model, iter)) = selection.get_selected() {
                    let index = model
                        .get_value(&iter, 1)
                        .get::<u32>()
                        .ok()
                        .and_then(|value| value)
                        .expect("get_value.get<String> failed");

                    let chunk = &self.model.chunks[index as usize];

                    match chunk {
                        opensi::Chunk::Package(x) => {
                            let package_name = x.name.as_ref().unwrap();
                            self.body_editor.set_text(package_name);
                            println!("{:?}", x)
                        },
                        opensi::Chunk::Info(x) => println!("{:?}", x),
                        opensi::Chunk::Authors(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Rounds(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Round(x) => {
                            self.body_editor.set_text(&x.name);
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Theme(x) => {
                            self.body_editor.set_text(&x.name);
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Themes(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Questions(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Question(x) => {
                            self.body_editor.set_text(&x.scenario.atoms.first().unwrap().body.as_ref().unwrap());
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Variant(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Scenario(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Right(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Wrong(x) => {
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Answer(x) => {
                            // self.body_editor.set_visible(false);
                            println!("{:?}", x);
                        }
                        opensi::Chunk::Atom(x) => {
                            println!("{:?}", x);
                        }
                    }
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
        let builder = gtk::Builder::new_from_string(source);
        let window: gtk::Window = builder.get_object("editor").unwrap();

        let tree_view: gtk::TreeView = builder.get_object("tree").unwrap();
        let file_chooser: gtk::FileChooserButton = builder.get_object("file-chooser").unwrap();
        let body_editor: gtk::Entry = builder.get_object("body-editor").unwrap();

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
            body_editor,
            model,
        }
    }
}

fn main() {
    Win::run(()).expect("run failed");
}
