use std::process::Command;

// program order
// 1. get package name
// 2. install package
// 3. run command
// 4. log
// 5. uninstall package


// struct Pacman;
// impl Pkgmgr for Pacman {
//     const NAME: &'static str = "pacman";

//     // 4th rewrite. lots of lines for "pkgfile -b $pkg | head -n 1"
//     fn get_pkg_name(&self, supplied_cmd: &String) -> String {
//         String::from_utf8( Command::new("pkgfile")
//             // -b filters pkgfile results for binaries
//             // in my experience, this usually filters the
//             // bash completion files that "pacman -Fq" doesn't catch
//             // pkgfile also seems to be faster. space/time tradeoff?
//             // inevitably some edge case will ruin this
//             .args(["-b", supplied_cmd])
//             .output()
//             .unwrap()
//             .stdout
//             .split(|i| *i == '\n' as u8)
//             .nth(0)
//             .unwrap()
//             .to_owned()
//             )
//             .unwrap()
        
//     }

//     fn install(&self, pkg_name: &String) -> std::process::Output {
//         Command::new(Self::NAME)
//             .args(["--sync", "--noconfirm", &pkg_name])
//             .output()
//             .unwrap()
//     }
//     fn uninstall(&self, pkg_name: &String) -> std::process::Output {
//         Command::new(Self::NAME)
//             .args(["--remove", "--noconfirm", &pkg_name])
//             .output()
//             .unwrap()
//     }
// }

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
