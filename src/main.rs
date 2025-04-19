mod error;
mod path;

use dioxus::desktop::{
    Config as DesktopConfig,
    WindowBuilder,
};
use dioxus::prelude::*;

use crate::error::Error;
use crate::path::app_data_local_dir;

const APP_NAME: &str = "Warframe Builder";
const APP_IDENTIFIER: &str = "app.warframe-builder";

fn main() -> Result<(), Error> {
    let data_directory = app_data_local_dir()?;

    let window = WindowBuilder::default().with_title(APP_NAME);
    let config = DesktopConfig::default()
        .with_data_directory(data_directory)
        .with_menu(None)
        .with_window(window);

    LaunchBuilder::new().with_cfg(config).launch(App);

    Ok(())
}

#[component]
fn App() -> Element {
    rsx! { "Hello, World!" }
}
