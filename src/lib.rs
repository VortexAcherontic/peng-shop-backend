pub enum Transaction{
    NotImplemented,
    Success,
    NotFound(Vec<String>),
    TimeOut
}

pub fn add_repo(url:String, name:String) -> Transaction{
    return Transaction::NotImplemented;
}

/*
Installs a set of packages
*/
pub fn install(packages:Vec<String>) -> Transaction{
    return Transaction::NotImplemented;
}

/*
Removes a given set of packages.
*/
pub fn uninstall(packages:Vec<String>) -> Transaction{
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

fn has_binary(binary:&str) -> bool{
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut packages_install:Vec<String> = Vec::new();
        packages_install.push("value".to_string());

        let mut packages_uninstall:Vec<String> = Vec::new();
        packages_uninstall.push("value".to_string());

        assert_eq!(install(packages_install), Transaction::NotImplemented);
        assert_eq!(uninstall(packages_uninstall), Transaction::NotImplemented);
        assert_eq!(upgrade(), Transaction::NotImplemented);
        assert_eq!(refresh(), Transaction::NotImplemented);
        assert_eq!(distribution_upgrade(), Transaction::NotImplemented);
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
