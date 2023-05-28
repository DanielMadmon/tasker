//! this module is responisble for configuring the path to the tasks database and creating it if it does not exist.
//! right now OS enum is being used only for linux but is left here for future use in other platforms as well.
//TODO:better permissions control
//TODO:make compatible with mac & windoes

use std::fs::{self, OpenOptions};
use crate::enums::OS;
use simple_home_dir::home_dir;

impl OS {
    pub fn get_os() -> OS {
        if cfg!(target_os = "windows") {
            return OS::Windows;
        }
        if cfg!(target_os = "linux") {
            return OS::Linux;
        }
        if cfg!(target_os = "macos") {
            OS::Mac
        } else {
            OS::Unknown
        }
    }
}
pub fn set_db_path() -> String {
    if !is_root(){
        let home_dir_path = home_dir();
        if OS::get_os() == OS::Linux {
        if let Some(mut home) = home_dir_path {
            home.push(".config/tasker/");
            let _res_create = fs::create_dir_all(home.clone());
            home.push("tasker.db");
            let conf_path_string: String = home.to_str().unwrap().to_string();
            return conf_path_string;
        }
    }
    }else if is_root(){
        if OS::get_os() == OS::Linux {
                let _res = fs::create_dir_all("/etc/tasker/");
                crate::root_key_request::mod_path("/etc/tasker");
                let file = 
                OpenOptions::new().read(true).create(true).open("/etc/tasker/tasker.db");
                if let Ok(mut file_new) = file{
                    crate::root_key_request::mod_file(& mut file_new);
                }
                return "/etc/tasker/tasker.db".to_string();
                
        }
    }
    

    //if all else failes just create database in parent dir
    "./databases/tasker.db".to_string()
}
pub fn is_root()->bool{
    let id: sudo::RunningAs = sudo::check();
    if id == sudo::RunningAs::Root{
        return true;
    }
    false
}

