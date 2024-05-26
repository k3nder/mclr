use crate::deserialize::json_manifest::Manifest;
use crate::utils::io_utils::{get_string};
pub async fn manifest() -> Manifest {
    manifest_url("https://launchermeta.mojang.com/mc/game/version_manifest.json").await
}
pub async fn manifest_url(url: &str) -> Manifest {
    let r: Manifest = serde_json::from_str(&*get_string(url).await.expect("Error in url, cannot make a Request to Manifest")).expect("Cannot parse the manifest");
    r
}