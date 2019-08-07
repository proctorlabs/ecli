use super::*;
use crate::config::*;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

const HEADER: &str = r#"#!/usr/bin/env sh
# Add .ecli to YAML file associations to get syntax highlighting, or just use a yaml file with
# the shebang line
"": | # This hackery is for compatibility with a variety of shebang lines while also keeping the yaml valid
  true;
  exec cargo run -- $0
# "#;

pub fn generate(file: PathBuf) -> Result<()> {
    let config = AppConfig::default();
    let mut f = std::fs::File::create(file)?;
    let metadata = f.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    f.set_permissions(permissions)?;

    write!(f, "{}", HEADER)?;
    serde_yaml::to_writer(f, &config)?;
    Ok(())
}
