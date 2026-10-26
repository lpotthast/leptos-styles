use leptos::prelude::*;
use leptos_styles::{
    Styles, UncheckedDeclaration,
    css::{
        LengthPercentage, custom_property,
        property::{PADDING, PADDING_TOP},
        px, rem, var,
    },
};

custom_property!(SPACE: LengthPercentage = "--space");

/// Renders updates that change a longhand next to an unchanged shorthand that
/// the browser cannot write as one value. Each element has a reference element
/// that renders the final declarations directly, so the browser test can check
/// that the update gives the same result as the first render.
///
/// - A `border` with a `border-top-color`, whose color the update removes.
/// - A `padding: var(--space)` with a `padding-top`, which the update removes.
/// - A group whose fallback `padding` stays below an outer `padding-top`.
#[component]
pub fn ShorthandTarget() -> impl IntoView {
    let (accented, set_accented) = signal(true);
    let (wide, set_wide) = signal(false);

    let border = Styles::new()
        .with(UncheckedDeclaration::new("border", "1px solid red"))
        .with(move || {
            accented
                .get()
                .then(|| UncheckedDeclaration::new("border-top-color", "blue"))
        });
    let border_reference = Styles::new().with(UncheckedDeclaration::new("border", "1px solid red"));

    let padding = Styles::new()
        .with(SPACE.declare(rem(1)))
        .with(PADDING.declare(var(&SPACE)))
        .with(move || accented.get().then(|| PADDING_TOP.declare(px(0))));
    let padding_reference = Styles::new()
        .with(SPACE.declare(rem(1)))
        .with(PADDING.declare(var(&SPACE)));

    let wide_group = || {
        Styles::new()
            .with(UncheckedDeclaration::new("color", "red"))
            .with_fallback(Styles::new().with(UncheckedDeclaration::new("padding", "16px")))
    };
    let group = Styles::new()
        .with(UncheckedDeclaration::new("padding-top", "0px"))
        .with(move || {
            if wide.get() {
                wide_group()
            } else {
                Styles::new().with(UncheckedDeclaration::new("color", "blue"))
            }
        });
    let group_reference = Styles::new()
        .with(UncheckedDeclaration::new("padding-top", "0px"))
        .with(wide_group);

    view! {
        <section>
            <h2>"shorthand"</h2>
            <div id="shorthand-border-target" style=border>"border"</div>
            <div id="shorthand-border-reference" style=border_reference>"border"</div>
            <div id="shorthand-padding-target" style=padding>"padding"</div>
            <div id="shorthand-padding-reference" style=padding_reference>"padding"</div>
            <div id="shorthand-group-target" style=group>"group"</div>
            <div id="shorthand-group-reference" style=group_reference>"group"</div>
            <button id="shorthand-remove-accent" on:click=move |_| set_accented.set(false)>
                "Remove the accent"
            </button>
            <button id="shorthand-widen" on:click=move |_| set_wide.set(true)>
                "Widen the group"
            </button>
        </section>
    }
}
