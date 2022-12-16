use std::{fmt::Display, path::PathBuf};

use anyhow::Result;
use serde_json::{json, Value};

use crate::{
  editor_config::{create_config, Editorconfilgs},
  render::merge,
};

#[derive(Debug, Clone)]
pub enum StyleGuide {
  Default,
  Airbnb,
  Standard,
}

impl Display for StyleGuide {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StyleGuide::Default => write!(f, "default"),
      StyleGuide::Airbnb => write!(f, "airbnb"),
      StyleGuide::Standard => write!(f, "standard"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ConfigFiles {
  editorconfig: ConfigFile,
  eslintrc: ConfigFile,
  prettierrc: ConfigFile,
}

impl ConfigFiles {
  fn new() -> Self {
    Self {
      editorconfig: ConfigFile::new(".editorconfig".to_string()),
      eslintrc: ConfigFile::new(".eslintrc".to_string()),
      prettierrc: ConfigFile::new(".prettierrc".to_string()),
    }
  }
  pub fn write_to_disk(&self, root: &PathBuf) -> Result<()> {
    self.editorconfig.write_to_disk(&root)?;
    self.prettierrc.write_to_disk(&root)?;
    self.eslintrc.write_to_disk(&root)?;
    Ok(())
  }
}

impl Display for ConfigFiles {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\n{}\n{}\n",
      self.editorconfig, self.eslintrc, self.prettierrc
    )
  }
}

#[derive(Debug, Clone)]
pub struct ConfigFile(String, Option<String>);

impl ConfigFile {
  fn new(name: String) -> Self {
    Self(name, None)
  }
  fn push_string(&mut self, value: String) {
    if let Some(v) = &mut self.1 {
      v.push_str(&value);
    } else {
      self.1 = Some(value);
    }
  }
  pub fn write_to_disk(&self, root: &PathBuf) -> Result<()> {
    match &self.1 {
      Some(contents) => {
        let filename = self.0.clone();
        let path = root.join(filename);
        std::fs::write(path, contents)?;
      }
      _ => {}
    }
    Ok(())
  }
}

impl Display for ConfigFile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{:?}", self.0, self.1)
  }
}

#[derive(Debug, Clone)]
pub struct EslintConfig(Value);

impl EslintConfig {
  fn new() -> Self {
    let value = json!(
      {
        "root": true,
        "extends": ["plugin:vue/essential"]
      }
    );
    Self(value)
  }

  fn add_extend(&mut self, plugin: &str) {
    let plugins = self.0.get_mut("extends").unwrap();
    let plugins = plugins.as_array_mut().unwrap();
    plugins.push(json!(plugin));
  }

  fn get_value(&self) -> &Value {
    &self.0
  }

  fn merge(&mut self, other: &Value) {
    self.0 = merge(&self.0, other).unwrap();
  }

  fn set_val(&mut self, key: String, val: Value) {
    let map = self.0.as_object_mut().unwrap();
    map.insert(key, val);
  }
}

impl Display for EslintConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let content = serde_json::to_string_pretty(&self.0).unwrap();
    write!(f, "{}", content)
  }
}

#[derive(Debug, Clone)]
pub struct PkgConfig(Value);
impl PkgConfig {
  fn new() -> Self {
    let value = json!({"devDependencies":{}});
    Self(value)
  }

  fn add_dependency(&mut self, name: &str) {
    let map = self.0.get_mut("devDependencies").unwrap();
    let map = map.as_object_mut().unwrap();
    let binding = get_version_map();
    let version = binding.get(name).unwrap();
    map.insert(name.to_string(), version.clone());
  }

  fn merge(&mut self, other: &Value) {
    self.0 = merge(&self.0, other).unwrap();
  }

  fn get(&self, key: &str) -> Option<&Value> {
    let map = self.0.as_object().unwrap();
    map.get(key)
  }
  fn has_dependency(&self, name: &str) -> bool {
    let map = self.get("devDependencies").unwrap().as_object().unwrap();
    map.contains_key(name)
  }
}

impl Display for PkgConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let content = serde_json::to_string_pretty(&self.0).unwrap();
    write!(f, "{}", content)
  }
}

pub struct CreateConfig {
  pub style_guide: StyleGuide,
  pub has_typescript: bool,
  pub needs_prettier: bool,
  pub additional_config: Option<Value>,
  pub additional_dependencies: Option<Value>,
}

const CREATE_ALIAS_SETTING_PLACEHOLDER: &str = "CREATE_ALIAS_SETTING_PLACEHOLDER";

