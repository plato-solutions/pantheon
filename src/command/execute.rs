use super::Command;
use crate::{error::RunnerErrorKind, webdriver::Webdriver};

pub struct Execute {
    script: String,
    variable: Option<String>,
}

impl Execute {
    pub fn new(script: String, var: Option<String>) -> Self {
        Self {
            script,
            variable: var,
        }
    }
}

#[async_trait::async_trait]
impl Command for Execute {
    async fn run<D>(&self, runner: &mut crate::runner::Runner<D>) -> Result<(), RunnerErrorKind>
    where
        D: Webdriver,
    {
        let res = runner.exec(&self.script).await?;
        if let Some(var) = self.variable.as_ref() {
            runner.save_value(var.clone(), res);
        }

        Ok(())
    }
}
