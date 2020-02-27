use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{Inhibit, WidgetExt, Window, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

struct Model {}

#[derive(Msg)]
enum Msg {
    Quit,
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
    window: Window,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("opensi editor");
        window.set_property_default_width(640);
        window.set_property_default_height(480);

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        window.show_all();

        Win {
            model,
            widgets: Widgets { window: window },
        }
    }
}

fn main() {
    Win::run(()).expect("run failed");
}
