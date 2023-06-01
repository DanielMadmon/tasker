use std::sync::{Arc, Mutex};

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    enums::{self, Conn, ErrorType, TableType},
    structs::{self, LogEntry, LogEntryEncrypted},
};

impl structs::LogEntryEncrypted {
    pub fn open_db() {
        let conn_enum = <LogEntryEncrypted as enums::ConnDb>::conn_enum();
        match conn_enum {
            enums::Conn::Conn(conn) => {
                let res_create_log_table = conn.execute(
                    "
                CREATE TABLE IF NOT EXISTS logs(
                    name BLOB,
                    command BLOB,
                    output BLOB,
                    execution_year BLOB,
                    execution_month BLOB,
                    execution_week BLOB,
                    execution_day_of_month BLOB,
                    execution_day_of_week BLOB,
                    execution_hour BLOB,
                    execution_minute BLOB
                )
                ",
                    (),
                );
                match res_create_log_table {
                    Ok(_ok) => {}
                    Err(_error) => {
                        ErrorType::error(&ErrorType::RusqliteError);
                    }
                }
            }
            enums::Conn::Error(_err) => {
                ErrorType::error(&ErrorType::ConnectionError);
            }
        }
    }
    fn add_log_internal(&self) -> bool {
        let conn_enum = <LogEntryEncrypted as enums::ConnDb>::conn_enum();
        match conn_enum {
            Conn::Conn(conn) => {
                let res_conn_prepare = conn.prepare(
                    "
                    INSERT INTO logs
                    (
                    name,
                    command,
                    output,
                    execution_year,
                    execution_month,
                    execution_week,
                    execution_day_of_month,
                    execution_day_of_week,
                    execution_hour,
                    execution_minute
                    )
                    VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
                ",
                );
                match res_conn_prepare {
                    Ok(mut conn) => {
                        let res_insert = conn.execute(rusqlite::params![
                            &self.name,
                            &self.command,
                            &self.output,
                            &self.execution_year,
                            &self.execution_month,
                            &self.execution_week,
                            &self.execution_day_of_month,
                            &self.execution_day_of_week,
                            &self.execution_hour,
                            &self.execution_minute
                        ]);
                        match res_insert {
                            Ok(_ok) => true,
                            Err(_error) => {
                                ErrorType::error(&ErrorType::RusqliteError);
                                false
                            }
                        }
                    }
                    Err(_error) => {
                        ErrorType::error(&ErrorType::RusqliteError);
                        false
                    }
                }
            }
            Conn::Error(_) => {
                ErrorType::error(&ErrorType::ConnectionError);
                false
            }
        }
    }

