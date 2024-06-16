#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod routes;
mod app;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_menu(None)
        .with_custom_head(r#"<link rel="stylesheet" href="/assets/tailwind.css">"#.to_string())
        .with_disable_context_menu(true)
        .with_close_behaviour(dioxus::desktop::WindowCloseBehaviour::LastWindowExitsApp);
    
    LaunchBuilder::desktop().with_cfg(cfg).launch(app::App);
}

