use std::path::Path;
use std::env;

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

pub fn find_sudo_tool() -> String {
    let headless_su_tool = ["su", "sudo"];
    let headed_su_tools = ["pkexec", "xdg-su", "kdesu", "gnome-su"];

    if os_is_headless() {
        for s in headless_su_tool {
            if has_binary(s) {
                return s.to_string();
            }
        }
    } else {
        for s in headed_su_tools {
            if has_binary(s) {
                return s.to_string();
            }
        }
    }

    return "".to_string();
}

pub fn is_os_headless() -> bool {
    if is_unix() {
        let xdg_session_key = "XDG_SESSION_TYPE";
        let display_servers = ["x11", "wayland"];
        match env::var(xdg_session_key) {
            Ok(val) => {
                for s in display_servers {
                    if s.to_lowercase() == val.to_lowercase() {
                        return false;
                    }
                }
                if val == "tty" {
                    return true;
                }
            },
            Err(e) => {
                println!("couldn't interpret {}: {}", xdg_session_key, e)
            }
        }
    } else {
        //Headless Winblows, good joke
        //This isn't a thing right?
        return false;
    }
    return true;
}