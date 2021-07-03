use super::Command;
use crate::{
    error::RunnerErrorKind,
    webdriver::{Element, Locator, Webdriver},
};
use serde_json::Value;

pub struct StoreText {
    target: Locator,
    variable: String,
}

impl StoreText {
    pub fn new(target: Locator, variable: String) -> Self {
        Self { target, variable }
    }
}

#[async_trait::async_trait]
impl Command for StoreText {
    async fn run<D>(&self, runner: &mut crate::runner::Runner<D>) -> Result<(), RunnerErrorKind>
    where
        D: Webdriver,
    {
        let value = runner
            .get_webdriver()
            .find(self.target.clone())
            .await?
            .text()
            .await?;

        let value = Value::String(value);
        runner.save_value(self.variable.clone(), value);

        Ok(())
    }
}
