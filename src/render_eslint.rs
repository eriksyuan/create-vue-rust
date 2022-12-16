use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;

use crate::create_eslint_config::{self, CreateConfig, StyleGuide};

struct RenderEslint {
  root: PathBuf,
  needs_typescript: bool,
  needs_cypress: bool,
  needs_cypress_ct: bool,
  needs_prettier: bool,
}

impl RenderEslint {
  fn render(&self) -> Result<()> {
    let mut additional_config = json!({});
    let mut additional_dependencies = json!({});

    let root = self.root.clone();

    if self.needs_cypress {
      let config = additional_config.as_object_mut().unwrap();
      let overrides_files = if self.needs_cypress_ct {
        vec![
          "**/__tests__/*.{cy,spec}.{js,ts,jsx,tsx}",
          "cypress/e2e/**/*.{cy,spec}.{js,ts,jsx,tsx}",
        ]
      } else {
        vec!["cypress/e2e/**/*.{cy,spec}.{js,ts,jsx,tsx}"]
      };

      let overrides = json!({
        "files":overrides_files,
        "extends": ["plugin:cypress/recommended"]
      });

      config.insert("overrides".to_string(), overrides);
      let dep = additional_dependencies.as_object_mut().unwrap();

      dep.insert("eslint-plugin-cypress".to_string(), json!("^2.12.1"));
    };

    let config = CreateConfig {
      style_guide: StyleGuide::Default,
      has_typescript: self.needs_typescript,
      needs_prettier: self.needs_prettier,
      additional_config: Some(additional_config),
      additional_dependencies: Some(additional_dependencies),
    };

    let (pkg,files) = config.create().unwrap();

    files.write_to_disk(&root)?;

    todo!()
  }
}
