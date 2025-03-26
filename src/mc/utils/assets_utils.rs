use dwldutil::cas::DLStorage;
use dwldutil::{DLFile, DLHashes, Downloader};

use crate::deserialize::assets::Assets;
use crate::deserialize::json_version::JsonVersion;
use crate::utils::{CounterEvent, HandleEvent};
use std::fs;
use std::path::Path;
use std::sync::Arc;

const BASE_URL: &str = "https://resources.download.minecraft.net";

pub fn save_indexes_load(file_str: &str, json: &JsonVersion) -> Assets {
    let indexes = &json.asset_index;
    let dl = Downloader::new().add_file(
        DLFile::new()
            .with_url(&indexes.clone().url)
            .with_path(file_str),
    );
    dl.start();
    let content = std::fs::read_to_string(file_str).unwrap();
    serde_json::from_str(&content.as_str()).unwrap()
}

pub fn find(
    destination: &str,
    json_version: &JsonVersion,
    event: HandleEvent<CounterEvent>,
) -> Downloader {
    find_url(destination, json_version, event, BASE_URL)
}

pub fn find_url(
    destination: &str,
    json_version: &JsonVersion,
    event: HandleEvent<CounterEvent>,
    url: &str,
) -> Downloader {
    let assets = &save_indexes_load(
        format!(
            "{}/indexes/{}.json",
            destination,
            json_version.assets.clone().as_str()
        )
        .as_str(),
        json_version,
    );

    let object_path = format!("{}/objects", destination);

    let mut index = 0;
    let mut files = Vec::new();
    let mut downloader = Downloader::new();
    let storage = DLStorage::new(object_path.as_str());
    for (key, value) in &assets.objects {
        let hash = &value.hash;
        let block = &hash[..2];
        let url = format!("{}/{}/{}", url, block, hash);
        let key_path = key.as_str();
        if !std::path::Path::new(format!("{}/virtual/legacy/", destination).as_str()).exists() {
            fs::create_dir_all(format!("{}/virtual/legacy/", destination)).unwrap();
        }
        let path = format!("{}/virtual/legacy/{}", destination, key_path);
        if !Path::new(&path).exists() {
            let file = DLFile::new()
                .with_url(&url)
                .with_path(&path)
                .with_size(value.size)
                .with_hashes(DLHashes::new().sha1(hash))
                .with_cas(storage.clone());
            files.push(file);
        }
        index += 1;
        event.event(CounterEvent::new(assets.objects.len(), index))
    }
    downloader.with_files(files)
}
