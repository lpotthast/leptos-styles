//! Integration tests for checked and unchecked declarations through `Styles`.

use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use assertr::prelude::*;
use leptos::prelude::*;
use leptos_styles::{
    StyleEntry, Styles, UncheckedDeclaration, css,
    css::property::{
        ALL, BACKGROUND_COLOR, COLOR, DISPLAY, GAP, HEIGHT, MARGIN, PADDING, RIGHT, TOP, WIDTH,
    },
    css::{
        Auto, Color, CustomProperty, Declaration, LengthPercentage, Margin, NamedColor, Property,
        custom_property, em, pct, px, rem, rgb, var, var_or, vh, vw,
    },
};

custom_property!(ACCENT_COLOR: Color = "--AccentColor");
custom_property!(SPACE: LengthPercentage = "--space");

#[test]
fn checked_property_value_pairs_render_canonical_css() {
    let styles = Styles::new()
        .with(WIDTH.declare(px(320)))
        .with(HEIGHT.declare(vh(40)))
        .with(PADDING.declare(em(1.5)))
        .with(MARGIN.declare(Margin::symmetric(rem(2), Auto)))
        .with(RIGHT.declare(vw(10)));

    assert_that!(styles.to_style_string())
        .is_equal_to("width:320px;height:40vh;padding:1.5em;margin:2rem auto;right:10vw;");
}

#[test]
fn one_selector_type_serves_every_property_of_a_grammar() {
    fn tinted(styles: Styles, target: Property<Color>) -> Styles {
        styles.with(target.declare(rgb(255, 0, 0)))
    }

    let styles = tinted(tinted(Styles::new(), COLOR), BACKGROUND_COLOR);
    assert_that!(styles.to_style_string())
        .is_equal_to("color:rgb(255 0 0);background-color:rgb(255 0 0);");
}

#[test]
fn checked_calculations_stay_inside_their_grammar() {
    let styles = Styles::new()
        .with(WIDTH.declare(pct(100) - px(20)))
        .with(PADDING.declare(px(8) - px(16)));

    assert_that!(styles.to_style_string())
        .is_equal_to("width:calc(100% - 20px);padding:calc(-8px);");
}

#[test]
fn checked_custom_properties_and_references_retain_their_grammar() {
    let missing = CustomProperty::<Color>::new("--MissingColor");
    let styles = Styles::new()
        .with(ACCENT_COLOR.declare(NamedColor::Fuchsia))
        .with(COLOR.declare(var_or(&ACCENT_COLOR, NamedColor::Black)))
        .with(BACKGROUND_COLOR.declare(var_or(&missing, NamedColor::CurrentColor)))
        .with(SPACE.declare(rem(1)))
        .with(GAP.declare(var(&SPACE)));

    assert_that!(styles.to_style_string()).is_equal_to(
        "--AccentColor:fuchsia;color:var(--AccentColor, black);\
         background-color:var(--MissingColor, currentcolor);--space:1rem;gap:var(--space);",
    );
}

#[test]
fn optional_checked_values_render_only_when_present() {
    let some = Styles::new().with(Some(TOP.declare(px(8))));
    assert_that!(some.to_style_string()).is_equal_to("top:8px;");

    let none = Styles::new().with(None::<Declaration>);
    assert_that!(none.to_style_string()).is_equal_to("");
}

#[test]
fn signal_carrying_exact_property_value_updates_reactively() {
    let owner = Owner::new();
    owner.with(|| {
        let (color, set_color) = signal(Some(NamedColor::Red));
        let styles = Styles::new().with(move || color.get().map(|value| COLOR.declare(value)));

        assert_that!(styles.to_style_string()).is_equal_to("color:red;");

        set_color.set(Some(NamedColor::Blue));
        assert_that!(styles.to_style_string()).is_equal_to("color:blue;");

        set_color.set(None);
        assert_that!(styles.to_style_string()).is_equal_to("");
    });
}

#[test]
fn always_present_reactive_declarations_use_direct_closures() {
    let owner = Owner::new();
    owner.with(|| {
        let (color, set_color) = signal(rgb(255, 0, 0));
        let styles = Styles::new()
            .with(move || COLOR.declare(color.get()))
            .with(move || BACKGROUND_COLOR.declare(color.get()));

        assert_that!(styles.to_style_string())
            .is_equal_to("color:rgb(255 0 0);background-color:rgb(255 0 0);");

        set_color.set(rgb(0, 0, 255));
        assert_that!(styles.to_style_string())
            .is_equal_to("color:rgb(0 0 255);background-color:rgb(0 0 255);");
    });
}

