use std::fs;
use std::fs::File;
use std::io::{copy, Read};
use std::path::{Path, PathBuf};
use std::time::Duration;
use bytes::Bytes;
use reqwest::{Client, Error};

use crate::utils::sync_utils::sync;

pub async fn get(url: &String) -> Result<Bytes, Error> {
    return match reqwest::get(url).await {
        Ok(response) => {
            let value = response.bytes().await?;
            return Ok(value);
        }
        Err(err) => {
            Err(err)
        }
    };
}

pub async fn get_string(url: &str) -> Result<String, Error> {
    return match reqwest::get(url).await {
        Ok(response) => {
            let value = response.text().await?;
            return Ok(value);
        }
        Err(err) => {
            Err(err)
        }
    };
}

fn fetch_data(url: &str) -> Result<Bytes, Error> {
    let client = Client::new();
    let response = sync().block_on(client.get(url)
        .timeout(Duration::from_secs(10)) // Establece un timeout de 10 segundos
        .send()).unwrap()
        .error_for_status();

    Ok(sync().block_on(response.unwrap().bytes()).unwrap())
}

pub fn download(file_str: &str, url: &str) {
    // Realiza la solicitud GET para obtener el contenido del archivo

    let response = reqwest::blocking::get(url);
    match response {
        Ok(response) => {
            let parent_dir = get_parent_directory(Path::new(file_str)).unwrap();
            if !parent_dir.exists() { fs::create_dir_all(parent_dir).expect("Cannot create dir") }
            // Abre un archivo en modo de escritura para guardar el contenido descargado
            let mut dest = File::create(file_str).unwrap();

            // Copia el contenido de la respuesta HTTP al archivo
            let content = response.bytes().unwrap();
            copy(&mut content.as_ref(), &mut dest).unwrap();
        }
        Err(_e) => {
            download(file_str, url);
        }
    }
}

fn get_parent_directory(path: &Path) -> Option<PathBuf> {
    // Usa el método 'parent' para obtener el directorio padre
    path.parent().map(|p| p.to_path_buf())
}

pub mod compress {
    use std::fs::{create_dir_all, File, metadata};
    use std::{fs, io};
    use std::io::{BufReader, Read};
    use std::path::{Path};
    use flate2::read::GzDecoder;
    use sha1::{Digest, Sha1};
    use tar::Archive;
    use zip::{ZipArchive};
    use crate::utils::io_utils;
    use crate::utils::io_utils::get_resource_name;

    pub fn extract_zip(destination: &str, file_str: &str) {
        let filepath = Path::new(file_str);
        ////println!("{}", filepath.display());
        let file = std::fs::File::open(&filepath).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        let output_dir = Path::new(destination);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = output_dir.join(file.sanitized_name());

            if (&*file.name()).ends_with('/') {
                create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = std::fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }

    pub fn is_tar_gz(file_path: &str) -> io::Result<bool> {
        let mut file = std::fs::File::open(file_path).unwrap();

        let mut magic_bytes = [0u8; 2];
        file.read_exact(&mut magic_bytes).unwrap();

        if magic_bytes == [0x1F, 0x8B] {
            // Comienza con la cabecera GZIP
            let mut decoder = GzDecoder::new(file);
            let mut tar_magic_bytes = [0u8; 4];
            decoder.read_exact(&mut tar_magic_bytes).unwrap();

            // Verifica si los siguientes bytes corresponden a un archivo TAR
            Ok(&tar_magic_bytes[..] == b"ustar")
        } else {
            // No es un archivo TAR.GZ
            Ok(false)
        }
    }

    pub fn is_zip(file_path: &str) -> io::Result<bool> {
        let mut file = std::fs::File::open(file_path).unwrap();

        let mut magic_bytes = [0u8; 4];
        file.read_exact(&mut magic_bytes).unwrap();

        // Verifica si los primeros bytes corresponden a una firma ZIP
        Ok(&magic_bytes[..] == b"PK\x03\x04")
    }

    pub fn extract(destination: &str, file_str: &str) {
        if is_zip(file_str).unwrap() { extract_zip(destination, file_str); } else if is_tar_gz(file_str).unwrap() { extract_tar(destination, file_str); }
    }

    pub fn extract_tar(destination: &str, file_str: &str) {
        let tar_gz_path = Path::new(file_str);
        let tar_gz = std::fs::File::open(&tar_gz_path).unwrap();
        let tar_buf = BufReader::new(tar_gz);
        let gz_decoder = GzDecoder::new(tar_buf);
        let mut archive = Archive::new(gz_decoder);

        let output_dir = Path::new(destination);
        archive.unpack(output_dir).unwrap()
    }

    pub fn download(url: &str, destination: &str) {
        let binding = get_resource_name(url).unwrap();
        let FILE: &str = binding.as_str();
        io_utils::download(FILE, url);
        extract(destination, FILE);
        fs::remove_file(FILE).expect("Cannot remove temp file jre");
    }
    pub fn verify_integrity(len: u64, file: &str) -> bool {
        if Path::new(file).exists() {
            return metadata(file).unwrap().len() == len;
            //if compute_sha1(file).unwrap().eq(sha1) {
            //    return true;
            //}
        }
        return false;
    }
    fn compute_sha1<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
        let mut file = File::open(file_path).unwrap();
        let mut hasher = Sha1::new();
        let mut buffer = [0; 4096];

        loop {
            let n = file.read(&mut buffer).unwrap();
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
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
    }
}