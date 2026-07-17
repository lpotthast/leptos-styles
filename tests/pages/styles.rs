use browser_test::thirtyfour::WebDriver;
use leptos_browser_test::Report;

use crate::pages::BaseActions;

pub struct StylesPage<'a> {
    pub driver: &'a WebDriver,
    pub base_url: &'a str,
}

impl BaseActions for StylesPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

impl StylesPage<'_> {
    pub async fn goto(&self) -> Result<(), Report> {
        self.goto_path("/").await
    }

    pub async fn click(&self, id: &str) -> Result<(), Report> {
        self.click_element_with_id(id).await
    }
}
