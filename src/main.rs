use dioxus::desktop::Config as DesktopConfig;
use dioxus::prelude::*;

fn main() {
    let config = DesktopConfig::default().with_menu(None);

    LaunchBuilder::new().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "Hello, World!" }
}
