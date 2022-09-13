pub mod utils;

pub enum Transaction{
    NotImplemented,
    Success,
    PakagesNotFound(Vec<String>),
    TimeOut,
    RepoAlreadyExists
}

pub enum SourceHint {
    Zypper,
    Apt,
    Dnf,
    Pacman,
    Flatpak,
    Snapcraft,
    AppImage,
    Yum,
    Aur,
    None
}

pub fn add_repo(url:&str, name:&str, source:SourceHint) -> Transaction{
    return Transaction::NotImplemented;
}

/*
Installs a set of packages
*/
pub fn install(packages:Vec<String>, source:SourceHint) -> Transaction{
    return Transaction::NotImplemented;
}

/*
Removes a given set of packages.
*/
pub fn uninstall(packages:Vec<String>, source:SourceHint) -> Transaction{
    return Transaction::NotImplemented;
}

/*
Updates the package cache
*/
pub fn refresh() -> Transaction{
    return Transaction::NotImplemented;
}

/*
Performs a system update (not a full upgrade to a new release of the distribution)
*/
pub fn upgrade() -> Transaction{
    return Transaction::NotImplemented;
}

/*
Updates the system to the next available major release.
 */
pub fn distribution_upgrade() -> Transaction{
    return Transaction::NotImplemented;
}

#[cfg(test)]
mod tests {
    use crate::utils::has_binary;

    use super::*;

    #[test]
    fn it_works() {
        let mut packages_install:Vec<String> = Vec::new();
        packages_install.push("value".to_string());

        let mut packages_uninstall:Vec<String> = Vec::new();
        packages_uninstall.push("value".to_string());

        assert!(matches!(install(packages_install, SourceHint::None), Transaction::NotImplemented));
        assert!(matches!(uninstall(packages_uninstall, SourceHint::None), Transaction::NotImplemented));
        assert!(matches!(upgrade(), Transaction::NotImplemented));
        assert!(matches!(refresh(), Transaction::NotImplemented));
        assert!(matches!(distribution_upgrade(), Transaction::NotImplemented));
        assert!(matches!(has_binary("su"), true));
    }
}
