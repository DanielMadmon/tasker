use std::fs;
use std::io::Write;
use std::path::Path;
use readable_perms::{FChmodExt,ChmodExt,Permissions};
use themis::keys::SymmetricKey;


pub fn get_key_root() -> Vec<u8>{
    let res_get: Option<Vec<u8>> = get_stored_key();
    if let Some(key) = res_get{
        return key;
    }if res_get.is_none(){
        set_key();
        let get_key: Vec<u8> = get_stored_key().expect("error! please check writing permissions for /etc/");
        return get_key;
    }else{
        panic!("error! please check writing permissions for /etc/")
    }
}

fn get_stored_key()->Option<Vec<u8>> {
    let path_key_storage = Path::new("/etc/tasker/.data/tasker.key");
    let key_file: Result<Vec<u8>, std::io::Error> = fs::read(path_key_storage);

    match key_file{
        Ok(key_enc) => {
            return Some(key_enc);
        }
        Err(_) => {
            None
        }
    }
}
fn set_key(){
    //generate new 256 bit long symmetric key
    let key = SymmetricKey::new();
        
    let mut path_key_storage: &Path = Path::new("/etc/tasker/.data/");
    let _dir_create = std::fs::create_dir_all(path_key_storage);
    let path_key_storage_parent: &Path = Path::new("/etc/tasker/");

    //change ownership for folder and parent folder
    mod_path(path_key_storage_parent);
    mod_path(path_key_storage);

    path_key_storage = Path::new("/etc/tasker/.data/tasker.key");

    let file: Result<std::fs::File, std::io::Error> = std::fs::File::create(path_key_storage);

    if let Ok(mut key_file) = file {
        //change ownership of file
        mod_file(& mut key_file);

       key_file.write_all(&key.as_ref())
       .expect("failed to set new password please check permission for /etc/ folder");
    }else if file.is_err(){
        panic!("failed to set new password please check permission for /etc/ folder");
    }
}

pub (crate) fn mod_file(file: &mut std::fs::File)
{
	file.chmod(Permissions::from_mask(0o600))
    .expect("fatal error trying to change key file permissions")
}
pub (crate) fn mod_path<P: AsRef<Path>>(path: P)
{
	path.chmod(Permissions::from_mask(0o600))
    .expect("fatal error trying to change key file permissions");
}