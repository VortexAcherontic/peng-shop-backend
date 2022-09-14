pub mod utils;

#[cfg(target_feature = "appimage")]
use peng_shop_backend_appimage;
#[cfg(target_feature = "aur")]
use peng_shop_backend_aur;
#[cfg(target_feature = "zypp")]
use peng_shop_backend_zypper;
#[cfg(target_feature = "apt")]
use peng_shop_backend_apt;
#[cfg(target_feature = "snap")]
use peng_shop_backend_snap;
#[cfg(target_feature = "dnf")]
use peng_shop_backend_dnf;
#[cfg(target_feature = "yum")]
use peng_shop_backend_yum;
#[cfg(target_feature = "pacman")]
use peng_shop_backend_pacman;
#[cfg(target_feature = "flatpak")]
use peng_shop_backend_flatpak;
#[cfg(target_feature = "pkgkit")]
use peng_shop_backend_packagekit;


pub enum Transaction{
    NotImplemented,
    Success,
    PakagesNotFound(Vec<String>),
    TimeOut,
    RepoAlreadyExists,
    FeatureDisabled,
    InvalidURI
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
    Pkgkit,
    None
}

pub fn add_repo(url:&str, name:&str, source:SourceHint) -> Transaction{
    match source {
        SourceHint::AppImage => {
            if cfg!(target_feature = "appimage") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Apt => {
            if cfg!(target_feature = "apt") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Aur => {
            if cfg!(target_feature = "aur") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Dnf => {
            if cfg!(target_feature = "dnf") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Flatpak => {
            if cfg!(target_feature = "flatpak") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Pacman => {
            if cfg!(target_feature = "pacman") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Snapcraft => {
            if cfg!(target_feature = "snap") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Yum => {
            if cfg!(target_feature = "yum") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::Zypper => {
            if cfg!(target_feature = "zypp") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        },
        SourceHint::None => {
            return Transaction::FeatureDisabled;
        },
        SourceHint::Pkgkit => {
            if cfg!(target_feature = "pkgkit") {
                return Transaction::NotImplemented;
            } else {
                return Transaction::FeatureDisabled;
            }
        }
    }
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
        assert!(matches!(add_repo("https://repo.com", "TestRepo", SourceHint::None), Transaction::FeatureDisabled));
        assert!(matches!(distribution_upgrade(), Transaction::NotImplemented));
        assert!(matches!(has_binary("su"), true));
    }

    #[test]
    #[cfg_attr(not(feature = "zypp"), ignore)]
    fn test_zypper(){
        let mut packages_install:Vec<String> = Vec::new();
        packages_install.push("peng-shop".to_string());

        let mut packages_uninstall:Vec<String> = Vec::new();
        packages_uninstall.push("peng-shop".to_string());

        assert!(matches!(install(packages_install, SourceHint::Zypper), Transaction::NotImplemented));
    }
}
