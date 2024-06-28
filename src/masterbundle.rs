use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::manifest::Manifest;

#[derive(Debug)]
pub struct MasterBundle {
    /// The name of the [`MasterBundle`], gotten from `MasterBundle.dat`.
    pub name: String,
    // All of the assets inside the [`MasterBundle`] manifest.
    pub assets: Vec<PathBuf>,
}

#[derive(Debug, Default)]
pub struct MasterBundleData {
    pub name: String,
    pub asset_prefix: String,
}

impl MasterBundle {
    /// Creates a new [`MasterBundle`].
    pub fn new<P: Into<PathBuf>>(path: P) -> anyhow::Result<Self> {
        let path: PathBuf = path.into();

        let mut masterbundle_location = PathBuf::from(&path);
        masterbundle_location.push("MasterBundle.dat");

        let masterbundle_data = Self::parse_masterbundle_data(&masterbundle_location)
            .context("Failed to parse MasterBundle data")?;
        let assets = Self::parse(
            path,
            &masterbundle_data.name,
            &masterbundle_data.asset_prefix,
        )
        .context("Failed to parse MasterBundle assets")?;

        Ok(Self {
            name: masterbundle_data.name,
            assets,
        })
    }

    /// Gathers the [`MasterBundle`] assets.
    pub fn parse<P: Into<PathBuf>>(
        path: P,
        name: &str,
        asset_prefix: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
        let mut manifest_location = path.into();
        manifest_location.push(format!("{}.manifest", name));

        let manifest_file = std::fs::File::open(&manifest_location)
            .with_context(|| format!("Failed to open file: {}", manifest_location.display()))?;
        let manifest: Manifest =
            serde_yaml::from_reader(manifest_file).context("Failed to parse manifest file")?;

        let assets: Vec<PathBuf> = manifest.assets.into_iter().map(PathBuf::from).collect();
        let assets: Vec<&Path> = assets
            .iter()
            .filter_map(|asset| match asset.strip_prefix(asset_prefix) {
                Ok(path) => Some(path),
                Err(_) => None,
            })
            .collect();
        let assets: Vec<PathBuf> = assets.into_iter().map(PathBuf::from).collect();

        Ok(assets)
    }

    /// Reads a `MasterBundle.dat` file and returns its name and asset prefix
    pub fn parse_masterbundle_data(path: &Path) -> anyhow::Result<MasterBundleData> {
        let path: PathBuf = path.into();
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let mut data = MasterBundleData::default();

        for line in contents.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Asset_Bundle_Name" => data.name = value.into(),
                "Asset_Prefix" => data.asset_prefix = value.into(),
                _ => {}
            }
        }

        Ok(data)
    }
}
