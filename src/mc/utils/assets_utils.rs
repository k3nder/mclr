use std::fmt::format;
use std::fs;
use std::path::Path;
use serde_json::Value::String;
use crate::deserialize::assets::Assets;
use crate::deserialize::json_version::JsonVersion;
use crate::mc;
use crate::utils::io_utils;
use crate::utils::io_utils::download;

const BASE_URL: &str = "https://resources.download.minecraft.net";

pub fn save_indexes_load(file_str: &str, json: &JsonVersion) -> Assets {
    let indexes = &json.assetIndex;
    io_utils::download(file_str, &indexes.url);
    let content = std::fs::read_to_string(file_str).unwrap();
    return serde_json::from_str(&content.as_str()).unwrap();
}
pub fn download_all(destination: &str, json_version: &JsonVersion) {
    let assets = &mc::utils::assets_utils::save_indexes_load(format!("{}\\indexes\\{}.json", destination, json_version.assets.as_str()).as_str(), json_version);
    for (key, value) in &assets.objects {
        let hash = &value.hash;
        let block = &hash[..2];
        let url = format!("{}/{}/{}", BASE_URL, block, hash);
        println!("{}", url);
        let key_path = key.as_str();
        if !std::path::Path::new(format!("{}/virtual/legacy/", destination).as_str()).exists() {
            fs::create_dir_all(format!("{}/virtual/legacy/", destination)).unwrap();
        }
        if !std::path::Path::new(format!("{}/objects/{}", destination, block).as_str()).exists() {
            fs::create_dir_all(format!("{}/objects/{}", destination, block)).unwrap();
        }
        let path = format!("{}/virtual/legacy/{}", destination, key_path);
        let object_path = format!("{}/objects/{}/{}", destination, block, hash);
        //println!("{}::::{}", path, object_path);
        if !Path::new(&object_path).exists() {
            download(&object_path, &url);
        }
        if !Path::new(&path).exists() {
            download(&path, &url);
        }
        println!("{}", block);
    }
}