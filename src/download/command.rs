use crate::Execute;

use anyhow::Result;

#[derive(Clone, Debug, clap::Parser)]
#[clap(about = "Download a chapter")]
#[command(arg_required_else_help(true))]
pub struct Download { }

#[async_trait::async_trait]
impl Execute for Download {
    fn pre_check(&self) -> Result<()> {
        Ok(())
    }

    async fn execute(self) -> Result<()> {
        Ok(())
    }
}
