use clap::Args;
use eyre::Result;

use crate::shell::completions;

/// Generate shell completions
#[derive(Debug, Args)]
#[clap(hide = true, verbatim_doc_comment)]
pub struct RenderCompletion {}

impl RenderCompletion {
    pub fn run(self) -> Result<()> {
        let cmd = crate::cli::Cli::command();

        let script = completions::zsh_complete(&cmd)?;
        rtxprintln!("{}", script.trim());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_completion() {
        assert_cli!("render-completion", "zsh");
    }
}
