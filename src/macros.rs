#[macro_export]
macro_rules! cpmcfg_avail {
    () => {
        std::fs::read_dir(".")
        .unwrap()
        .find(|x| x.as_ref().unwrap().file_name() == "cpmcfg.json")
        .unwrap_or_else(|| {
            cprintln!("<red,bold> ERROR: Unable to find cpmcfg.json in current directory");
            std::process::exit(1);
        })
        .unwrap();

    };
}