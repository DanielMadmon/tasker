use crate::enums::{Conn, ErrorType};
use crate::structs::{Task, TaskEncrypted};
use crate::enums;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

impl TaskEncrypted {
    pub fn open_db() {
        let conn_enum = <TaskEncrypted as enums::ConnDb>::conn_enum();
        match conn_enum {
            Conn::Conn(connection) => {
                let _res_create_table = connection.execute(
                    "
                CREATE TABLE IF NOT EXISTS tasks(
                name BLOB,
                shell BLOB,
                command BLOB,
                comment BLOB,
                month BLOB,
                day_of_month BLOB,
                day_of_week BLOB,
                hour BLOB,
                minute BLOB,
                year_added BLOB,
                month_added BLOB,
                day_of_month_added BLOB,
                hour_added BLOB,
                minute_added BLOB
            )",
                    (),
                );
            }
            Conn::Error(_error) => {
                ErrorType::error(&ErrorType::RusqliteError);
            }
        }
    }
    fn add_task_internal(&self) -> bool {
        let conn_enum = <TaskEncrypted as enums::ConnDb>::conn_enum();
        match conn_enum {
            Conn::Conn(conn_db_ok) => {
                let add_new = conn_db_ok.prepare(
                    "INSERT INTO tasks (
                        name,
                        shell,
                        command,
                        comment,
                        month,
                        day_of_month,
                        day_of_week,
                        hour,
                        minute,
                        year_added,
                        month_added,
                        day_of_month_added,
                        hour_added,
                        minute_added
                    )
                    VALUES(?1, ?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",
                );
                match add_new {
                    Ok(mut statement) => {
                        let statement_res = statement.execute(rusqlite::params![
                            &self.name,
                            &self.shell,
                            &self.command,
                            &self.comment,
                            &self.month,
                            &self.day_of_month,
                            &self.day_of_week,
                            &self.hour,
                            &self.minute,
                            &self.year_added,
                            &self.month_added,
                            &self.day_of_month_added,
                            &self.hour_added,
                            &self.minute_added
                        ]);
                        match statement_res {
                            Ok(_ok) => true,
                            Err(_err) => false,
                        }
                    }
                    Err(_error) => false,
                }
            }
            Conn::Error(_no_such_db) => false,
        }
    }

    pub fn add_task(new_row: Task, key: Vec<u8>) {
        Self::open_db();
        let task = <TaskEncrypted as enums::NewRowTableBuilder>::is_duplicate(
            enums::RowType::TaskRow(new_row),
            key.clone(),
        );
        Self::clear_tasks_table();
        Self::open_db();
        let key = Arc::new(Mutex::new(key.clone()));
        match task {
            enums::TableType::TaskTable(mut vec_tasks) => {
                let vec_iter = vec_tasks.par_iter_mut().enumerate();
                vec_iter.for_each(|row: (usize, &mut Task)| {
                    let res_enc = Task::encrypt_fields(row.1, key.lock().unwrap().to_owned());
                    match res_enc {
                        Some(enc_task) => {
                            Self::add_task_internal(&enc_task);
                        }
                        None => {}
                    }
                });
            }
            enums::TableType::LogTable(_) => {}
        }
    }

