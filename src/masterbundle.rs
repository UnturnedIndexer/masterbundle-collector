use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::manifest::Manifest;

pub struct MasterBundle {
    pub name: String,
    pub path: PathBuf,
    pub asset_prefix: String,
}

impl MasterBundle {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let file = std::fs::read_to_string(&path)?;

        let lines = file.lines();
        let mut asset_prefix = String::new();
        let mut name = String::new();

        for line in lines {
            let mut values = line.split_whitespace();

            let field = values.next().unwrap_or("");
            let value = values.next().unwrap_or("");

            match field {
                "Asset_Bundle_Name" => name = value.into(),
                "Asset_Prefix" => asset_prefix = value.into(),
                _ => {} // Do nothing, we dont care.
            }
        }

        Ok(Self {
            name,
            path,
            asset_prefix,
        })
    }

    pub fn parse(&self) -> anyhow::Result<Vec<PathBuf>> {
        let mut path = PathBuf::from(&self.path);
        path.pop(); // remove `MasterBundle.dat`
        path.push(format!("{}.manifest", &self.name)); // cursed, lol

        let manifest =
            std::fs::read_to_string(path).context("failed to read manifest to string")?;
        let manifest: Manifest = serde_yaml::from_str(&manifest).context("what the sigma")?;

        let assets: Vec<PathBuf> = manifest.assets.into_iter().map(PathBuf::from).collect();

        let assets: Vec<&Path> = assets
            .iter()
            .filter_map(|x| match x.strip_prefix(&self.asset_prefix) {
                Ok(path) => Some(path),
                Err(_) => None,
            })
            .collect();

        let directories = assets
            .into_iter()
            .filter(|x| match x.extension() {
                Some(ext) => ext == OsStr::new("prefab"),
                None => false,
            })
            .map(|x| x.parent().unwrap()) // probably not safe, fuck it we ball.
            .collect::<Vec<_>>();

        let dat_paths: Vec<PathBuf> = directories
            .iter()
            .map(|original_prefab| {
                let stem = original_prefab.file_stem().unwrap().to_str().unwrap(); // ohh boy
                let mut newy = PathBuf::from(original_prefab);
                newy.push(format!("{stem}.dat"));
                newy
            })
            .collect();

        Ok(dat_paths)
    }
}
