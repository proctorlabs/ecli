use super::*;
use crate::config::*;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

const HEADER: &str = include_str!("shebang.yml");

pub fn generate(file: PathBuf, add_shebang: bool) -> Result<()> {
    let config = AppConfig::default();
    let mut f = std::fs::File::create(file)?;
    let metadata = f.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    f.set_permissions(permissions)?;

    if add_shebang {
        write!(f, "{}#", HEADER)?;
    }

    serde_yaml::to_writer(f, &config)?;
    Ok(())
}
