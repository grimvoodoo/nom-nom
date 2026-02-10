use dioxus::prelude::*;

mod components;
mod pages;

use pages::{Home, Recipes, Larder};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/recipes")]
    Recipes {},
    #[route("/larder")]
    Larder {},
}

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-base-100 text-base-content",
            components::Navbar {}
            main { class: "container mx-auto px-4 py-8",
                Outlet::<Route> {}
            }
            components::Footer {}
        }
    }
}
