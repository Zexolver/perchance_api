use playwright::api::{Browser, BrowserContext, Playwright};
use crate::utils::generate_user_agent;
use crate::error::Result;

pub struct Generator {
    playwright: Option<Playwright>,
    browser: Option<Browser>,
    pub context: Option<BrowserContext>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            playwright: None,
            browser: None,
            context: None,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        if self.playwright.is_none() {
            let pw = Playwright::initialize().await?;
            self.playwright = Some(pw);
        }

        if self.browser.is_none() {
            let pw = self.playwright.as_ref().unwrap();
            let browser = pw.chromium().launcher().headless(true).launch().await?;
            self.browser = Some(browser);
        }

        if self.context.is_none() {
            let browser = self.browser.as_ref().unwrap();
            let context = browser
                .context_builder()
                .user_agent(&generate_user_agent())
                .build()
                .await?;
            self.context = Some(context);
        }
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        if let Some(ctx) = self.context.take() {
            ctx.close().await?;
        }
        if let Some(browser) = self.browser.take() {
            browser.close().await?;
        }
        Ok(())
    }
}
