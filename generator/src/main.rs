use std::{collections::HashMap, path::Path, process::Command};

use anyhow::{anyhow, ensure, Result};
use clap::Parser;
use fs_err as fs;
use serde::Deserialize;

mod generator;
mod json_loader;

#[derive(Debug, Parser)]
struct Args {
    package: String,
}

const CONF: &str = include_str!("../config.toml");
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Deserialize)]
struct PackageConfigToml {
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    version: Option<String>,
}

struct PackageConfig {
    exclude: Vec<glob::Pattern>,
    version: Option<String>,
}

impl PackageConfig {
    fn new(toml: &PackageConfigToml) -> Result<Self> {
        let exclude = toml
            .exclude
            .iter()
            .map(|s| glob::Pattern::new(s))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            exclude,
            version: toml.version.clone(),
        })
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let configs: HashMap<String, PackageConfigToml> = toml::from_str(CONF)?;

    let package = &args.package;
    if package == "all" {
        for i in configs.keys() {
            do_package(&configs, i)?;
        }
    } else {
        do_package(&configs, &package)?;
    }
    Ok(())
}

fn do_package(
    configs: &HashMap<String, PackageConfigToml>,
    package: &str,
) -> Result<(), anyhow::Error> {
    let config = PackageConfig::new(configs.get(package).ok_or_else(|| {
        anyhow!(
            "No known package {package}, all I know is {:?}",
            configs.keys().collect::<Vec<_>>()
        )
    })?)?;
    let workspace_root = Path::new(CARGO_MANIFEST_DIR).parent().unwrap();

    let package_spec = match &config.version {
        Some(v) => format!("{package}:{v}"),
        None => package.to_owned(),
    };

    ensure!(Command::new("cargo")
        .args([
            "+nightly",
            "rustdoc",
            "-p",
            &package_spec,
            "--features",
            &package_spec,
            "--",
            "-w",
            "json",
            "-Z",
            "unstable-options",
        ])
        .current_dir(workspace_root)
        .status()?
        .success());

    let package = package.replace("-", "_");
    let json_path = workspace_root
        .join("target")
        .join("doc")
        .join(format!("{package}.json"));
    let output_path = workspace_root
        .join("src")
        .join("gen_impls")
        .join(format!("{package}.rs"));
    let json = fs::read_to_string(&json_path)?;
    let krate = json_loader::load_rjd(&json)?;
    let rust = generator::generate(&krate, &config)?;

    fs::write(output_path, rust)?;
    Ok(())
}
