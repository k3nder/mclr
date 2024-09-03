use std::string::String;
use std::fs;
use std::path::Path;
use crate::deserialize::assets::Assets;
use crate::deserialize::json_version::JsonVersion;
use crate::utils::{CounterEvent, HandleEvent, io_utils};
use crate::utils::io_utils::download;

const BASE_URL: &str = "https://resources.download.minecraft.net";

pub fn save_indexes_load(file_str: &str, json: &JsonVersion) -> Assets {
    let indexes = &json.assetIndex;
    io_utils::download(file_str, &indexes.clone().url);
    let content = std::fs::read_to_string(file_str).unwrap();
    return serde_json::from_str(&content.as_str()).unwrap();
}

pub fn download_all(destination: &str, json_version: &JsonVersion,on_download: HandleEvent<String> , event: HandleEvent<CounterEvent>) {
    download_all_url(destination, json_version, event, on_download, BASE_URL)
}
pub fn verify(destination: &str, json_version: &JsonVersion, counter: HandleEvent<CounterEvent>) -> bool {
    verify_url(destination, json_version, counter, BASE_URL)
}
pub fn verify_url(destination: &str, json_version: &JsonVersion, counter: HandleEvent<CounterEvent>, url: &str) -> bool {
    let assets = &save_indexes_load(format!("{}/indexes/{}.json", destination, json_version.assets.clone().as_str()).as_str(), json_version);
    let mut index = 0;
    for (key, value) in &assets.objects {
        let hash = &value.hash;
        let block = &hash[..2];
        let _url = format!("{}/{}/{}", url, block, hash);
        //////println!("{}", url);
        let key_path = key.as_str();
        let path = format!("{}/virtual/legacy/{}", destination, key_path);
        let object_path = format!("{}/objects/{}/{}", destination, block, hash);
        ////println!("{}::::{}", path, object_path);
        index += 1;
        ////println!("{}", key);
        counter.event(CounterEvent::new(assets.objects.len(), index));


        //if ( verify_size(obj_p, value.size) || verify_size(value_p, value.size)) {
        //    ////println!("e");
        //    return false;
        //}
        ////println!("{}", block);
    }
    true
}
pub fn download_all_url(destination: &str, json_version: &JsonVersion, event: HandleEvent<CounterEvent>, on_download: HandleEvent<String>, url: &str) {
    let assets = &save_indexes_load(format!("{}/indexes/{}.json", destination, json_version.assets.clone().as_str()).as_str(), json_version);
    let mut index = 0;
    for (key, value) in &assets.objects {
        let hash = &value.hash;
        let block = &hash[..2];
        let url = format!("{}/{}/{}", url, block, hash);
        ////println!("{}", url);
        let key_path = key.as_str();
        if !std::path::Path::new(format!("{}/virtual/legacy/", destination).as_str()).exists() {
            fs::create_dir_all(format!("{}/virtual/legacy/", destination)).unwrap();
        }
        if !std::path::Path::new(format!("{}/objects/{}", destination, block).as_str()).exists() {
            fs::create_dir_all(format!("{}/objects/{}", destination, block)).unwrap();
        }
        let path = format!("{}/virtual/legacy/{}", destination, key_path);
        let object_path = format!("{}/objects/{}/{}", destination, block, hash);
        ////println!("{}::::{}", path, object_path);
        if !Path::new(&object_path).exists() {
            download(&object_path, &url);
        }
        if !Path::new(&path).exists() {
            download(&path, &url);
            on_download.event(url);
        }
        index += 1;
        event.event(CounterEvent::new(assets.objects.len(), index))
        ////println!("{}", block);
    }
}