    pub fn read_db(key: Vec<u8>) -> Vec<Task> {
        let mut task_table_enc: Vec<TaskEncrypted> = vec![];
        let mut task_table_decrypted: Vec<Task> = vec![];
        let conn_enum = <TaskEncrypted as enums::ConnDb>::conn_enum();
        match conn_enum {
            Conn::Conn(statement) => {
                let mut res_stmt = statement.prepare(
                    "SELECT
                name,
                shell,
                command,
                comment,
                month,
                day_of_month,
                day_of_week,
                hour,
                minute,
                year_added,
                month_added,
                day_of_month_added,
                hour_added,
                minute_added
                 FROM tasks",
                );
                match &mut res_stmt {
                    Ok(stmt) => {
                        let rows = stmt.query([]);
                        match rows {
                            Ok(mut rows) => {
                                while let Ok(Some(row)) = rows.next() {
                                    let mut name: Vec<u8> = vec![];
                                    let mut shell: Vec<u8> = vec![];
                                    let mut command: Vec<u8> = vec![];
                                    let mut comment: Vec<u8> = vec![];
                                    let mut month: Option<Vec<u8>> = None;
                                    let mut day_of_month: Option<Vec<u8>> = None;
                                    let mut day_of_week: Option<Vec<u8>> = None;
                                    let mut hour: Option<Vec<u8>> = None;
                                    let mut minute: Option<Vec<u8>> = None;
                                    let mut year_added: Vec<u8> = vec![];
                                    let mut month_added: Vec<u8> = vec![];
                                    let mut day_of_month_added: Vec<u8> = vec![];
                                    let mut hour_added: Vec<u8> = vec![];
                                    let mut minute_added: Vec<u8> = vec![];

                                    let res_name: Result<Vec<u8>, rusqlite::Error> = row.get(0);
                                    let res_shell: Result<Vec<u8>, rusqlite::Error> = row.get(1);
                                    let res_command: Result<Vec<u8>, rusqlite::Error> = row.get(2);
                                    let res_comment: Result<Vec<u8>, rusqlite::Error> = row.get(3);
                                    let res_month: Result<Vec<u8>, rusqlite::Error> = row.get(4);
                                    let res_day_of_month: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(5);
                                    let res_day_of_week: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(6);
                                    let res_hour: Result<Vec<u8>, rusqlite::Error> = row.get(7);
                                    let res_minute: Result<Vec<u8>, rusqlite::Error> = row.get(8);
                                    let res_year_added: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(9);
                                    let res_month_added: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(10);
                                    let res_day_of_month_added: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(11);
                                    let res_hour_added: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(12);
                                    let res_minute_added: Result<Vec<u8>, rusqlite::Error> =
                                        row.get(13);

                                    if let Ok(get_name) = res_name {
                                        name = get_name;
                                    }
                                    if let Ok(get_shell) = res_shell {
                                        shell = get_shell;
                                    }
                                    if let Ok(get_command) = res_command {
                                        command = get_command;
                                    }
                                    if let Ok(get_comment) = res_comment {
                                        comment = get_comment;
                                    }
                                    if let Ok(get_month) = res_month {
                                        month = Some(get_month);
                                    }
                                    if let Ok(get_day_of_month) = res_day_of_month {
                                        day_of_month = Some(get_day_of_month);
                                    }
                                    if let Ok(get_day_of_week) = res_day_of_week {
                                        day_of_week = Some(get_day_of_week);
                                    }
                                    if let Ok(get_hour) = res_hour {
                                        hour = Some(get_hour);
                                    }
                                    if let Ok(get_minute) = res_minute {
                                        minute = Some(get_minute);
                                    }
                                    if let Ok(get_year_added) = res_year_added {
                                        year_added = get_year_added;
                                    }
                                    if let Ok(get_month_added) = res_month_added {
                                        month_added = get_month_added;
                                    }
                                    if let Ok(get_day_of_month_added) = res_day_of_month_added {
                                        day_of_month_added = get_day_of_month_added;
                                    }
                                    if let Ok(get_hour_added) = res_hour_added {
                                        hour_added = get_hour_added;
                                    }
                                    if let Ok(get_minute_added) = res_minute_added {
                                        minute_added = get_minute_added;
                                    }
                                    let task_enc = TaskEncrypted {
                                        name,
                                        shell,
                                        command,
                                        comment,
                                        month,
                                        day_of_month,
                                        day_of_week,
                                        hour,
                                        minute,
                                        year_added,
                                        month_added,
                                        day_of_month_added,
                                        hour_added,
                                        minute_added,
                                    };
                                    task_table_enc.push(task_enc);
                                }
                                let res_dec = Task::decrypt_fields(task_table_enc, key);

                                if let Some(res_decrypted) = res_dec {
                                    for task in res_decrypted {
                                        task_table_decrypted.push(task.clone());
                                    }
                                }
                            }
                            Err(_error) => {}
                        }
                    }
                    Err(_error) => {}
                }
            }
            Conn::Error(_err) => {
                
            }
        }
        task_table_decrypted
    }

    pub fn rm_task(task_name: String, key: Vec<u8>) {
        let mut current_table = Self::read_db(key.clone());
        for i in 0..current_table.len() {
            if current_table[i].name == Some(task_name.clone()) {
                current_table.remove(i);
                break;
            }
        }
        Self::clear_tasks_table();
        for task in current_table{
            Self::add_task(task, key.clone());
        }
    }
    pub fn clear_tasks_table() -> bool {
        let conn_enum = <TaskEncrypted as enums::ConnDb>::conn_enum();
        match conn_enum {
            Conn::Conn(statement) => {
                let res_clear_db = statement.execute("DELETE FROM tasks", ());
                match res_clear_db {
                    Ok(_res) => true,
                    Err(_err) => false,
                }
            }
            Conn::Error(_) => false,
        }
    }
}
