use leptos::prelude::*;
use leptos_styles::{StyleEntry, UncheckedDeclaration};

/// Renders an unchecked declaration and a reactive entry directly with
/// `style=`, without building a `Styles`.
///
/// Checks that both render, hydrate, and update like a `Styles` that holds
/// them.
#[component]
pub fn SingleTarget() -> impl IntoView {
    let (wide, set_wide) = signal(false);
    let entry = StyleEntry::new(move || {
        UncheckedDeclaration::new("width", if wide.get() { "200px" } else { "100px" })
    });

    view! {
        <section>
            <h2>"single"</h2>
            <div id="single-unchecked-target" style=UncheckedDeclaration::new("contain", "layout")>
                "unchecked"
            </div>
            <div id="single-entry-target" style=entry>"entry"</div>
            <button id="single-widen" on:click=move |_| set_wide.set(true)>"Widen"</button>
        </section>
    }
}
