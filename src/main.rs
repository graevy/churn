use std::process::Command;

// program order
// 1. get package name
// 2. install package
// 3. run command
// 4. log
// 5. uninstall package


mod pacman;
use churn::Pkgmgr;
fn main() {
    let mut args = std::env::args();

    // in e.g. churn nmap 192.168.1.1/24,
    // the package manager has to find the package containing nmap
    // nmap is the "supplied command"
    let supplied_cmd = args.nth(1).unwrap();

    let pkg_mgr = pacman::Pacman;

    let pkg_name = String::from(
        pkg_mgr
        .get_pkg_name(&supplied_cmd)
        .lines()
        .nth(0)
        .unwrap()
    );

    pkg_mgr.install(&pkg_name);

    // execute nested cmd
    Command::new(supplied_cmd)
        .args(&args.collect::<Vec<String>>());

    pkg_mgr.uninstall(&pkg_name);
    
    print!("{}", pkg_name);
}
