use dioxus::prelude::*;

#[component]
pub fn Larder() -> Element {
    rsx! {
        div {
            h1 { class: "text-3xl font-bold mb-6", "My Larder" }
            p { class: "text-base-content/70", "Ingredient tracking coming soon..." }
        }
    }
}
