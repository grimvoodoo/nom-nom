use dioxus::prelude::*;
use crate::Route;
use super::ThemeToggle;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar bg-base-200",
            div { class: "flex-1",
                Link { to: Route::Home {}, class: "btn btn-ghost text-xl", "🍳 nom-nom" }
            }
            div { class: "flex-none gap-2",
                ul { class: "menu menu-horizontal px-1",
                    li {
                        Link { to: Route::Recipes {}, "Recipes" }
                    }
                    li {
                        Link { to: Route::Larder {}, "Larder" }
                    }
                }
                ThemeToggle {}
            }
        }
    }
}
