use crate::enums::ErrorType;
use crate::enums::ErrorType::{ConnectionError, RusqliteError};

impl ErrorType {
    pub fn error(&self) {
        match &self {
            ConnectionError => {
                println!(
                    "error connectiong to the database, please ensure read/write permissions."
                );
            }
            RusqliteError => {
                println!("error, writing or reading in the database, please ensure read/write permissions");
            }
            ErrorType::EncryptionError => {
                eprintln!("error encrypting task/log!");
            }
            ErrorType::EmptyTable => {}
            ErrorType::DecryptionError=>{
                eprintln!("error decrypting task/log");
            }
            ErrorType::KeyringError => {
                eprintln!("error getting keyring, please check keyring installation");
            }
            ErrorType::ShellNotFound => {
                eprintln!("error executing command please check shell configuration")
            }
            ErrorType::CommandError => {
                eprintln!("error executing command please check command configuration")
            }
        }
    }
}
