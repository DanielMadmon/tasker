use std::{thread, time::Duration};
use cryptex::linux::LinuxOsKeyRing;
use sysinfo::{System, SystemExt, ProcessExt};
use themis::keys::SymmetricKey;
use crate::enums::ErrorType;



fn set_key_linux<'a>(key: &[u8]) {
    let key_ring = 
    <LinuxOsKeyRing<'a> as cryptex::NewKeyRing>::new("tasker_service");
    let is_root = crate::config::is_root();
    let mut id = String::from("tasker_pass");
    if is_root{
        id = "tasker_pass_root".to_string();
    }
    match key_ring {
        Ok(mut new_ring) => {
            let conn = <LinuxOsKeyRing<'a> as cryptex::DynKeyRing>::set_secret(
                &mut new_ring,
                &id,
                key,
            );
            if let Ok(_res) = conn {
            }
        }
        Err(_) => {
           ErrorType::error(&ErrorType::KeyringError);
        }
    }
}
fn is_wallet_running(){
    loop{
        let mut is_running: bool = false;
        let system: System = System::new_all();
        for (_,procces) in system.processes(){
        if procces.name() == "kwalletd5"{
           is_running = true;
           break;
        }
    }
    if is_running{
        break;
    }
    thread::sleep(Duration::from_secs(30));
    }
}
fn get_key_linux_internal<'b>() -> Option<Vec<u8>> {
    is_wallet_running();
    let key_ring = 
    <LinuxOsKeyRing<'b> as cryptex::NewKeyRing>::new("tasker_service");
    match key_ring {
        Ok(mut new_ring) => {
            let key_ring =
             <LinuxOsKeyRing<'b> as cryptex::DynKeyRing>::get_secret(
                &mut new_ring,
                "tasker_pass",
            );
            if let Ok(secret) = key_ring.clone() {
                let secret = secret.0.clone();
                return Some(secret);
            } else {
                ErrorType::error(&ErrorType::KeyringError);
                None
            }
        }
        Err(_) => {
            ErrorType::error(&ErrorType::KeyringError);
            None
        }
    }
}
pub fn get_key_linux() -> Vec<u8> {
    let key = get_key_linux_internal();
    //TODO: Add root detection in config with encryption.   
    if key.is_none() {
        let symm_key = SymmetricKey::new();
        let key = symm_key.as_ref();
        set_key_linux(key);
        //add new key
        let key = get_key_linux_internal();
     if let Some(new_key) = key {
            new_key
        }else {
            ErrorType::error(&ErrorType::KeyringError);
            panic!("can't get key for database encryption")
        }
    } else if key.is_some() {
        key.unwrap()
    } else {
        ErrorType::error(&ErrorType::KeyringError);
        panic!("can't get key for database encryption");
    }
}
