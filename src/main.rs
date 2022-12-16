mod args;
mod prompts;
mod render;
mod utils;
mod banner;
mod editor_config;
mod create_eslint_config;
mod render_eslint;


use args::Args;
use clap::Parser;
use serde_json::json;
use std::{env, fs};

use crate::{
  prompts::{E2eTesting, Prompts},
  render::render_template,
  utils::{empty_dir, get_path_from_cwd, mkdir}, banner::print_banner,
};

fn main() {
  print_banner();

  let args = Args::parse();

  let prompts = Prompts::new(&args);

  let Prompts {
    package_name,
    project_name,
    needs_e2e_testing,
    needs_eslint,
    needs_jsx,
    needs_pinia,
    needs_prettier,
    needs_router,
    needs_type_script,
    needs_vitest,
    should_overwrite,
  } = prompts;

  let needs_cypress = match needs_e2e_testing {
    E2eTesting::Cypress => true,
    _ => false,
  };

  let needs_cypress_ct = needs_cypress && !needs_vitest;

  let needs_play_wright = match needs_e2e_testing {
    E2eTesting::Playwright => true,
    _ => false,
  };

  if should_overwrite {
    empty_dir(&project_name).unwrap();
  } else {
    mkdir(&project_name).unwrap();
  }

  let project_root = get_path_from_cwd(&project_name).unwrap();

  println!("\nScaffolding project in: {:?}", &project_root);

  let pkg = json!({
    "name":package_name.clone(),
    "version":"0.0.0",
  });

  let pkg_str = serde_json::to_string_pretty(&pkg).unwrap();

  fs::write(&project_root.join("package.json"), &pkg_str).unwrap();

  render(vec!["base"], &project_name);

  if needs_jsx {
    render(vec!["config", "jsx"], &project_name);
  }

  if needs_router {
    render(vec!["config", "router"], &project_name);
  }

  if needs_pinia {
    render(vec!["config", "pinia"], &project_name);
  }

  if needs_vitest {
    render(vec!["config", "vitest"], &project_name);
  }

  if needs_cypress {
    render(vec!["config, cypress"], &project_name);
  }

  if needs_cypress_ct {
    render(vec!["config", "cypress-ct"], &project_name);
  }

  if needs_play_wright {
    render(vec!["config", "playwright"], &project_name);
  }

  // render typescript configs
  if needs_type_script {
    render(vec!["config", "typescript"], &project_name);
    render(vec!["tsconfig", "base"], &project_name);
    if needs_cypress {
      render(vec!["tsconfig", "cypress"], &project_name);
    }

    if needs_cypress_ct {
      render(vec!["tsconfig", "cypress-ct"], &project_name);
    }

    if needs_play_wright {
      render(vec!["tsconfig", "playwright"], &project_name);
    }

    if needs_vitest {
      render(vec!["tsconfig", "vitest"], &project_name);
    }
  }

  println!("Hello, world!,{:?}", args);
}

fn render(names: Vec<&str>, project_name: &String) -> () {
  let mut template_path = env::current_dir().unwrap();
  template_path.push("src");
  template_path.push("template");

  for name in names {
    template_path.push(name);
  }

  let dest = get_path_from_cwd(project_name).unwrap();

  render_template(&template_path, &dest).unwrap()
}
