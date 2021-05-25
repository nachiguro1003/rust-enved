use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};
use std::env;

pub fn read_yaml(file_name: &str) -> Vec<u8> {
    let relative_path = Path::new(file_name);
    let pwd = env::current_dir().unwrap();
    let target = pwd.join(relative_path);

    let mut file = std::fs::File::open(target).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    buf
}

pub fn parse_yaml<T>(buf: Vec<u8>, target: T) -> T
    where
        T: serde::de::DeserializeOwned,
{
    let res: T = serde_yaml::from_slice(buf.as_slice()).unwrap();
    res
}
