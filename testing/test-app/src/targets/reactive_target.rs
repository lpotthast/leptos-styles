use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders an optional style backed by a signal.
///
/// Buttons mutate the signal so the browser test can assert the rendered
/// `style` attribute updates reactively, and that resetting to `None` removes
/// the property entirely instead of leaving an empty value.
#[component]
pub fn ReactiveTarget() -> impl IntoView {
    let (color, set_color) = signal(Some("tomato"));

    let styles = Styles::new()
        .with(UncheckedDeclaration::new("display", "grid"))
        .with(move || {
            color
                .get()
                .map(|color| UncheckedDeclaration::new("color", color))
        });

    view! {
        <section>
            <h2>"reactive"</h2>
            <div id="reactive-target" style=styles>"reactive"</div>
            <button id="set-royalblue" on:click=move |_| set_color.set(Some("royalblue"))>
                "Set royalblue"
            </button>
            <button id="clear-color" on:click=move |_| set_color.set(None)>
                "Clear color"
            </button>
        </section>
    }
}
