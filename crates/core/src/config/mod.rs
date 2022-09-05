use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "camelCase", default)]
pub struct Config {
  pub input: HashMap<String, String>,
  pub output: OutputConfig,
  pub root: String,
  pub mode: Mode,
  pub resolve: ResolveConfig,
  pub external: Vec<String>,
  pub runtime: RuntimeConfig,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      input: HashMap::new(),
      root: std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string(),
      output: OutputConfig::default(),
      mode: Mode::Development,
      resolve: ResolveConfig::default(),
      external: vec![],
      runtime: Default::default(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "camelCase", default)]
pub struct OutputConfig {
  path: String,
  public_path: String,
  filename: String,
}

impl Default for OutputConfig {
  fn default() -> Self {
    Self {
      filename: "[name].[contenthash].[ext]".to_string(),
      public_path: "/".to_string(),
      path: "dist".to_string(),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Mode {
  #[serde(rename = "development")]
  Development,
  #[serde(rename = "production")]
  Production,
}

impl Default for Mode {
  fn default() -> Self {
    Self::Development
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "camelCase", default)]
pub struct ResolveConfig {
  pub alias: HashMap<String, String>,
  pub main_fields: Vec<String>,
  pub main_files: Vec<String>,
  pub extensions: Vec<String>,
  pub conditions: Vec<String>,
  pub symlinks: bool,
}

impl Default for ResolveConfig {
  fn default() -> Self {
    Self {
      alias: HashMap::new(),
      main_fields: vec![
        String::from("browser"),
        String::from("module"),
        String::from("main"),
      ],
      main_files: vec![String::from("index")],
      extensions: vec![
        String::from("tsx"),
        String::from("ts"),
        String::from("jsx"),
        String::from("mjs"),
        String::from("js"),
        String::from("json"),
        String::from("html"),
        String::from("css"),
      ],
      conditions: vec![
        String::from("import"),
        String::from("require"),
        String::from("browser"),
        String::from("development"),
        String::from("production"),
        String::from("default"),
      ],
      symlinks: true,
    }
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "camelCase", default)]
pub struct RuntimeConfig {
  /// the compiled runtime file path, a runtime is required for script module loading, executing and hot module updating.
  pub path: String,
  /// the runtime plugins
  pub plugins: Vec<String>,
}
