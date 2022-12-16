use serde_json::{json, Value};

use crate::create_eslint_config::StyleGuide;

pub fn create_config() -> [Value; 2] {
  let airbnb: Value = json!({
    "$schema": "https://json.schemastore.org/prettierrc",
    "arrowParens": "always",
    "bracketSameLine": false,
    "bracketSpacing": true,
    "endOfLine": "lf",
    "jsxSingleQuote": false,
    "printWidth": 100,
    "proseWrap": "preserve",
    "quoteProps": "as-needed",
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2,
    "trailingComma": "all",
    "useTabs": false
  });

  let standard: Value = json!({
    "$schema": "https://json.schemastore.org/prettierrc",
    "arrowParens": "always",
    "bracketSameLine": false,
    "bracketSpacing": true,
    "endOfLine": "lf",
    "jsxSingleQuote": false,
    "printWidth": 100,
    "proseWrap": "preserve",
    "quoteProps": "as-needed",
    "semi": true,
    "singleQuote": true,
    "tabWidth": 2,
    "trailingComma": "all",
    "useTabs": false
  });
  [airbnb, standard]
}

pub struct Editorconfilgs {
  pub airbnb: String,
  pub standard: String,
}

impl Editorconfilgs {
  pub fn new() -> Self {
    let airbnb = r#"root = true
[*.{js,jsx,mjs,cjs,ts,tsx,mts,cts,vue}]
charset = utf-8
end_of_line = lf
indent_size = 2
indent_style = space
insert_final_newline = true
max_line_length = 100
trim_trailing_whitespace = true
"#
    .to_string();

    let standard = r#"root = true

[*.{js,jsx,mjs,cjs,ts,tsx,mts,cts,vue}]
charset = utf-8
indent_size = 2
indent_style = space
insert_final_newline = true
trim_trailing_whitespace = true"#
      .to_string();

    Self { airbnb, standard }
  }

  pub fn get(&self, style_guide: &StyleGuide) -> String {
    match style_guide {
      StyleGuide::Airbnb => self.airbnb.clone(),
      StyleGuide::Standard => self.standard.clone(),
      _ => String::from(""),
    }
  }
}

pub struct Prettierconfigs {
  pub airbnb: Value,
  pub standard: Value,
}

impl Prettierconfigs {
  pub fn new() -> Self {
    let airbnb: Value = json!({
      "$schema": "https://json.schemastore.org/prettierrc",
      "arrowParens": "always",
      "bracketSameLine": false,
      "bracketSpacing": true,
      "endOfLine": "lf",
      "jsxSingleQuote": false,
      "printWidth": 100,
      "proseWrap": "preserve",
      "quoteProps": "as-needed",
      "semi": true,
      "singleQuote": true,
      "tabWidth": 2,
      "trailingComma": "all",
      "useTabs": false
    });

    let standard: Value = json!({
      "$schema": "https://json.schemastore.org/prettierrc",
      "arrowParens": "always",
      "bracketSameLine": false,
      "bracketSpacing": true,
      "endOfLine": "lf",
      "jsxSingleQuote": false,
      "printWidth": 100,
      "proseWrap": "preserve",
      "quoteProps": "as-needed",
      "semi": true,
      "singleQuote": true,
      "tabWidth": 2,
      "trailingComma": "all",
      "useTabs": false
    });

    Self { airbnb, standard }
  }

  pub fn get (&self, style_guide: &StyleGuide) -> Value {
    match style_guide {
      StyleGuide::Airbnb => self.airbnb.clone(),
      StyleGuide::Standard => self.standard.clone(),
      _ => json!({}),
    }
  }
}
