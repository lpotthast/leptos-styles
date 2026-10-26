use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::property::{HEIGHT, PADDING, WIDTH},
    css::{min, pct, px, rem, vh},
};

/// Renders grammar-checked CSS expression values.
///
/// The browser test asserts the browser's parsed form so regressions in the
/// typed calculation writer are caught end-to-end, including the `calc()`
/// wrapper that keeps a negative padding valid.
#[component]
pub fn CalcTarget() -> impl IntoView {
    let styles = Styles::new()
        .with(WIDTH.declare(pct(100) - px(20)))
        .with(HEIGHT.declare(min(vh(50) + rem(1), px(400))))
        .with(PADDING.declare(px(4) - px(8)));

    view! {
        <section>
            <h2>"calc"</h2>
            <div id="calc-target" style=styles>"calc"</div>
        </section>
    }
}
