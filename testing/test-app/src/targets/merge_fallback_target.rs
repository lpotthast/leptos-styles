use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders a merge of two style layers where the higher-priority layer can
/// resolve to `None`, allowing a lower-priority fallback to take over.
///
/// `Set merge orange` re-asserts the higher-priority `Some`; `Clear merge color`
/// drops it back to `None`, so the browser test can verify that the fallback
/// `gray` rendering kicks in only while the upper layer is missing.
#[component]
pub fn MergeFallbackTarget() -> impl IntoView {
    let (color, set_color) = signal(Some("orange".to_string()));

    let styles = Styles::builder()
        .with_optional_unchecked("color", color)
        .build()
        .merge(Styles::new().add_unchecked(" Color ", "gray"));

    view! {
        <section>
            <h2>"merge fallback"</h2>
            <div id="merge-fallback-target" style=styles>"merge fallback"</div>
            <button id="clear-merge-color" on:click=move |_| set_color.set(None)>
                "Clear merge color"
            </button>
            <button
                id="set-merge-orange"
                on:click=move |_| set_color.set(Some("orange".to_string()))
            >
                "Set merge orange"
            </button>
        </section>
    }
}
