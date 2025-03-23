use bytes::Bytes;
use hex::encode;
use std::fs;
use std::fs::File;
use std::io::{copy, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use dwldutil::{DLBuilder, DLFile};

pub fn verify_size(_path: &Path, _size: u64) -> bool {
    // let file = File::open(_path).unwrap();
    // let metadata = file.metadata().unwrap();

    // metadata.len().eq(&_size)
    true
}

fn get_parent_directory(path: &Path) -> Option<PathBuf> {
    // Usa el método 'parent' para obtener el directorio padre
    path.parent().map(|p| p.to_path_buf())
}

pub mod compress {
    use crate::utils::io_utils;
    use crate::utils::io_utils::system::OperatingSystem;
    use crate::utils::io_utils::{get_resource_name, verify_size};
    use dwldutil::decompress::DLDecompressionConfig;
    use dwldutil::decompress::DecompressionMethod;
    use dwldutil::{DLBuilder, DLFile, DLHashes};
    use flate2::read::GzDecoder;
    use std::fs::{create_dir_all, File};
    use std::io::{BufReader, Read};
    use std::path::Path;
    use std::{fs, io};
    use tar::Archive;
    use zip::ZipArchive;

    pub fn download(url: &str, destination: &str, _size: u64, _sha1: &String) {
        let binding = get_resource_name(url).unwrap();
        let file: &str = binding.as_str();

        let dl = DLBuilder::new().add_file(
            DLFile::new()
                .with_url(url)
                .with_path(file)
                .with_size(_size)
                .with_hashes(DLHashes::new().sha1(_sha1))
                .with_decompression_config(DLDecompressionConfig::new(
                    DecompressionMethod::TarGzip,
                    destination,
                )),
        );

        dl.start();
    }
}

use url::Url;

pub fn get_resource_name(url_str: &str) -> Option<String> {
    // Parsea la URL
    let url = Url::parse(url_str).ok()?;

    // Obtiene los segmentos de la ruta de la URL
    let path_segments: Vec<_> = url.path_segments()?.collect();

    // Devuelve una copia del último segmento de la ruta (nombre del recurso)
    path_segments.last().map(|s| s.to_string())
}

pub mod system {
    #[derive(Debug)]
    pub enum OperatingSystem {
        Linux,
        Windows,
        MacOS,
        Other,
    }

    impl OperatingSystem {
        pub fn detect() -> Self {
            if cfg!(target_os = "linux") {
                OperatingSystem::Linux
            } else if cfg!(target_os = "windows") {
                OperatingSystem::Windows
            } else if cfg!(target_os = "macos") {
                OperatingSystem::MacOS
            } else {
                OperatingSystem::Other
            }
        }
        pub fn name(&self) -> &str {
            match self {
                OperatingSystem::Linux => "linux",
                OperatingSystem::Windows => "windows",
                OperatingSystem::MacOS => "osx",
                _ => "unknow",
            }
        }
    }
}
