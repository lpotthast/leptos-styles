use leptos::prelude::*;
use leptos_styles::Styles;

/// Verifies the README's central pitch — passing `Styles` through component
/// layers — in the actual Leptos renderer.
///
/// Each layer extends the inherited container with its own entry; the leaf
/// renders the accumulated `style=styles`. The browser test asserts the final
/// declaration order matches the call order (`color` → `padding` → `margin`).
#[component]
pub fn DrillingTarget() -> impl IntoView {
    view! {
        <section>
            <h2>"drilling"</h2>
            <DrillingMid styles=Styles::new().add_unchecked("color", "purple") />
        </section>
    }
}

#[component]
fn DrillingMid(#[prop(into, optional)] styles: Styles) -> impl IntoView {
    view! {
        <DrillingLeaf styles=styles.add_unchecked("padding", "5px") />
    }
}

#[component]
fn DrillingLeaf(#[prop(into, optional)] styles: Styles) -> impl IntoView {
    view! {
        <div id="drilling-target" style=styles.add_unchecked("margin", "10px")>"drilling"</div>
    }
}
