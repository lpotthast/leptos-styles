use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders a single optional style entry that the browser test toggles to
/// `None` and back.
///
/// When every entry resolves to `None`, `Styles` removes the `style` attribute
/// instead of leaving `style=""`. Restoring the value creates the attribute
/// again.
#[component]
pub fn EmptyTarget() -> impl IntoView {
    let (color, set_color) = signal(Some("teal"));
    let styles = Styles::new().with(move || {
        color
            .get()
            .map(|color| UncheckedDeclaration::new("color", color))
    });

    view! {
        <section>
            <h2>"empty"</h2>
            <div id="empty-target" style=styles>"empty"</div>
            <button id="empty-clear" on:click=move |_| set_color.set(None)>
                "Clear style"
            </button>
            <button id="empty-restore" on:click=move |_| set_color.set(Some("teal"))>
                "Restore style"
            </button>
        </section>
    }
}
