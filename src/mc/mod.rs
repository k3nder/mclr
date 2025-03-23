pub mod utils;

use crate::deserialize::json_version::{Client, JavaVersion, JsonVersion, LogSettings};
use crate::utils::io_utils;
use crate::utils::io_utils::compress;
use dwldutil::{DLBuilder, DLFile};
use io_utils::system::OperatingSystem;
use std::path::Path;

pub struct JreUrls {
    pub windows: Box<JREPlatform>,
    pub other: Box<JREPlatform>,
}
pub struct JREPlatform {
    pub jre8: JRE,
    pub jre21: JRE,
}
#[derive(Clone)]
pub struct JRE {
    pub url: String,
    pub name: String,
    pub size: u64,
    pub sha1: String,
}
pub fn get_compatible_java(destination: &str, version: &JavaVersion) -> String {
    get_compatible_java_urls(destination, version, JreUrls {
        windows: Box::new(JREPlatform {
            jre8: JRE {
                url: "https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u422-b05/OpenJDK8U-jre_x64_windows_hotspot_8u442b05.zip".to_string(),
                name: "jdk8u422-b05".to_string(),
                size: 40132081,
                sha1: "ab2f7b10f5cf7607f142beb9b5933f2105399e8b".to_string()
            },
            jre21: JRE {
                url: "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.3%2B9/OpenJDK21U-jre_x64_windows_hotspot_21.0.3_9.zip".to_string(),
                name: "jdk-21.0.3+9-jre".to_string(),
                size: 48772289,
                sha1: "0a36d67c443387bab59e335b8a073d7d0f3a9575".to_string()
            }
        }),
        other: Box::new(JREPlatform {
            jre8: JRE {
                url: "https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u392-b08/OpenJDK8U-jre_x64_linux_hotspot_8u392b08.tar.gz".to_string(),
                name: "jdk8u392-b08-jre".to_string(),
                size: 41394316,
                sha1: "c79031d21c8c99c0f5e926c535f7216fe084721a".to_string()
            },
            jre21: JRE {
                url: "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.3%2B9/OpenJDK21U-jre_x64_linux_hotspot_21.0.3_9.tar.gz".to_string(),
                name: "jdk-21.0.3+9-jre".to_string(),
                size: 52430722,
                sha1: "b35f77729f5cc96a5ec7e7be97af391f6e3a60ca".to_string()
            }
        }),
    })
}
pub fn get_config_logger(log: &LogSettings, destination: &str) {
    let dl = DLBuilder::new().add_file(
        DLFile::new()
            .with_url(&log.client.file.url)
            .with_path(destination),
    );
    dl.start();
}
pub fn get_compatible_java_urls(destination: &str, version: &JavaVersion, urls: JreUrls) -> String {
    let system: OperatingSystem = OperatingSystem::detect();
    let platform: JREPlatform = match system {
        OperatingSystem::Windows => *urls.windows,
        OperatingSystem::Linux => *urls.other,
        _ => *urls.other,
    };

    let _jre = if version.major_version <= 8 {
        platform.jre8
    } else {
        platform.jre21
    };
    jre(_jre.clone(), destination);
    format!("{destination}/{}/bin/java", _jre.name)
}

fn jre(url: JRE, destination: &str) {
    if !Path::new(format!("{destination}/{}", url.name).as_str()).exists() {
        compress::download(
            url.url.as_str(),
            format!("{destination}").as_str(),
            url.size,
            &url.sha1,
        );
    }
}
pub fn download(destination: &str, json_version: &JsonVersion) {
    download_jar(&json_version.downloads.client, destination);
}
fn download_jar(client: &Client, file_str: &str) {
    let _path = Path::new(file_str);

    let dl = DLBuilder::new().add_file(DLFile::new().with_url(&client.url).with_path(file_str));
    dl.start();
}
