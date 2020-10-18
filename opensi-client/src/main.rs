use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::Label;

fn build_ui() -> impl Widget<()> {
        Label::new("Hello world")
}

fn main() -> Result<(), PlatformError> {
        AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
            Ok(())
}

