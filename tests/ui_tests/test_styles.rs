use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use leptos_browser_test::{Report, bail};

use crate::pages::{BaseActions, styles::StylesPage};

pub struct StylesTests {}

#[async_trait]
impl BrowserTest<str> for StylesTests {
    fn name(&self) -> Cow<'_, str> {
        "styles_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = StylesPage { driver, base_url };
        page.goto().await?;

        assert_static_target(&page).await?;
        assert_hydrate_target(&page).await?;
        assert_reactive_target(&page).await?;
        assert_normalized_target(&page).await?;
        assert_fallback_target(&page).await?;
        assert_typed_value_target(&page).await?;
        assert_checked_declaration_target(&page).await?;
        assert_drilling_target(&page).await?;
        assert_transition_target(&page).await?;
        assert_managed_target(&page).await?;
        assert_empty_target(&page).await?;
        assert_var_target(&page).await?;
        assert_calc_target(&page).await?;
        assert_units_target(&page).await?;
        assert_parse_target(&page).await?;
        assert_layout_target(&page).await?;
        assert_coexist_target(&page).await?;
        assert_lifecycle_target(&page).await?;
        assert_quoted_target(&page).await?;
        assert_overlap_target(&page).await?;
        assert_update_target(&page).await?;
        assert_shorthand_target(&page).await?;
        assert_single_target(&page).await?;

        Ok(())
    }
}

async fn assert_static_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("static-target", "display: grid; color: red;")
        .await?;

    // Hydration takes over the server's attribute without writing to the element.
    let attribute = page.style_attribute("static-target").await?;
    if attribute.as_deref() != Some("display:grid;color:red;") {
        bail!("hydration rewrote #static-target, its style attribute is {attribute:?}");
    }
    Ok(())
}

async fn assert_hydrate_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("hydrate-target", "display: grid; color: green;")
        .await
}

async fn assert_reactive_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("reactive-target", "display: grid; color: tomato;")
        .await?;

    page.click("set-royalblue").await?;
    page.wait_for_style("reactive-target", "display: grid; color: royalblue;")
        .await?;

    page.click("clear-color").await?;
    page.wait_for_style("reactive-target", "display: grid;")
        .await
}

async fn assert_normalized_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "normalized-target",
        "touch-action: none; --ThemeAccent: tomato;",
    )
    .await
}

async fn assert_fallback_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("fallback-target", "padding: 0px 16px 16px; color: orange;")
        .await?;

    page.click("clear-fallback-color").await?;
    page.wait_for_style("fallback-target", "color: gray; padding: 0px 16px 16px;")
        .await?;

    page.click("set-fallback-orange").await?;
    page.wait_for_style("fallback-target", "padding: 0px 16px 16px; color: orange;")
        .await
}

async fn assert_typed_value_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("typed-value-target", "width: 120px; color: red;")
        .await?;

    page.click("set-typed-blue").await?;
    page.wait_for_style("typed-value-target", "width: 120px; color: blue;")
        .await?;

    page.click("set-typed-transparent").await?;
    page.wait_for_style("typed-value-target", "width: 120px; color: transparent;")
        .await
}

async fn assert_checked_declaration_target(page: &StylesPage<'_>) -> Result<(), Report> {
    // The initial declaration is server-rendered and then hydrated in place.
    page.wait_for_style("checked-declaration-target", "color: rgb(255, 0, 0);")
        .await?;

    page.click("checked-value-update").await?;
    page.wait_for_style("checked-declaration-target", "color: rgb(0, 0, 255);")
        .await?;

    page.click("checked-property-update").await?;
    page.wait_for_style(
        "checked-declaration-target",
        "background-color: rgb(0, 128, 0);",
    )
    .await?;

    page.click("checked-custom-property").await?;
    page.wait_for_style(
        "checked-declaration-target",
        "--DirectAccent: rgb(255 0 255);",
    )
    .await?;

    page.click("checked-reset").await?;
    page.wait_for_style("checked-declaration-target", "").await
}

async fn assert_drilling_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "drilling-target",
        "color: purple; padding: 5px; margin: 10px;",
    )
    .await
}

async fn assert_transition_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("transition-target", "display: grid; color: seagreen;")
        .await?;

    page.click("transition-clear").await?;
    page.wait_for_style("transition-target", "display: grid;")
        .await?;

    page.click("transition-set-color").await?;
    page.wait_for_style("transition-target", "display: grid; color: seagreen;")
        .await?;

    page.click("transition-freeze-static").await?;
    page.wait_for_style("transition-target", "display: grid; color: olive;")
        .await?;

    page.click("transition-enable-reactive").await?;
    page.wait_for_style("transition-target", "display: grid; color: seagreen;")
        .await?;

    page.click("transition-clear").await?;
    page.wait_for_style("transition-target", "display: grid;")
        .await
}

async fn assert_managed_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("managed-target", "display: grid; color: indigo;")
        .await?;

    page.click("managed-mutate").await?;
    page.wait_for_style("managed-target", "color: black; background-color: gold;")
        .await?;

    // Setting the same value again changes no declaration, so nothing is written.
    page.click("managed-set-same-color").await?;
    page.wait_for_style("managed-target", "color: black; background-color: gold;")
        .await?;

    // The unchanged `display` shows no value, because the external code removed it, so the
    // update writes every declaration again. The externally written `background-color` is not
    // managed and stays.
    page.click("managed-set-crimson").await?;
    page.wait_for_style(
        "managed-target",
        "background-color: gold; display: grid; color: crimson;",
    )
    .await?;

    // Removing the color only removes `color`, because the unchanged `display` shows its value.
    page.click("managed-clear").await?;
    page.wait_for_style("managed-target", "background-color: gold; display: grid;")
        .await
}

