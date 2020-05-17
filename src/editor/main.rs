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
    body_container: gtk::Box,
    body_editor: gtk::Entry,
    body_label: gtk::Label,
    image_preview: gtk::Image,
    editor_container: gtk::Box,
    answer_entry: gtk::Entry,
    answer_container: gtk::Box,
    model: Model,
}

struct Model {
    chunks: Vec<Chunk>,
    // TODO: try CoW
    filename: Option<std::path::PathBuf>,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            chunks: Vec::new(),
            filename: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::PackageSelect => {
                let filename = self.file_chooser.get_filename().unwrap();
                let package =
                    opensi::Package::open_with_extraction(&filename).expect("Failed to open file");

                self.model.filename = Some(filename);

                let store = gtk::TreeStore::new(&[String::static_type(), u32::static_type()]);
                let columns = &[0u32, 1u32];
                self.model.chunks = Vec::new();
                let mut i = 0u32;

                package.rounds.rounds.iter().for_each(|round| {
                    let round_parent =
                        store.insert_with_values(None, None, columns, &[&round.name, &i]);
                    i += 1;
                    self.model.chunks.push(Chunk::Round(round.clone()));

                    round.themes.themes.iter().for_each(|theme| {
                        let theme_parent = store.insert_with_values(
                            Some(&round_parent),
                            None,
                            columns,
                            &[&theme.name, &i],
                        );

                        i += 1;
                        self.model.chunks.push(Chunk::Theme(theme.clone()));

                        theme.questions.questions.iter().for_each(|question| {
                            store.insert_with_values(
                                Some(&theme_parent),
                                None,
                                columns,
                                &[&question.price.to_string(), &i],
                            );

                            i += 1;
                            self.model.chunks.push(Chunk::Question(question.clone()));
                        })
                    });
                });

                self.tree_view.set_model(Some(&store));
            }
            Msg::ItemSelect => {
                self.image_preview.set_visible(false);
                self.body_container.set_visible(false);
                self.answer_container.set_visible(false);

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
                        Chunk::Round(x) => {
                            self.body_container.set_visible(true);
                            self.body_editor.set_text(&x.name);
                            self.body_label.set_text("раунд:");

                            println!("{:?}", x);
                        }
                        Chunk::Theme(x) => {
                            self.body_container.set_visible(true);
                            self.body_editor.set_text(&x.name);
                            self.body_label.set_text("тема:");

                            println!("{:?}", x);
                        }
                        Chunk::Question(x) => {
                            println!("{:?}", x);

                            self.body_editor.set_text(
                                &x.scenario.atoms.first().unwrap().body.as_ref().unwrap(),
                            );

                            x.scenario
                                .atoms
                                .iter()
                                .filter(|atom| {
                                    !atom
                                        .variant
                                        .as_ref()
                                        .unwrap_or(&String::from("heh"))
                                        .eq("marker")
                                })
                                .for_each(|atom| {
                                    // empty variant means text atom
                                    if atom.variant.is_none() {
                                        let body = atom.body.as_ref().unwrap();
                                        self.body_container.set_visible(true);
                                        self.body_label.set_text("вопрос:");
                                        self.body_editor.set_text(body);
                                        return;
                                    }

                                    let path = self
                                        .model
                                        .filename
                                        .as_ref()
                                        .and_then(|x| x.file_name())
                                        .and_then(|x| x.to_str())
                                        .unwrap();
                                    
                                    if let Some(resource) = atom.get_resource(path) {
                                        match resource {
                                            opensi::Resource::Image(path) =>  draw_image(&self, path),
                                            _ => {}
                                        }
                                    }
                                });

                            x.right.answers.iter().for_each(|answer| {
                                self.answer_container.set_visible(true);
                                if let Some(body) = answer.body.as_ref() {
                                    self.answer_entry.set_text(body);
                                }
                            })
                        }
                    }
                }
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

fn draw_image(win: &Win, path: std::path::PathBuf) {
    let allocation = win.editor_container.get_allocation();
    let mut pixbuf: gdk_pixbuf::Pixbuf = gdk_pixbuf::Pixbuf::new_from_file(path).unwrap();

    // todo add height scaling
    if pixbuf.get_width() > allocation.width {
        let new_width = allocation.width;
        let ratio = allocation.width as f32 / pixbuf.get_width() as f32;
        let new_height = ((pixbuf.get_height() as f32) * ratio).floor() as i32;

        pixbuf = pixbuf
            .scale_simple(new_width, new_height, gdk_pixbuf::InterpType::Bilinear)
            .unwrap();
    }

    win.image_preview.set_from_pixbuf(Some(pixbuf.as_ref()));
    win.image_preview.set_visible(true);
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
        let image_preview: gtk::Image = builder.get_object("image-preview-editor").unwrap();
        let body_container: gtk::Box = builder.get_object("body-container").unwrap();
        let body_label: gtk::Label = builder.get_object("body-label").unwrap();

        let answer_entry: gtk::Entry = builder.get_object("answer-entry").unwrap();
        let answer_container: gtk::Box = builder.get_object("answer-container").unwrap();

        let editor_container: gtk::Box = builder.get_object("editor-container").unwrap();

        window.show();

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
            image_preview,
            body_container,
            body_label,
            editor_container,
            answer_entry,
            answer_container,
            model,
        }
    }
}

#[derive(Debug)]
pub enum Chunk {
    Round(opensi::Round),
    Theme(opensi::Theme),
    Question(opensi::Question),
}

fn main() {
    Win::run(()).expect("Window failed to run");
}
