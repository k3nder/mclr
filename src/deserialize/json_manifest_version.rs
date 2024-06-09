use std::fs::File;
use std::io::Write;
use serde::Deserialize;
use crate::deserialize::json_version;
use crate::utils::io_utils::{get_string};
use crate::utils::sync_utils::sync;

#[derive(Deserialize, Debug)]
pub struct JsonVersion {
    pub id: String,
    #[serde(rename = "type")] pub versionType: String,
    pub url: String,
    pub time: String,
    pub releaseTime: String
}
impl JsonVersion {
    pub fn save(&self ,file: &str) {
        let mut file_file = File::create(file).unwrap();
        let response = get_string(self.url.as_str());
        let content = sync().block_on(response);
        file_file.write_all(content.expect("error").as_ref()).expect("TODO: panic message");
    }
    pub fn save_and_load(&self, file: &str) -> json_version::JsonVersion {
        self.save(file);
        json_version::load(file)
    }
    //pub fn quilt_loader(v: &str) -> str {
    //    //return sync().block_on(get_string(format!("https://meta.quiltmc.org/v3/versions/loader/{}", v).as_str()))
    //}

    //pub fn quilt(vanilla: &str, loader: &str) -> JsonVersion {
//
    //}
}