use std::path::Path;

pub fn is_unix() -> bool {
    return cfg!(unix);
}

pub fn is_win() -> bool {
    return cfg!(windows);
}

pub fn has_binary(binary:&str) -> bool{
    if is_unix() {
        let binary_path:[&str;2] = ["/usr/bin", "/bin"];
        for s in binary_path {
            if Path::new(&(s.to_owned() + "/" + binary)).exists() {
                return true;
            }
        }
    }
    return false;
}