#[test]
fn heterogeneous_checked_and_unchecked_declarations_can_be_stored_together() {
    let declarations: Vec<StyleEntry> = vec![
        WIDTH.declare(pct(100)).into(),
        DISPLAY.grid().into(),
        UncheckedDeclaration::new("grid-template-areas", "\"a b\"").into(),
    ];
    let styles = Styles::new().with_all(declarations);

    assert_that!(styles.to_style_string())
        .is_equal_to("width:100%;display:grid;grid-template-areas:\"a b\";");
}

#[test]
fn reactive_complete_declaration_can_change_its_property() {
    let use_background = Arc::new(AtomicBool::new(false));
    let evaluations = Arc::new(AtomicUsize::new(0));
    let use_background_for_style = Arc::clone(&use_background);
    let evaluations_for_style = Arc::clone(&evaluations);
    let styles = Styles::new().with(move || {
        evaluations_for_style.fetch_add(1, Ordering::Relaxed);
        let target = if use_background_for_style.load(Ordering::Relaxed) {
            BACKGROUND_COLOR
        } else {
            COLOR
        };
        target.declare(NamedColor::Red)
    });

    assert_that!(styles.to_style_string()).is_equal_to("color:red;");
    assert_that!(evaluations.load(Ordering::Relaxed)).is_equal_to(1);

    use_background.store(true, Ordering::Relaxed);
    assert_that!(styles.to_style_string()).is_equal_to("background-color:red;");
    assert_that!(evaluations.load(Ordering::Relaxed)).is_equal_to(2);
}

#[test]
fn fallback_priority_is_resolved_per_current_property() {
    let higher = Styles::new().with(COLOR.declare(NamedColor::Red));
    let lower = Styles::new()
        .with(UncheckedDeclaration::new("color", "blue"))
        .with(PADDING.declare(px(8)));

    assert_that!(higher.with_fallback(lower).to_style_string())
        .is_equal_to("padding:8px;color:red;");
}

#[test]
fn css_wide_keywords_use_explicit_checked_paths() {
    let styles = Styles::new()
        .with(PADDING.inherit())
        .with(ALL.revert_layer());

    assert_that!(styles.to_style_string()).is_equal_to("padding:inherit;all:revert-layer;");
}

#[test]
fn parsed_and_unchecked_declarations_remain_available_as_escape_hatches() {
    let parsed = Styles::parse("display: grid; color: blue").expect("CSS should parse");
    let extended = parsed.with(UncheckedDeclaration::new("font-family", "system-ui"));

    assert_that!(extended.to_style_string())
        .is_equal_to("display:grid;color:blue;font-family:system-ui;");
}

#[test]
fn a_fallback_shorthand_never_overrides_a_longhand_of_a_higher_layer() {
    let defaults = Styles::new().with(PADDING.declare(px(16)));
    let styles = Styles::new()
        .with(css::property::PADDING_TOP.declare(px(0)))
        .with_fallback(defaults);

    assert_that!(styles.to_style_string()).is_equal_to("padding:16px;padding-top:0px;");
}

#[test]
fn a_nested_styles_keeps_its_fallbacks_above_the_fallbacks_of_the_receiver() {
    let variant = Styles::new()
        .with(COLOR.declare(NamedColor::Red))
        .with_fallback(PADDING.declare(px(4)));
    let styles = Styles::new()
        .with(DISPLAY.grid())
        .with(variant)
        .with_fallback(PADDING.declare(px(16)));

    assert_that!(styles.to_style_string()).is_equal_to("padding:4px;display:grid;color:red;");
}

#[test]
fn a_closure_can_switch_an_optional_group_of_declarations() {
    let owner = Owner::new();
    owner.with(|| {
        let (compact, set_compact) = signal(true);
        let styles = Styles::new().with(DISPLAY.grid()).with(move || {
            compact.get().then(|| {
                Styles::new()
                    .with(GAP.declare(px(2)))
                    .with(PADDING.declare(px(4)))
            })
        });

        assert_that!(styles.to_style_string()).is_equal_to("display:grid;gap:2px;padding:4px;");
        set_compact.set(false);
        assert_that!(styles.to_style_string()).is_equal_to("display:grid;");
    });
}
