use dioxus::prelude::*;

use super::Route;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}