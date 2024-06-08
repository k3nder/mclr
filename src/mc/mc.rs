use std::fmt::format;
use std::path::Path;
use crate::utils::io_utils;
use crate::utils::io_utils::{compress};
use io_utils::system::OperatingSystem;
use crate::deserialize::json_version::{Client, JavaVersion, JsonVersion};
use crate::utils::io_utils::compress::verify_integrity;

pub struct JreUrls {
    pub windows: Box<JreUrl>,
    pub other: Box<JreUrl>
}
pub struct JreUrl {
    pub jre8: String,
    pub jre22: String
}
pub fn get_compatible_java(destination: &str, version: &JavaVersion) -> String {
    get_compatible_java_urls(destination, version, JreUrls {
        windows: Box::new(JreUrl {
            jre8: "https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u382-b05/OpenJDK8U-jre_x64_windows_hotspot_8u382b05.zip".to_string(),
            jre22: "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.3%2B9/OpenJDK21U-jre_x64_windows_hotspot_21.0.3_9.zip".to_string()
        }),
        other: Box::new(JreUrl {
            jre8: "https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u392-b08/OpenJDK8U-jdk_x64_linux_hotspot_8u392b08.tar.gz".to_string(),
            jre22: "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.3%2B9/OpenJDK21U-jre_x64_linux_hotspot_21.0.3_9.tar.gz".to_string()
        }),
    })
}
pub fn get_compatible_java_urls(destination: &str, version: &JavaVersion, urls: JreUrls) -> String {
    let system: OperatingSystem = OperatingSystem::detect();

    let jre8: &str = match system {
        OperatingSystem::Windows => urls.windows.jre8.as_str(),
        _ => urls.other.jre8.as_str(),
    };

    let jre20: &str = match system {
        OperatingSystem::Windows => urls.windows.jre22.as_str(),
        _ => urls.other.jre22.as_str(),
    };

    if !Path::new(format!("{destination}/8").as_str()).exists() {
        compress::download(&jre8, format!("{destination}/8").as_str());
    }
    if !Path::new(format!("{destination}/20").as_str()).exists() {
        compress::download(&jre20, format!("{destination}/20").as_str());
    }
    // calculate compatible java
    let java_version = version.majorVersion;
    return if java_version <= 8 {
        format!("{}\\8\\jdk8u382-b05-jre\\bin\\java", destination).to_string()
    } else {
        format!("{}\\20\\jdk-21.0.3+9-jre\\bin\\java", destination).to_string()
    }
}
pub fn download(destination: &str,json_version: &JsonVersion) {
    download_jar(&json_version.downloads.client, destination);
}
fn download_jar(client: &Client, file_str: &str){
    if !verify_integrity(client.size, file_str) {
        io_utils::download(file_str, client.url.as_str());
    }
}