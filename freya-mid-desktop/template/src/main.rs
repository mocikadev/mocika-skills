use freya::prelude::*;
use tokio::runtime::Runtime;

mod app;
mod components;
mod core;
mod theme;
mod views;

fn main() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter();

    launch(LaunchConfig::new().with_window(WindowConfig::new(app::app)));
}
