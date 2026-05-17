pub mod config;

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use serde::Deserialize;
use tracing::error;

use config::world::WorldData;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
	pub world: WorldData,
}

#[derive(Debug)]
pub enum ConfigError {
	Io { path: String, source: std::io::Error },
	Yaml { path: String, source: serde_yaml::Error },
	AlreadyInitialized,
}

impl fmt::Display for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ConfigError::Io { path, .. } => {
				write!(f, "Failed to read config file: {}", path)
			}
			ConfigError::Yaml { path, .. } => {
				write!(f, "Failed to parse config file: {}", path)
			}
			ConfigError::AlreadyInitialized => {
				write!(f, "Config already initialized")
			}
		}
	}
}

impl Error for ConfigError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			ConfigError::Io { source, .. } => Some(source),
			ConfigError::Yaml { source, .. } => Some(source),
			ConfigError::AlreadyInitialized => None,
		}
	}
}

fn config_cell() -> &'static OnceLock<Config> {
	static CONFIG: OnceLock<Config> = OnceLock::new();
	&CONFIG
}

pub fn load_from_yaml<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
	let path_ref: &Path = path.as_ref();
	let path_str: String = path_ref.to_string_lossy().to_string();

	let contents: String = fs::read_to_string(path_ref).map_err(|source| {
		error!(path = %path_str, error = %source, "Failed to read config file");
		ConfigError::Io {
			path: path_str.clone(),
			source,
		}
	})?;

	let config: Config = serde_yaml::from_str(&contents).map_err(|source| {
		error!(path = %path_str, error = %source, "Failed to parse config file");
		ConfigError::Yaml {
			path: path_str.clone(),
			source,
		}
	})?;

	Ok(config)
}

pub fn init_global<P: AsRef<Path>>(path: P) -> Result<&'static Config, ConfigError> {
	let config = load_from_yaml(path)?;
	config_cell()
		.set(config)
		.map_err(|_| ConfigError::AlreadyInitialized)?;
	Ok(config_cell()
		.get()
		.expect("Config was just initialized"))
}

#[allow(dead_code)]
pub fn get() -> &'static Config {
	config_cell()
		.get()
		.expect("Config not initialized. Call config::init_global() first.")
}

#[allow(unused_imports)]
pub use config::*;
