use color_print::*;
use std::{ io::Write};



use crate::{cpmcfg_avail, parsing::Config};

static JSONFILE: &str = r#"{
    "project": {
      "name": "thing",
      "version": "0.0.1"
    },
    "dependencies": [
      "./src/main.c"
    ],
    "compile": {
      "flags": [
        "-O2",
        "-Wall"
      ],
      "cache": true,
      "exec": "/bin/gcc",
      "jobs": 1
    },
    "link": {
      "flags": [],
      "ltype": "bin",
      "exec": "/bin/gcc"
    }
  }
"#;

static CFILE: &str = r#"#include <stdio.h>
    
int main(void){
    printf("hello, world!\n");
    return 0;
}
"#;

pub fn run(verbose:bool){
    crate::buildlogic::build(verbose);
    let config = Config::from_str(&std::fs::read_to_string("./cpmcfg.json").unwrap());
    let output = std::process::Command::new(format!("./target/{}", config.project.name)).output().unwrap();
    if verbose{
    cprintln!("<magenta,bold>Forwarding stdout from runnable program");
    cprintln!("<magenta,bold>Entered program space\n------------------------");
    }

    cprintln!("<white>{}", String::from_utf8(output.stdout).unwrap());
    if verbose {
        cprintln!("<magenta,bold>program exited with status: {};\nstderr: {}", output.status, String::from_utf8(output.stderr).unwrap())
    }
}

pub fn clean(){
    cpmcfg_avail!();
    match std::fs::remove_dir_all("./target"){
        Ok(_) => {
            cprintln!("<red,bold>Removing</><white,bold> ./target")
        },
        Err(_) => (),
    }
}


pub fn info() {
    std::fs::read_dir(".")
        .unwrap()
        .find(|x| x.as_ref().unwrap().file_name() == "cpmcfg.json")
        .unwrap_or_else(|| panic!("didn't find cpm.cfg"))
        .unwrap();
    let file = std::fs::read_to_string("cpmcfg.json").unwrap();
    let config = Config::from_str(&file);
    cprintln!("<blue, bold>Project info</>");
    cprintln!("<red, bold>  |- name: {}</>", config.project.name);
    cprintln!("<red, bold>  |- version: {}</>", config.project.version);
    cprintln!("<red, bold>  |- </><blue, bold>Dependencies</>");
    for i in &config.dependencies {
        cprintln!(
            "<blue,bold>  |    |- {}</>",
            i.to_str().unwrap(),
        );
    }
    cprintln!("<green,bold>  |- Compiler</>");
    cprintln!(
        "<green,bold>  |       |- flags: [{}]</>",
        config.compile.flags.join(", ")
    );
    cprintln!("<green,bold>  |       |- cache: {}", config.compile.cache);
    cprintln!(
        "<green,bold>  |       |- exec: {}",
        config.compile.exec.to_str().unwrap()
    );
    cprintln!("<green,bold>  |       |- jobs: {}", config.compile.jobs);
    cprintln!("<green,bold>  |");
    cprintln!("<magenta, bold>  |- Linker</>");
    cprintln!(
        "<magenta, bold>  |       |- flags: [{}]</>",
        config.link.flags.join(", ")
    );
    cprintln!("<magenta, bold>  |       |- type: {}</>", config.link.ltype);
    cprintln!(
        "<magenta, bold>  |       |- exec: {}</>",
        config.link.exec.to_str().unwrap()
    );
}

pub fn new(path: Option<std::path::PathBuf>) {
    match path {
        None => {
            std::fs::DirBuilder::new().create("./src").unwrap_or(());
            match std::fs::File::create_new("./src/main.c") {
                Ok(mut file) => {
                    file.write_all(CFILE.as_bytes()).unwrap();
                }
                Err(_) => (),
            }
            match std::fs::File::create_new("./cpmcfg.json") {
                Ok(mut file) => {
                    file.write_all(JSONFILE.as_bytes()).unwrap();
                }
                Err(_) => (),
            }
        }
        Some(pathc) => {
            let cfgpath =
                std::path::PathBuf::from(format!("{}/cpmcfg.json", pathc.to_str().unwrap()));
            let cpath = std::path::PathBuf::from(format!("{}/src/main.c", pathc.to_str().unwrap()));
            let srcdir = std::path::PathBuf::from(format!("{}/src", pathc.to_str().unwrap()));

            std::fs::DirBuilder::new()
                .recursive(true)
                .create(srcdir)
                .unwrap();
            match std::fs::File::create_new(cpath) {
                Ok(mut file) => {
                    file.write_all(CFILE.as_bytes()).unwrap();
                }
                Err(_) => (),
            }
            match std::fs::File::create_new(cfgpath) {
                Ok(mut file) => {
                    file.write_all(JSONFILE.as_bytes()).unwrap();
                }
                Err(_) => (),
            }
        }
    }
}

pub fn add(glob:bool){
    cpmcfg_avail!();
    let config = Config::from_str(&std::fs::read_to_string("./cpmcfg.json").unwrap());
    if glob {

    } else {
        
    }
}
