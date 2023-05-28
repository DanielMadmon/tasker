//! this module implements Conn enum, to avoid rewriting all this lines of code whenever we want to execute 
//! an sqlite query

use crate::{config, enums::Conn};
use rusqlite::Connection;

impl Conn<Connection, rusqlite::Error> {
    fn create_conn() -> Result<Connection, rusqlite::Error> {
        let path = config::set_db_path();
        Connection::open(path)
    }
    fn get_conn_option_result() -> Conn<Connection, rusqlite::Error> {
        match Self::create_conn() {
            Ok(conn_db_ok) => Conn::Conn(conn_db_ok),
            Err(db_not_exist) => Conn::Error(db_not_exist),
        }
    }
    pub fn get_conn_res() -> Conn<Connection, rusqlite::Error> {
        Self::get_conn_option_result()
    }
}
