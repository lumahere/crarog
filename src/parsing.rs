use std::collections::HashMap;

use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Config{
    pub project: Project,
    pub dependencies:  HashMap<String, Dependencies>,
    pub compile: Compilation,
    pub link: Linker
}
#[derive(Debug, Default, Deserialize)]
pub struct Project{
    pub name: String,
    pub version:String

}
#[derive(Debug, Default, Deserialize)]
pub struct Compilation{
    pub flags:Vec<String>,
    pub cache:bool,
    pub exec:std::path::PathBuf,
    pub jobs:u8
    
}
#[derive(Debug, Default, Deserialize)]
pub struct Linker{
    pub flags:Vec<String>,
    pub ltype:String,
    pub exec:std::path::PathBuf,
}

#[derive(Debug, Default, Deserialize)]
pub struct Dependencies{
    pub path: std::path::PathBuf,
    pub property: Option<Vec<String>>
}



impl Config{
    pub fn from_str(s:&str) -> Self{
        let cfg = serde_json::from_str::<Config>(s).unwrap();
        cfg
    }
}