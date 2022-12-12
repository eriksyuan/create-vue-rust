use std::fmt::Display;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::{
  args::Args,
  utils::{can_skip_emptying, is_valid_package_name, to_valid_package_name},
};
use anyhow::Result;
use console::Term;

#[derive(Debug)]
pub struct Prompts {
  pub project_name: String,
  pub should_overwrite: bool,
  pub package_name: String,
  pub needs_type_script: bool,
  pub needs_jsx: bool,
  pub needs_router: bool,
  pub needs_pinia: bool,
  pub needs_vitest: bool,
  pub needs_e2e_testing: E2eTesting,
  pub needs_eslint: bool,
  pub needs_prettier: bool,
}

#[derive(Debug, Clone)]
pub enum E2eTesting {
  None,
  Cypress,
  Playwright,
}

impl Display for E2eTesting {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      E2eTesting::None => write!(f, "No"),
      E2eTesting::Cypress => write!(f, "Cypress"),
      E2eTesting::Playwright => write!(f, "Playwright"),
    }
  }
}

impl Prompts {
  pub fn new(args: &Args) -> Self {
    let project_name = match args.project_name.clone() {
      Some(name) => name,
      None => get_string_from_user("Project name", "my-vue-app").unwrap(),
    };

    let is_feature_flags_used = &args.default
      | &args.ts
      | &args.jsx
      | &args.router
      | &args.pinia
      | &args.tests
      | &args.vitest
      | &args.cypress
      | &args.playwright
      | &args.eslint;

    let show_should_overwrite = can_skip_emptying(&project_name).unwrap() | &args.force;

    let should_overwrite = if show_should_overwrite {
      false
    } else {
      let x = get_should_overwrite(&project_name).unwrap_or(false);
      if !x {
        panic!("Operation cancelled")
      }
      x
    };

    let package_name = match is_valid_package_name(&project_name) {
      Ok(true) => project_name.clone(),
      Ok(false) => {
        let name = get_string_from_user(
          "Package name:",
          to_valid_package_name(&project_name).unwrap().as_str(),
        )
        .unwrap();
        match is_valid_package_name(&name) {
          Ok(true) => name,
          _ => panic!("Invalid package name"),
        }
      }
      _ => panic!(" Operation cancelled"),
    };

    let needs_type_script = if is_feature_flags_used {
      args.ts
    } else {
      get_bool_from_user("Add TypeScript Support?", false).unwrap()
    };

    let needs_jsx = if is_feature_flags_used {
      args.jsx
    } else {
      get_bool_from_user("Add JSX Support?", false).unwrap()
    };

    let needs_router = if is_feature_flags_used {
      args.router
    } else {
      get_bool_from_user(
        "Add Vue Router for Single Page Application development?",
        false,
      )
      .unwrap()
    };

    let needs_pinia = if is_feature_flags_used {
      args.pinia
    } else {
      get_bool_from_user("Add Pinia for state management?", false).unwrap()
    };

    let needs_vitest = if is_feature_flags_used {
      args.vitest
    } else {
      get_bool_from_user("Add Vitest for Unit Testing?", false).unwrap()
    };

    let needs_e2e_testing = if is_feature_flags_used {
      E2eTesting::None
    } else {
      get_e2e_testing()
    };

    let needs_eslint = if is_feature_flags_used {
      args.eslint
    } else {
      get_bool_from_user("Add ESLint for code quality?", false).unwrap()
    };

    let needs_prettier = if is_feature_flags_used | !needs_eslint {
      args.eslint_with_prettier
    } else {
      get_bool_from_user("Add Prettier for code formatting?", false).unwrap()
    };

    Self {
      project_name,
      should_overwrite,
      package_name,
      needs_type_script,
      needs_jsx,
      needs_router,
      needs_pinia,
      needs_vitest,
      needs_e2e_testing,
      needs_eslint,
      needs_prettier,
    }
  }
}

fn get_string_from_user(prompt: &str, default: &str) -> Result<String> {
  let default_theme = &ColorfulTheme::default();
  let name = Input::<String>::with_theme(default_theme)
    .default(default.to_string())
    .with_prompt(prompt)
    .interact()
    .unwrap();
  Ok(name)
}

fn get_should_overwrite(package_name: &String) -> Result<bool> {
  let default_theme = &ColorfulTheme::default();
  let target_dir = if package_name == "." {
    "current directory".to_string()
  } else {
    format!("directory {}", &package_name)
  };

  let notice_str = format!(
    "{},is not empty. Remove existing files and continue?",
    target_dir
  );
  let should_overwrite = Confirm::with_theme(default_theme)
    .default(false)
    .with_prompt(notice_str)
    .interact()
    .unwrap();
  Ok(should_overwrite)
}

fn get_bool_from_user(prompt: &str, default: bool) -> Result<bool> {
  let default_theme = &ColorfulTheme::default();
  let should_overwrite = Confirm::with_theme(default_theme)
    .default(default)
    .with_prompt(prompt)
    .interact()
    .unwrap();
  Ok(should_overwrite)
}

fn get_e2e_testing() -> E2eTesting {
  let items = vec![
    E2eTesting::None,
    E2eTesting::Cypress,
    E2eTesting::Playwright,
  ];

  let e2e_testing = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Add an End-to-End Testing Solution?")
    .items(&items)
    .default(0)
    .interact_on_opt(&Term::stderr())
    .unwrap();

  match e2e_testing {
    Some(index) => items.get(index).unwrap().clone(),
    None => E2eTesting::None,
  }
}
