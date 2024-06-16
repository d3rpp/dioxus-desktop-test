use dioxus::prelude::*;

mod home;
pub use home::*;

mod blog;
pub use blog::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}