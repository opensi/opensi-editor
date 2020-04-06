use gtk::prelude::*;
use gtk::{ButtonExt, CssProvider, Inhibit, StyleContext, WidgetExt, Window};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

#[derive(Msg)]
enum Msg {
    Quit,
}

struct Model {
    //
}

struct Win {
    model: Model,
    window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn init_view(&mut self) {
        let provider = CssProvider::new();
        provider
            .load_from_path("resource/client.css")
            .expect("Failed to load CSS");

        StyleContext::add_provider_for_screen(
            &self
                .window
                .get_screen()
                .expect("Error initializing application style"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let source = include_str!("main.glade");
        let builder = gtk::Builder::new_from_string(source);
        let window: gtk::Window = builder.get_object("client").unwrap();

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        let profile_select: gtk::Widget = builder.get_object("profile_select").unwrap();
        profile_select.connect_enter_notify_event(|widget, _| {
            widget.get_style_context().set_state(gtk::StateFlags::PRELIGHT);
            Inhibit(true)
        });
        profile_select.connect_leave_notify_event(|widget, _| {
            widget.get_style_context().set_state(gtk::StateFlags::empty());
            Inhibit(true)
        });

        let button_exit: gtk::Button = builder.get_object("button_exit").unwrap();
        connect!(relm, button_exit, connect_clicked(_), Msg::Quit);

        window.show_all();

        Win { model, window }
    }
}

fn main() {
    Win::run(()).unwrap();
}
