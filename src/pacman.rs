use std::process::Command;

pub struct Pacman;
impl churn::Pkgmgr for Pacman {
    const NAME: &'static str = "pacman";

    // 4th rewrite. lots of lines for "pkgfile -b $pkg | head -n 1"
    fn get_pkg_name(&self, supplied_cmd: &String) -> String {
        String::from_utf8( Command::new("pkgfile")
            // -b filters pkgfile results for binaries
            // in my experience, this usually filters the
            // bash completion files that "pacman -Fq" doesn't catch
            // pkgfile also seems to be faster. space/time tradeoff?
            // inevitably some edge case will ruin this
            .args(["-b", supplied_cmd])
            .output()
            .unwrap()
            .stdout
            .split(|i| *i == '\n' as u8)
            .nth(0)
            .unwrap()
            .to_owned()
            )
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