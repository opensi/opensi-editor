#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn setup_logger() -> Result<(), fern::InitError> {
    use fern::Dispatch;

    #[cfg(not(target_arch = "wasm32"))]
    let dispatch = {
        use fern::colors::{Color, ColoredLevelConfig};
        use std::time::SystemTime;

        let colors =
            ColoredLevelConfig::new().info(Color::Green).error(Color::Red).warn(Color::Yellow);

        Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {}] {}",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    colors.color(record.level()),
                    message
                ))
            })
            .level(log::LevelFilter::Info)
            .chain(std::io::stdout())
    };

    #[cfg(target_arch = "wasm32")]
    let dispatch = {
        Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!("[{}] {}", record.level(), message))
            })
            .level(log::LevelFilter::Info)
            .chain(Box::new(eframe::WebLogger::new(log::LevelFilter::Info)) as Box<dyn log::Log>)
    };

    dispatch.apply()?;
    Ok(())
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result {
    setup_logger().expect("logger init");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        centered: true,
        persist_window: true,
        ..Default::default()
    };
    eframe::run_native(
        "OpenSI Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(opensi_editor::EditorApp::new(cc)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    setup_logger().expect("logger init");

    let web_options = eframe::WebOptions::default();

    let document = web_sys::window().expect("No window").document().expect("No document");
    let canvas = document
        .get_element_by_id("the_canvas_id")
        .expect("Failed to find the_canvas_id")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("the_canvas_id was not a HtmlCanvasElement");

    let start_result = eframe::WebRunner::new()
        .start(canvas, web_options, Box::new(|cc| Ok(Box::new(opensi_editor::EditorApp::new(cc)))))
        .await;

    // Remove the loading text and spinner:
    if let Some(loading_text) = document.get_element_by_id("loading_text") {
        match start_result {
            Ok(_) => {
                loading_text.remove();
            },
            Err(e) => {
                loading_text.set_inner_html(
                    "<p> The app has crashed. See the developer console for details. </p>",
                );
                panic!("Failed to start eframe: {e:?}");
            },
        }
    }
}