impl CreateConfig {
  pub fn create(&self) -> Result<(PkgConfig, ConfigFiles)> {
    let style_guide = self.style_guide.clone();
    let has_typescript = self.has_typescript;
    let needs_prettier = self.needs_prettier;
    let additional_config = self.additional_config.clone();
    let additional_dependencies = self.additional_dependencies.clone();

    let mut pkg = PkgConfig::new();

    pkg.add_dependency("eslint");
    pkg.add_dependency("eslint-plugin-vue");

    match &style_guide {
      StyleGuide::Default => {}
      _ => {
        if has_typescript || needs_prettier {
          pkg.add_dependency("@rushstack/eslint-patch");
        }
      }
    }

    let language = if has_typescript {
      "typescript"
    } else {
      "javascript"
    };

    let mut eslint_config = EslintConfig::new();

    match (&style_guide, language) {
      (StyleGuide::Default, "typescript") => {
        eslint_config.add_extend("eslint:recommended");
        eslint_config.add_extend("@vue/eslint-config-typescript");
        pkg.add_dependency("@vue/eslint-config-typescript");
      }
      (StyleGuide::Default, "javascript") => {
        eslint_config.add_extend("eslint:recommended");
      }
      (StyleGuide::Airbnb, "javascript") => {
        eslint_config.add_extend("@vue/eslint-config-airbnb");
        pkg.add_dependency("@vue/eslint-config-airbnb");
      }
      (StyleGuide::Standard, "javascript") => {
        eslint_config.add_extend("@vue/eslint-config-standard");
        pkg.add_dependency("@vue/eslint-config-standard");
      }
      (StyleGuide::Airbnb, "typescript") => {
        eslint_config.add_extend("@vue/eslint-config-airbnb-with-typescript")
      }
      (StyleGuide::Standard, "typescript") => {
        eslint_config.add_extend("@vue/eslint-config-standard-with-typescript")
      }
      _ => {}
    }

    if needs_prettier {
      pkg.add_dependency("prettier");
      pkg.add_dependency("@vue/eslint-config-prettier");
      eslint_config.add_extend("@vue/eslint-config-prettier");
    }

    // merge additional_dependencies
    match additional_dependencies {
      Some(dep) => pkg.merge(&dep),
      _ => {}
    }

    match additional_config {
      Some(config) => eslint_config.merge(&config),
      _ => {}
    };

    let mut files = ConfigFiles::new();

    let editor_configs = create_config();

    match &style_guide {
      StyleGuide::Default => {
        files
          .eslintrc
          .push_string("/* eslint-env node */\n".to_string());

        eslint_config.set_val(
          "parserOptions".to_string(),
          json!({
            "ecmaVersion": "latest"
          }),
        );
      }
      _ => {
        let editor_config = Editorconfilgs::new();
        let config = editor_config.get(&style_guide);

        files.editorconfig.push_string(config);
      }
    }

    if pkg.has_dependency("@rushstack/eslint-patch") {
      files.eslintrc.push_string(
        r#"require('@rushstack/eslint-patch/modern-module-resolution')\n\n"#.to_string(),
      );
    }

    files.eslintrc.push_string(format!(
      "module.exports = {}\n",
      stringify_js(&eslint_config, &style_guide)
    ));

    if needs_prettier {
      let prettier_config = Editorconfilgs::new();

      files
        .prettierrc
        .push_string(prettier_config.get(&style_guide));
    }

    Ok((pkg, files))
  }
}

fn stringify_js(eslint_config: &EslintConfig, style_guide: &StyleGuide) -> String {
  let result = serde_json::to_string_pretty(&eslint_config.0).unwrap();

  let replacer = format!(
    r#"...require('@vue/eslint-config-{}/createAliasSetting')"#,
    style_guide
  );

  result.replace(CREATE_ALIAS_SETTING_PLACEHOLDER, &replacer)
}

fn get_version_map() -> Value {
  json!({
    "@rushstack/eslint-patch": "^1.1.4",
    "@vue/eslint-config-airbnb": "^7.0.0",
    "@vue/eslint-config-airbnb-with-typescript": "^7.0.0",
    "@vue/eslint-config-prettier": "^7.0.0",
    "@vue/eslint-config-standard": "^8.0.1",
    "@vue/eslint-config-standard-with-typescript": "^8.0.0",
    "@vue/eslint-config-typescript": "^11.0.0",
    "eslint": "^8.22.0",
    "eslint-plugin-vue": "^9.3.0",
    "prettier": "^2.7.1",
    "standard": "^17.0.0",
    "typescript": "~4.7.4"
  })
}

#[test]
fn main_test() {
  let config = CreateConfig {
    style_guide: StyleGuide::Airbnb,
    has_typescript: true,
    needs_prettier: true,
    additional_config: None,
    additional_dependencies: None,
  };
  let (pkg, files) = config.create().unwrap();
  println!("pkg:\n{}\n  file:\n{}", pkg, files)
}
