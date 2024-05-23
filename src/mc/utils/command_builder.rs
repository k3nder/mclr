use std::fmt::format;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use crate::deserialize::json_version::LibraryNatives;

pub struct Command {
    pub(crate) resources: CommandResourcesConfig,
    pub(crate) java_home: String,
    pub(crate) game_dir: String,
    pub(crate) assets: CommandAssetsConfig,
    pub(crate) user: CommandUserConfig,
    pub(crate) version: CommandVersionConfig,
    pub(crate) ram: CommandRamConfig,
    pub(crate) event: fn(String)
}
pub struct CommandResourcesConfig {
    pub(crate) libraries: String,
    pub(crate) jar_file: String,
    pub(crate) bin: String
}
pub struct CommandRamConfig {
    pub(crate) xmx: i32,
    pub(crate) xms: i32
}
pub struct CommandAssetsConfig {
    pub(crate) assets_dir: String,
    pub(crate) assets_index: String
}
pub struct CommandVersionConfig {
    pub(crate) version_id: String,
    pub(crate) version_type: String,
    pub(crate) main_class: String
}
pub struct CommandUserConfig {
    pub(crate) user_type: String,
    pub(crate) client_id: String,
    pub(crate) uuid: String,
    pub(crate) xuid: String,
    pub(crate) access_token: String,
    pub(crate) user_name: String
}
impl Command {
    pub fn run(&self) {
        let mut child = std::process::Command::new(self.java_home.as_str())
            .arg(format!("-Djna.tmpdir={}", self.resources.bin))
            .arg(format!("-Dio.netty.native.workdir={}", self.resources.bin))
            .arg(format!("-Djava.library.path={}", self.resources.bin))
            .arg("-cp")
            .arg(format!(".;{};{}\\*", self.resources.jar_file, self.resources.libraries))
            .arg(self.version.main_class.as_str())
            .arg("--version")
            .arg(self.version.version_id.as_str())
            .arg("--versionType")
            .arg(self.version.version_type.as_str())
            .arg("--accessToken")
            .arg(self.user.access_token.as_str())
            .arg("--uuid")
            .arg(self.user.uuid.as_str())
            .arg("--xuid")
            .arg(self.user.xuid.as_str())
            .arg("--clientId")
            .arg(self.user.client_id.as_str())
            .arg("--username")
            .arg(self.user.user_name.as_str())
            .arg("--userType")
            .arg(self.user.user_type.as_str())
            .arg("--assetIndex")
            .arg(self.assets.assets_index.as_str())
            .arg("--assetsDir")
            .arg(self.assets.assets_dir.as_str())
            .stdout(Stdio::piped())
            .spawn().unwrap();
        // Obtener el stdout del proceso hijo
        let stdout = child.stdout.take().expect("Failed to capture stdout");

        // Leer la salida del proceso hijo de manera asíncrona
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            (self.event)(line.unwrap())
        }

        // Esperar a que el proceso hijo termine
        child.wait().unwrap();
    }
}