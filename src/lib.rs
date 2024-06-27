use std::path::PathBuf;

pub mod manifest;
pub mod masterbundle;

pub fn parse_masterbundle(path: PathBuf) -> anyhow::Result<()> {
    use masterbundle::MasterBundle;

    let bundle = MasterBundle::new(path)?;
    let _ = bundle.parse();

    Ok(())
}
