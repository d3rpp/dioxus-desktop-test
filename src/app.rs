use dioxus::prelude::*;

use crate::routes::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
