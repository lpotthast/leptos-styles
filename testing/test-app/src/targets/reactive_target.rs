use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders an optional style backed by a signal.
///
/// Buttons mutate the signal so the browser test can assert the rendered
/// `style` attribute updates reactively, and that resetting to `None` removes
/// the property entirely instead of leaving an empty value.
#[component]
pub fn ReactiveTarget() -> impl IntoView {
    let (color, set_color) = signal(Some("tomato".to_string()));

    let styles = Styles::builder()
        .with_unchecked("display", "grid")
        .with_optional_unchecked("color", move || color.get())
        .build();

    view! {
        <section>
            <h2>"reactive"</h2>
            <div id="reactive-target" style=styles>"reactive"</div>
            <button
                id="set-royalblue"
                on:click=move |_| set_color.set(Some("royalblue".to_string()))
            >
                "Set royalblue"
            </button>
            <button id="clear-color" on:click=move |_| set_color.set(None)>
                "Clear color"
            </button>
        </section>
    }
}
