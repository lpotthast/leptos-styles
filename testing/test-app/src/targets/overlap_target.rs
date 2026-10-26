use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Combines a `padding` shorthand in `Styles` with a `style:padding-top`
/// directive on the same elements.
///
/// None of the updates touches the unchanged padding, so the directive keeps
/// its `padding-top`:
///
/// - The first element changes its color, which comes after the padding.
/// - The second element removes a trailing optional color.
/// - The third element changes a color that comes before a property that the
///   browser does not know.
#[component]
pub fn OverlapTarget() -> impl IntoView {
    let (color, set_color) = signal("navy");
    let (colored, set_colored) = signal(true);
    let styles = Styles::new()
        .with(UncheckedDeclaration::new("padding", "8px"))
        .with(move || UncheckedDeclaration::new("color", color.get()));
    let optional = Styles::new()
        .with(UncheckedDeclaration::new("padding", "8px"))
        .with(move || {
            colored
                .get()
                .then(|| UncheckedDeclaration::new("color", "navy"))
        });
    let unknown = Styles::new()
        .with(UncheckedDeclaration::new("padding", "8px"))
        .with(move || UncheckedDeclaration::new("color", color.get()))
        .with(UncheckedDeclaration::new("-unknown-smoothing", "grayscale"));

    view! {
        <section>
            <h2>"overlap"</h2>
            <div id="overlap-target" style=styles style:padding-top="1px">
                "overlap"
            </div>
            <div id="overlap-optional-target" style=optional style:padding-top="1px">
                "optional"
            </div>
            <div id="overlap-unknown-target" style=unknown style:padding-top="1px">
                "unknown"
            </div>
            <button id="overlap-set-maroon" on:click=move |_| set_color.set("maroon")>
                "Set maroon"
            </button>
            <button id="overlap-remove-color" on:click=move |_| set_colored.set(false)>
                "Remove the color"
            </button>
        </section>
    }
}
