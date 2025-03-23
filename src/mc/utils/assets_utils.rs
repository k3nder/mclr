use dwldutil::{DLBuilder, DLFile, DLStartConfig};

use crate::deserialize::assets::Assets;
use crate::deserialize::json_version::JsonVersion;
use crate::utils::io_utils::verify_size;
use crate::utils::{io_utils, CounterEvent, HandleEvent};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::string::String;
use std::sync::Arc;

const BASE_URL: &str = "https://resources.download.minecraft.net";

pub fn save_indexes_load(file_str: &str, json: &JsonVersion) -> Assets {
    let indexes = &json.assetIndex;
    let dl = DLBuilder::new().add_file(
        DLFile::new()
            .with_url(&indexes.clone().url)
            .with_path(file_str),
    );
    dl.start();
    let content = std::fs::read_to_string(file_str).unwrap();
    serde_json::from_str(&content.as_str()).unwrap()
}

pub fn download_all(
    destination: &str,
    json_version: &JsonVersion,
    on_download: HandleEvent<String>,
    event: HandleEvent<CounterEvent>,
) {
    download_all_url(destination, json_version, event, on_download, BASE_URL)
}
pub fn check(
    destination: &str,
    json_version: &JsonVersion,
    counter: HandleEvent<CounterEvent>,
) -> bool {
    check_url(destination, json_version, counter, BASE_URL)
}
pub fn check_url(
    destination: &str,
    json_version: &JsonVersion,
    counter: HandleEvent<CounterEvent>,
    url: &str,
) -> bool {
    let assets = &save_indexes_load(
        format!(
            "{}/indexes/{}.json",
            destination,
            json_version.assets.clone().as_str()
        )
        .as_str(),
        json_version,
    );
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

        let obj_p = Path::new(&object_path);
        let value_p = Path::new(&path);

        if !verify_size(obj_p, value.size) || !verify_size(value_p, value.size) {
            ////println!("e");
            return false;
        }
        ////println!("{}", block);
    }
    true
}
pub fn download_all_url(
    destination: &str,
    json_version: &JsonVersion,
    event: HandleEvent<CounterEvent>,
    on_download: HandleEvent<String>,
    url: &str,
) {
    let assets = &save_indexes_load(
        format!(
            "{}/indexes/{}.json",
            destination,
            json_version.assets.clone().as_str()
        )
        .as_str(),
        json_version,
    );
    let mut index = 0;
    let mut files = Vec::new();
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
            let file = DLFile::new()
                .with_url(&url)
                .with_path(&object_path)
                .with_size(value.size)
                .with_on_download(Arc::new(move |obj_path| {
                    if !Path::new(&path).exists() {
                        let ppath = Path::new(&path);
                        if let Some(parent) = ppath.parent() {
                            fs::create_dir_all(parent).unwrap();
                        }
                        fs::copy(&obj_path, &path).expect("Failed to copy file");
                    }
                }));
            files.push(file);
        }
        index += 1;
        event.event(CounterEvent::new(assets.objects.len(), index))
        ////println!("{}", block);
    }

    let dl = DLBuilder::from_files(files);

    let config = DLStartConfig::new().with_max_concurrent_downloads(5);

    dl.start_with_config(config);
}
