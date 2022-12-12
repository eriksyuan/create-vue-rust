use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  pub project_name: Option<String>,

  #[arg(long)]
  pub default: bool,

  #[arg(alias = "typescript", long)]
  pub ts: bool,

  #[arg(long)]
  pub jsx: bool,

  #[arg(alias = "vue-router", long)]
  pub router: bool,

  #[arg(long)]
  pub pinia: bool,

  #[arg(alias = "with-tests", long)]
  pub tests: bool,

  #[arg(long)]
  pub vitest: bool,

  #[arg(long)]
  pub cypress: bool,

  #[arg(long)]
  pub playwright: bool,

  #[arg(long)]
  pub eslint: bool,
  ///
  #[arg(alias = "eslint-with-prettier", long)]
  pub eslint_with_prettier: bool,

  #[arg(long)]
  pub force: bool,
}
