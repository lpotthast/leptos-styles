use leptos::prelude::*;
use leptos_styles::{
    css::{CssColor, css_custom_property, rgb},
    property::{BackgroundColorProperty, ColorProperty},
};

css_custom_property!(DIRECT_ACCENT: CssColor = "--DirectAccent");

/// Drives a direct checked declaration through its complete DOM lifecycle.
#[component]
pub fn CheckedDeclarationTarget() -> impl IntoView {
    let (step, set_step) = signal(0_u8);

    view! {
        <section>
            <h2>"checked declaration lifecycle"</h2>
            <div
                id="checked-declaration-target"
                style=move || match step.get() {
                    0 => Some(ColorProperty.declare(rgb(255, 0, 0))),
                    1 => Some(ColorProperty.declare(rgb(0, 0, 255))),
                    2 => Some(BackgroundColorProperty.declare(rgb(0, 128, 0))),
                    3 => Some(DIRECT_ACCENT.declare(rgb(255, 0, 255))),
                    _ => None,
                }
            >
                "direct checked declaration"
            </div>
            <button id="checked-value-update" on:click=move |_| set_step.set(1)>
                "Update checked value"
            </button>
            <button id="checked-property-update" on:click=move |_| set_step.set(2)>
                "Update checked property"
            </button>
            <button id="checked-custom-property" on:click=move |_| set_step.set(3)>
                "Use checked custom property"
            </button>
            <button id="checked-reset" on:click=move |_| set_step.set(4)>
                "Reset checked declaration"
            </button>
        </section>
    }
}
