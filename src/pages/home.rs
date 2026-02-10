use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "hero min-h-[60vh]",
            div { class: "hero-content text-center",
                div { class: "max-w-md",
                    h1 { class: "text-5xl font-bold", "🍳 nom-nom" }
                    p { class: "py-6",
                        "A smart cookbook that knows what's in your kitchen. Track your ingredients, discover recipes you can make right now."
                    }
                    div { class: "flex gap-4 justify-center",
                        Link {
                            to: Route::Recipes {},
                            class: "btn btn-primary",
                            "Browse Recipes"
                        }
                        Link {
                            to: Route::Larder {},
                            class: "btn btn-outline",
                            "My Larder"
                        }
                    }
                }
            }
        }
    }
}
