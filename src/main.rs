#![allow(dead_code, unused_imports)]
use dirs::{self, home_dir};
use serde::Deserialize;
// use std::fs;
use std::fs::{self, read_dir, DirEntry, File, FileType, ReadDir};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct ConfigFile {
    sources: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DootFile {
    ignore: Option<Vec<String>>,
    config: Vec<DootConfig>
}

#[derive(Debug, Deserialize)]
struct DootConfig {
    recursive: bool,x
    src: String,
    target: String,
}


fn main() {
    let conf_file = fs::read_to_string("/home/connor/.config/doot/config.toml")
        .expect("Failed to get config.toml file");
    let conf: ConfigFile = toml::from_str(&conf_file).expect("Failed to deserialize");
    println!("conf sources: {:?}", conf.sources);
    // checks that the sources exist.
    let sources: Vec<String> = conf
        .sources
        .iter()
        .filter(|src: &&String| Path::exists(Path::new(src)))
        .map(|str| {str.to_owned()})
        .collect();
    println!("sources: {:?}", sources);

    for source in sources.iter() {
        let mut doot_filepath = PathBuf::from(source);
        doot_filepath.push("doot.toml");

        if Path::exists(doot_filepath.as_path()) {
            println!("found doot.toml @ {:?}", doot_filepath);
            let dootfile = fs::read_to_string(doot_filepath).unwrap();
            let dootConfig: DootFile = toml::from_str(&dootfile).unwrap();
            println!("dootconf: {:?}", dootConfig);
        }
    }

    // let mut dir_path = home_dir().unwrap();
    // dir_path.push(".config");
    // let dir = read_dir(dir_path).unwrap();
    // println!("dir: {:?}", dir);
    // // let files: Vec<_> = dir.collect();
    // dir.for_each(|file| {
    //     if let Ok(de) = file {
    //         println!("{:?} | {:?}", de.file_name(), de.file_type().unwrap())
    //     };
    // });
    // // println!("files: {:?}", files);
    // println!("Hello, world!");
}
