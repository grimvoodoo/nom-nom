use dioxus::prelude::*;

#[component]
pub fn Recipes() -> Element {
    rsx! {
        div {
            h1 { class: "text-3xl font-bold mb-6", "Recipes" }
            p { class: "text-base-content/70", "Recipe management coming soon..." }
        }
    }
}
