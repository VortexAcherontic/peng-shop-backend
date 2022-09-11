/*
Installs a set of packages
*/
pub fn install(packages:Vec<String>) -> bool{
    println!("Not implemented yet!");
    return true;
}

/*
Removes a given set of packages.
*/
pub fn uninstall(packages:Vec<String>) -> bool{
    println!("Not implemented yet!");
    return true;
}

/*
Updates the package cache
*/
pub fn refresh() -> bool{
    println!("Not implemeted yet!");
    return true;
}

/*
Performs a system update (not a full upgrade to a new release of the distribution)
*/
pub fn upgrade() -> bool{
    println!("Not implemeted yet!");
    return true;
}

/*
Updates the system to the next available major release.
 */
pub fn distribution_upgrade() -> bool{
    println!("Not implemeted yet!");
    return true;
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

        assert_eq!(install(packages_install), true);
        assert_eq!(uninstall(packages_uninstall), true);
        assert_eq!(upgrade(), true);
        assert_eq!(refresh(), true);
        assert_eq!(distribution_upgrade(), true);
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
