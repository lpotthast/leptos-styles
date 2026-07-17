use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders a single optional style entry that the browser test toggles to
/// `None` and back.
///
/// When all entries resolve to `None` the rendered string is empty, and the
/// `style` attribute should be removed from the element entirely rather than
/// left as `style=""`. Restoring the value should re-create the attribute.
#[component]
pub fn EmptyTarget() -> impl IntoView {
    let (color, set_color) = signal(Some("ephemeral".to_string()));

    let styles = Styles::new().add_optional_unchecked("color", color);

    view! {
        <section>
            <h2>"empty"</h2>
            <div id="empty-target" style=styles>"empty"</div>
            <button id="empty-clear" on:click=move |_| set_color.set(None)>
                "Clear style"
            </button>
            <button
                id="empty-restore"
                on:click=move |_| set_color.set(Some("ephemeral".to_string()))
            >
                "Restore style"
            </button>
        </section>
    }
}
