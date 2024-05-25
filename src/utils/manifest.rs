use reqwest::Error;
use crate::deserialize::json_manifest;
use crate::utils::io_utils::{get, get_string};
pub async fn manifest() -> json_manifest::Manifest {
    manifest_url("https://launchermeta.mojang.com/mc/game/version_manifest.json")
}
pub async fn manifest_url(url: &str) -> json_manifest::Manifest {
    serde_json::from_str(&*get_string(url))
}