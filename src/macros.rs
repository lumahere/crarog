#[macro_export]
macro_rules! cpmcfg_avail {
    () => {
        std::fs::read_dir(".")
        .unwrap()
        .find(|x| x.as_ref().unwrap().file_name() == "cpmcfg.json")
        .unwrap_or_else(|| panic!("didn't find cpm.cfg"))
        .unwrap();

    };
}