use crate::deserialize::json_manifest::Manifest;
use crate::utils::sync_utils::sync;


pub fn manifest() -> Manifest {
    manifest_url("https://launchermeta.mojang.com/mc/game/version_manifest.json")
}
pub async fn manifest_url(url: &str) -> Manifest {
    let res = ureq::get(url).call().unwrap().body().read_to_string().unwrap();
    serde_json::from_str(&res).expect("Cannot parse the manifest")
}