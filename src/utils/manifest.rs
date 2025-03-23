use std::fs::File;
use std::io::Read;

use dwldutil::{DLBuilder, DLFile};

use crate::deserialize::json_manifest::Manifest;

pub fn manifest() -> Manifest {
    manifest_url("https://launchermeta.mojang.com/mc/game/version_manifest.json")
}
pub fn manifest_url(url: &str) -> Manifest {
    let dl = DLBuilder::new().add_file(
        DLFile::new()
            .with_url(url)
            .with_path("version_manifest.json"),
    );
    dl.start();
    let mut buffer = String::new();
    File::open("version_manifest.json")
        .expect("Cannot open the manifest")
        .read_to_string(&mut buffer)
        .expect("Cannot read the manifest");
    serde_json::from_str(&buffer).expect("Cannot parse the manifest")
}
