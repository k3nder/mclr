use std::fmt::format;
use std::path::Path;
use serde_json::json;
use crate::utils::io_utils;
use crate::utils::io_utils::{compress};
use io_utils::system::OperatingSystem;
use crate::deserialize::json_version::{Client, ClientDownloads, JavaVersion, JsonVersion};


pub fn get_compatible_java(destination: &str, version: &JavaVersion) -> String {
    let system: OperatingSystem = OperatingSystem::detect();

    let jre8: &str = match system {
        OperatingSystem::Windows => "https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u382-b05/OpenJDK8U-jre_x64_windows_hotspot_8u382b05.zip",
        _ => "https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u392-b08/OpenJDK8U-jdk_x64_linux_hotspot_8u392b08.tar.gz",
    };

    let jre20: &str = match system {
        OperatingSystem::Windows => "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.3%2B9/OpenJDK21U-jre_x64_windows_hotspot_21.0.3_9.zip",
        _ => "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.3%2B9/OpenJDK21U-jre_x64_linux_hotspot_21.0.3_9.tar.gz",
    };

    compress::download(&jre8, format!("{destination}/8").as_str());
    compress::download(&jre20, format!("{destination}/20").as_str());

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
    io_utils::download(file_str, client.url.as_str());
}