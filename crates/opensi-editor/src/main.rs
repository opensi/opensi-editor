#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
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
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    let start_result = eframe::WebRunner::new()
        .start(
            "the_canvas_id",
            web_options,
            Box::new(|cc| Ok(Box::new(opensi_editor::EditorApp::new(cc)))),
        )
        .await;

    // Remove the loading text and spinner:
    let loading_text = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("loading_text"));
    if let Some(loading_text) = loading_text {
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
