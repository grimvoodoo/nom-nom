use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer footer-center p-4 bg-base-200 text-base-content",
            div {
                p { "nom-nom — A smart cookbook that knows what's in your kitchen" }
            }
        }
    }
}