    //TODO:fix it's adding duplicate logs!! fix same way you did in tasks.
    pub fn add_log(new_row: LogEntry, key: Vec<u8>) -> bool {
        Self::open_db();
        let log = <LogEntryEncrypted as enums::NewRowTableBuilder>::is_duplicate(
            enums::RowType::LogRow(new_row),
            key.clone(),
        );
        let conn_enum = <LogEntryEncrypted as enums::ConnDb>::conn_enum();
        Self::clear_log_db(conn_enum);
        Self::open_db();
        let key = Arc::new(Mutex::new(key.clone()));
        let success = Arc::new(Mutex::new(false));
        match log {
            table => match table {
                TableType::TaskTable(_) => {}
                TableType::LogTable(mut vec_log) => {
                    let vec_iter = vec_log.par_iter_mut().enumerate();
                    vec_iter.for_each(|row: (usize, &mut LogEntry)| {
                        let res_enc =
                            LogEntry::encrypt_fields(row.1, key.lock().unwrap().to_owned());
                        match res_enc {
                            Some(row_enc) => {
                                let result = Self::add_log_internal(&row_enc);
                                let ret = Arc::clone(&success);
                                let ret_get = ret.lock();
                                match ret_get {
                                    Ok(mut res) => {
                                        *res = result;
                                    }
                                    Err(_) => {
                                        ErrorType::error(&ErrorType::EncryptionError);
                                    }
                                }
                            }
                            None => {
                                ErrorType::error(&ErrorType::EncryptionError);
                            }
                        }
                    });
                }
            },
        }

        let binding = Arc::clone(&success);
        let res_lock = binding.lock();
        match res_lock {
            Ok(ret) => *ret,
            Err(_err) => false,
        }
    }

    
    pub fn read_logs(key: Vec<u8>) -> Option<Vec<LogEntry>> {
        Self::open_db();
        let conn_enum = <LogEntryEncrypted as enums::ConnDb>::conn_enum();
        let mut log_table_enc: Vec<LogEntryEncrypted> = Vec::with_capacity(30);
        match conn_enum {
            Conn::Conn(statement) => {
                let res_stmt = statement.prepare(
                    "
                     SELECT
                     name,
                     command,
                     output,
                     execution_year,
                     execution_month,
                     execution_week,
                     execution_day_of_month,
                     execution_day_of_week,
                     execution_hour,
                     execution_minute
                     FROM logs
                     ",
                );
                match res_stmt {
                    Ok(mut stmt) => {
                        let rows = stmt.query([]);
                        match rows {
                            Ok(mut rows) => {
                                while let Ok(Some(row)) = rows.next() {
                                    let mut name: Vec<u8> = vec![];
                                    let mut command: Vec<u8> = vec![];
                                    let mut output: Option<Vec<u8>> = None;
                                    let mut execution_year: Vec<u8> = vec![];
                                    let mut execution_month: Vec<u8> = vec![];
                                    let mut execution_week: Vec<u8> = vec![];
                                    let mut execution_day_of_month: Vec<u8> = vec![];
                                    let mut execution_day_of_week: Vec<u8> = vec![];
                                    let mut execution_hour: Vec<u8> = vec![];
                                    let mut execution_minute: Vec<u8> = vec![];

                                    let res_name: Result<Vec<u8>, rusqlite::Error> = row.get(0);
                                    let res_command: Result<Vec<u8>, rusqlite::Error> = row.get(1);
                                    let res_output: Result<Vec<u8>, rusqlite::Error> = row.get(2);
                                    let res_execution_year: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(3);
                                    let res_execution_month: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(4);
                                    let res_execution_week: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(5);
                                    let res_execution_day_of_month: Result<
                                        Vec<u8>,
                                        rusqlite::Error,
                                    > = row.get(6);
                                    let res_execution_day_of_week: Result<
                                        Vec<u8>,
                                        rusqlite::Error,
                                    > = row.get(7);
                                    let res_execution_hour: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(8);
                                    let res_execution_minute: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(9);

                                    if let Ok(get_name) = res_name {
                                        name = get_name;
                                    }
                                    if let Ok(get_command) = res_command {
                                        command = get_command;
                                    }
                                    if let Ok(get_output) = res_output {
                                        output = Some(get_output);
                                    }
                                    if let Ok(get_execution_year) = res_execution_year {
                                        execution_year = get_execution_year;
                                    }
                                    if let Ok(get_execution_month) = res_execution_month {
                                        execution_month = get_execution_month;
                                    }
                                    if let Ok(get_execution_week) = res_execution_week {
                                        execution_week = get_execution_week;
                                    }
                                    if let Ok(get_execution_day_of_month) =
                                        res_execution_day_of_month
                                    {
                                        execution_day_of_month = get_execution_day_of_month;
                                    }
                                    if let Ok(get_execution_day_of_week) = res_execution_day_of_week
                                    {
                                        execution_day_of_week = get_execution_day_of_week;
                                    }
                                    if let Ok(get_execution_hour) = res_execution_hour {
                                        execution_hour = get_execution_hour;
                                    }
                                    if let Ok(get_execution_minute) = res_execution_minute {
                                        execution_minute = get_execution_minute;
                                    }
                                    let log_enc = LogEntryEncrypted {
                                        name,
                                        command,
                                        output,
                                        execution_year,
                                        execution_month,
                                        execution_week,
                                        execution_day_of_month,
                                        execution_day_of_week,
                                        execution_hour,
                                        execution_minute,
                                    };
                                    log_table_enc.push(log_enc);
                                }
                                let res_dec = LogEntry::decrypt_fields(log_table_enc, key);
                                match res_dec {
                                    Some(log_enc) => Some(log_enc),
                                    None => None,
                                }
                            }
                            Err(_) => {
                                ErrorType::error(&ErrorType::EmptyTable);
                                None
                            }
                        }
                    }
                    Err(_error) => {
                        ErrorType::error(&ErrorType::RusqliteError);
                        None
                    }
                }
            }
            Conn::Error(_) => {
                ErrorType::error(&ErrorType::ConnectionError);
                None
            }
        }
    }
    pub fn clear_log_db(conn_enum: Conn<rusqlite::Connection, rusqlite::Error>) -> bool {
        match conn_enum {
            Conn::Conn(conn) => {
                let res_clear_db = conn.execute(
                    "
                DELETE FROM logs",
                    (),
                );
                match res_clear_db {
                    Ok(_ok) => true,
                    Err(_error) => false,
                }
            }
            Conn::Error(_err) => {
                ErrorType::error(&ErrorType::ConnectionError);
                false
            }
        }
    }
}
