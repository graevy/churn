pub trait Pkgmgr {
    const NAME: &'static str;
    fn get_pkg_name(&self, supplied_cmd: &String) -> String;
    fn install(&self, pkg: &String) -> std::process::Output;
    fn uninstall(&self, pkg: &String) -> std::process::Output;
}
