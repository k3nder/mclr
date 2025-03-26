use crate::deserialize::json_version;
use dwldutil::{DLFile, Downloader};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonVersion {
    pub id: String,
    #[serde(alias = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(alias = "releaseTime")]
    pub release_time: String,
}
impl JsonVersion {
    pub fn save(&self, file: &str) {
        let dl = Downloader::new().add_file(DLFile::new().with_url(&self.url).with_path(file));
        dl.start();
    }
    pub fn save_and_load(&self, file: &str) -> json_version::JsonVersion {
        self.save(file);
        json_version::load(file)
    }
}
