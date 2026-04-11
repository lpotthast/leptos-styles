use thirtyfour::WebDriver;

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
    pub async fn goto(&self) -> anyhow::Result<()> {
        self.goto_path("/").await
    }

    pub async fn click(&self, id: &str) -> anyhow::Result<()> {
        self.click_element_with_id(id).await
    }
}
