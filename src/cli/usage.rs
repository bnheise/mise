use crate::{cli, env};
use eyre::Result;
use std::iter::once;

/// Run usage-cli
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment)]
pub struct Usage {
    #[clap(trailing_var_arg = true)]
    args: Vec<String>,
}

impl Usage {
    pub fn run(self) -> Result<()> {
        env::set_var("USAGE_CMD", "@rtx usage --");
        let args = once("usage".to_string())
            .chain(self.args)
            .collect::<Vec<_>>();
        if let Err(err) = usage_cli::run(&args) {
            panic!("{err:?}");
        }
        Ok(())
    }
}

pub fn display_usage(args: &[String]) {
    if let "--USAGE" = args.get(1).map(|s| s.as_str()).unwrap_or_default() {
        let spec: usage::Spec = cli::Cli::command().into();
        println!("{}", spec);
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_completion() {
        assert_cli_snapshot!("--USAGE");
    }
}
