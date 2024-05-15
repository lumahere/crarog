use std::fs::{self, read_dir};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;

use color_print::cprintln;

use crate::parsing::Config;
use crate::cpmcfg_avail;

///checks if there are c files in the ./src dir but isn't in cpmcfg's dependencies and vice-versa
fn pre_build(mut conf:Config) -> Config{
    let mut unlisted = vec![];
    let mut removed = vec![];
    let listed = &conf.dependencies.iter().map(|x| x.to_str().unwrap()).collect::<Vec<_>>();
    let dir = read_dir("./src").unwrap().map(|x| x.unwrap()).collect::<Vec<_>>();
        for f in &dir{
                if listed.iter().find(|x| **x == f.path().to_str().unwrap()) == None{
                    if f.path().extension().unwrap() == "c"{
                    unlisted.push(f.path());
                }
            }
    }
    let files = dir.iter().map(|x|x.path()).collect::<Vec<_>>();
    for i in listed{
        if files.iter().find(|x| &x.to_str().unwrap() == i) == None{
            removed.push(PathBuf::from(*i));
        };
    }

    conf.dependencies.extend(unlisted);
    for i in removed{
        let index = conf.dependencies.iter().position(|x|*x == i).unwrap();
        conf.dependencies.remove(index);
    }  

    let serialized = serde_json::to_string_pretty(&conf).unwrap();
    let mut file = fs::File::options().write(true).truncate(true).open("./cpmcfg.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    return conf;
}

fn sync_compile(config:&Config, verbose:bool){
    for i in &config.dependencies {
        
    let mut out_path = std::path::PathBuf::from("./target/build");
        let out_name = format!("{}.o", &i.file_name().unwrap().to_str().unwrap());
        out_path.push(out_name);
        let process = std::process::Command::new(&config.compile.exec)
            .args(&config.compile.flags)
            .arg("-c")
            .arg(&i)
            .arg("-o")
            .arg(&out_path)
            .output()
            .unwrap_or_else(|x| {
                cprintln!(
                    "<red, bold>ERROR: cannot start process: {}; Error: {}",
                    config.compile.exec.to_str().unwrap(),
                    x
                );
                exit(-1)
            });
        cprintln!("<g,bold> Compiling</> <white,bold>{}", i.to_str().unwrap());
    
    if verbose{
        cprintln!("<magenta> target -> {}", &out_path.to_str().unwrap());
        cprintln!("<white>{}", String::from_utf8(process.stdout).unwrap());
    }
    if process.stderr.len() != 0{
        cprintln!("<red, bold>Compile Error at compiling: {}", &i.to_str().unwrap());
        cprintln!("<red,bold>{}", String::from_utf8(process.stderr).unwrap_or("unable to parse stderr".to_string()));
        exit(1);
    }
}
}

fn link(conf:&Config)
{
    let target_files = read_dir("./target/build").unwrap();
    let mut target_f = vec![];
    for i in target_files{
        if let Ok(entr) = i{
            target_f.push(entr.path());
        }
    }
    let process = std::process::Command::new(&conf.link.exec).args(target_f).arg("-o").arg(format!("./target/{}", conf.project.name)).output().unwrap();
    if process.stderr.len() != 0 {
        cprintln!("<red,bold>LINKER ERROR\n{}", String::from_utf8(process.stderr).unwrap_or("unable to parse stderr".to_string()));
    }
}
pub fn build(verbose: bool) {
    cpmcfg_avail!();
    let file = std::fs::read_to_string("cpmcfg.json").unwrap();
    let config = Config::from_str(&file);
    let config = pre_build(config);
    
    std::fs::DirBuilder::new()
        .recursive(true)
        .create("./target/build")
        .unwrap();
    if config.compile.jobs < 2{
        sync_compile(&config, verbose);
    }
    link(&config);
    cprintln!("<blue, bold> Building finished!");
}