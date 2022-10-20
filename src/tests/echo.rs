use std::process::Command;

// substitute echo for simple interactive unit testing?
// code is copied, but all cmd flags are stripped (args methods -> arg methods)
pub struct Echo;
impl churn::Pkgmgr for Echo {
    const NAME: &'static str = "echo";

    fn get_pkg_name(&self, supplied_cmd: &String) -> String {
        String::from_utf8( Command::new("pkgfile")
            .arg(supplied_cmd)
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
            .arg(&pkg_name)
            .output()
            .unwrap()
    }

    fn uninstall(&self, pkg_name: &String) -> std::process::Output {
        Command::new(Self::NAME)
            .arg(&pkg_name)
            .output()
            .unwrap()
    }
}