async fn assert_empty_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("empty-target", "color: teal;").await?;

    page.click("empty-clear").await?;
    page.wait_for_no_style_attribute("empty-target").await?;

    page.click("empty-restore").await?;
    page.wait_for_style("empty-target", "color: teal;").await
}

async fn assert_var_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "var-target",
        "--AccentColor: fuchsia; color: var(--AccentColor); background-color: var(--MissingColor, currentcolor);",
    )
    .await
}

async fn assert_calc_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "calc-target",
        "width: calc(100% - 20px); height: min(1rem + 50vh, 400px); padding: calc(-4px);",
    )
    .await
}

async fn assert_units_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "units-target",
        "width: 50%; height: 40vh; padding: 1.5em; margin: 2rem auto; right: 10vw;",
    )
    .await
}

async fn assert_parse_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("parse-target", "color: rebeccapurple; padding: 8px;")
        .await
}

async fn assert_layout_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "layout-target",
        "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 1rem; justify-content: center; transform: rotate(3deg);",
    )
    .await
}

async fn assert_coexist_target(page: &StylesPage<'_>) -> Result<(), Report> {
    // Hydration finds the server's declarations in the attribute and leaves it as it is.
    page.wait_for_style("coexist-target", "color: navy; opacity: 0.5;")
        .await?;

    page.click("coexist-set-maroon").await?;
    page.wait_for_style("coexist-target", "opacity: 0.5; color: maroon;")
        .await
}

async fn assert_lifecycle_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("lifecycle-target", "display: grid; color: teal;")
        .await?;

    page.click("lifecycle-switch-property").await?;
    page.wait_for_style("lifecycle-target", "display: grid; background-color: teal;")
        .await?;

    page.click("lifecycle-hide").await?;
    page.wait_for_no_style_attribute("lifecycle-target").await?;

    page.click("lifecycle-show").await?;
    page.wait_for_style("lifecycle-target", "display: grid; background-color: teal;")
        .await
}

async fn assert_quoted_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style(
        "quoted-target",
        "font-family: \"Open Sans\", sans-serif; background-image: url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg'/>\");",
    )
    .await
}

async fn assert_overlap_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("overlap-target", "padding: 1px 8px 8px; color: navy;")
        .await?;

    page.wait_for_style(
        "overlap-unknown-target",
        "padding: 1px 8px 8px; color: navy;",
    )
    .await?;
    page.click("overlap-set-maroon").await?;
    page.wait_for_style("overlap-target", "padding: 1px 8px 8px; color: maroon;")
        .await?;
    page.wait_for_style(
        "overlap-unknown-target",
        "padding: 1px 8px 8px; color: maroon;",
    )
    .await?;

    page.wait_for_style(
        "overlap-optional-target",
        "padding: 1px 8px 8px; color: navy;",
    )
    .await?;
    page.click("overlap-remove-color").await?;
    page.wait_for_style("overlap-optional-target", "padding: 1px 8px 8px;")
        .await
}

async fn assert_update_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("update-repeated-target", "height: 100px; color: navy;")
        .await?;
    page.click("update-toggle-enhanced").await?;
    page.wait_for_style("update-repeated-target", "height: 50px; color: navy;")
        .await?;
    page.click("update-toggle-enhanced").await?;
    page.wait_for_style("update-repeated-target", "height: 100px; color: navy;")
        .await?;

    page.wait_for_style(
        "update-reordered-target",
        "display: grid; color: navy; gap: 4px;",
    )
    .await?;
    page.click("update-swap").await?;
    page.wait_for_style(
        "update-reordered-target",
        "display: grid; gap: 4px; color: navy;",
    )
    .await?;

    page.wait_for_style("update-rejected-target", "padding: 1px 8px 8px;")
        .await?;
    page.click("update-reject").await?;
    page.wait_for_style("update-rejected-target", "padding: 8px;")
        .await?;

    page.wait_for_style(
        "update-group-target",
        "display: grid; gap: 2px; padding: 4px;",
    )
    .await?;
    page.click("update-expand").await?;
    page.wait_for_style("update-group-target", "display: grid; gap: 8px;")
        .await
}

async fn assert_shorthand_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.click("shorthand-remove-accent").await?;
    page.click("shorthand-widen").await?;
    for name in ["border", "padding", "group"] {
        let reference = page
            .style_of(&format!("shorthand-{name}-reference"))
            .await?;
        page.wait_for_style(&format!("shorthand-{name}-target"), &reference)
            .await?;
    }
    Ok(())
}

async fn assert_single_target(page: &StylesPage<'_>) -> Result<(), Report> {
    page.wait_for_style("single-unchecked-target", "contain: layout;")
        .await?;

    page.wait_for_style("single-entry-target", "width: 100px;")
        .await?;
    page.click("single-widen").await?;
    page.wait_for_style("single-entry-target", "width: 200px;")
        .await
}
