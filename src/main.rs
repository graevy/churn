use std::process::{Command, Stdio, ExitStatus};

// program order
// 1. get package name
// 2. install package
// 3. run command
// 4. uninstall package

trait Pkgmgr {
    const NAME: &'static str;
    fn get_pkg_name(&self, supplied_cmd: &String) -> String;
    fn install(&self, package_containing_command: &String) -> std::process::Output;
    fn uninstall(&self, package_containing_command: &String) -> std::process::Output;
}

struct Pacman;
impl Pkgmgr for Pacman {
    const NAME: &'static str = "pacman";

    fn get_pkg_name(&self, supplied_cmd: &String) -> String {
        let pkg_name = Command::new("pkgfile")
            // -b filters pkgfile results for binaries
            .arg("-b")
            .arg(supplied_cmd)
            .output()
            .unwrap()
            .stdout;
            
        String::from_utf8(pkg_name)
        .unwrap()
    }

    fn install(&self, pkg_name: &String) -> std::process::Output {
        Command::new(Self::NAME)
            .args(["--sync", "--noconfirm", &pkg_name])
            .output()
            .unwrap()
    }
    fn uninstall(&self, pkg_name: &String) -> std::process::Output {
        Command::new(Self::NAME)
            .args(["--remove", "--noconfirm", &pkg_name])
            .output()
            .unwrap()
    }
}


fn main() {
    let mut args = std::env::args();

    // in e.g. churn nmap 192.168.1.1/24, we have to find the package containing nmap
    // each package manager should have a corresponding solution
    let supplied_cmd = args.nth(1).unwrap();

    let pkg_mgr = Pacman;
    let pkgfile_stdout = pkg_mgr.get_pkg_name(&supplied_cmd);

    let pkg_name = String::from(
        pkgfile_stdout.lines().nth(0).unwrap()
    );

    pkg_mgr.install(&pkg_name);

    // execute nested cmd
    Command::new(supplied_cmd)
        .args(&args.collect::<Vec<String>>());

    pkg_mgr.uninstall(&pkg_name);
    
    print!("{}", pkg_name);
}
