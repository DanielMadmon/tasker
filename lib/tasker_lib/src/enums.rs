use crate::structs::{LogEntry, LogEntryEncrypted, Task, TaskEncrypted};

#[derive(Clone, Debug)]
/// our main Error handler, for now it dosent do much beside printing the error to stderr
/// in the future we will use the feedback from the enum to respond and smooth the library ops.
//TODO:use for actions, not just printing error messages
pub enum ErrorType {
    ConnectionError,
    RusqliteError,
    EncryptionError,
    DecryptionError,
    KeyringError,
    EmptyTable,
    ShellNotFound,
    CommandError
}

///the Conn enum is used mainly for collecting the result of initiating a connection to our tasker.db
pub enum Conn<C, E> {
    Conn(C),
    Error(E),
}
///the OS enum is used for selecting the current platform
#[derive(PartialEq)]
pub enum OS {
    Linux,
    Mac,
    Windows,
    Unknown,
}

///a trait for Our struct to get the result of connecting to task.db
pub trait ConnDb {
    fn conn_enum() -> Conn<rusqlite::Connection, rusqlite::Error> {
        super::enums::Conn::get_conn_res()
    }
}
impl ConnDb for crate::structs::TaskEncrypted {}
impl ConnDb for LogEntryEncrypted {}
///because we use AES-256 for encrypting the fields, when decrypting we get 4 bytes for i32, this trait converts it
/// to a decimal number
pub trait ConvertBytesToHumanReadable {
    fn convert_le_bytes_i32(four_bytes: &[u8]) -> i32 {
        let mut index = 0;
        let mut res_int: i32 = 0;
        while index <= 3 {
            let num = i32::from(four_bytes[index]);
            if index == 0 {
                res_int += num;
            } else if index == 1 {
                res_int += num * 256;
            } else if index == 2 {
                res_int += num * 65_536;
            } else if index == 3 {
                res_int += num * 16_777_216;
            }
            index += 1;
        }
        res_int
    }
///same as above just for bytes, when fails we just push the error message into the table
    fn convert_bytes_to_string(bytes: Vec<u8>) -> String {
        let res_conv = String::from_utf8(bytes);
        match res_conv{
            Ok(res_string) => {
                res_string
            }
            Err(_)=>{
                String::from("Error Encypting/Decrypting..")
            }
        }
    }
}
///an enum for for getting the 'is_duplicate' function below,to get both types of rows.
pub enum RowType {
    TaskRow(Task),
    LogRow(LogEntry),
}
///an enum for getting the 'is_duplicate' function below, to get both types of tables.
#[derive(Debug)]
pub enum TableType {
    TaskTable(Vec<Task>),
    LogTable(Vec<LogEntry>),
}
///whenever we want to add or modify the logs/tasks table we use the is_duplicate method below.
/// it's job is to make sure we won't have duplicate tasks/logs , by reading the tables and comparing 
/// the name field.
/// if we try to add a log/task with the same name it would just update the fields with the new
/// values we passed in, and return the full modified table.
pub trait NewRowTableBuilder {
    fn is_duplicate(new_row: RowType, key: Vec<u8>) -> TableType {
        match new_row {
            RowType::TaskRow(new_task) => {
                /*
                read all tasks row by row and if rowname = new name change row data
                 */
                let mut is_exist = false;
                let mut tasks_table = TaskEncrypted::read_db(key);

                let mut idx_isize: isize = -1;
                for task in tasks_table.clone() {
                    idx_isize += 1;
                    if new_task.name.clone().unwrap() == task.name.unwrap() {
                        is_exist = true;
                        break;
                    }
                }
                if is_exist {
                    tasks_table[idx_isize as usize] = Task {
                        name: new_task.name,
                        shell: new_task.shell,
                        command: new_task.command,
                        comment: new_task.comment,
                        month: new_task.month,
                        day_of_month: new_task.day_of_month,
                        day_of_week: new_task.day_of_week,
                        hour: new_task.hour,
                        minute: new_task.minute,
                        year_added: new_task.year_added,
                        month_added: new_task.month_added,
                        day_of_month_added: new_task.day_of_month_added,
                        hour_added: new_task.hour_added,
                        minute_added: new_task.minute_added,
                    };
                    return TableType::TaskTable(tasks_table);
                } else {
                    tasks_table.push(new_task);
                    return TableType::TaskTable(tasks_table);
                }
            }
            RowType::LogRow(new_log) => {
                let mut is_exist = false;
                let mut logs_table_back: Vec<LogEntry> = Vec::new();
                let logs_option = LogEntryEncrypted::read_logs(key.clone());
                if logs_option.is_some() {
                    logs_table_back = logs_option.unwrap();
                }

                let mut idx_isize: isize = -1;
                for log in logs_table_back.clone() {
                    idx_isize += 1;
                    if log.name.clone() == new_log.name.clone() {
                        is_exist = true;
                        break;
                    }
                }
                if is_exist {
                    logs_table_back[idx_isize as usize] = LogEntry {
                        name: new_log.name,
                        command: new_log.command,
                        output: new_log.output,
                        execution_year: new_log.execution_year,
                        execution_month: new_log.execution_month,
                        execution_week: new_log.execution_week,
                        execution_day_of_month: new_log.execution_day_of_month,
                        execution_day_of_week: new_log.execution_day_of_week,
                        execution_hour: new_log.execution_hour,
                        execution_minute: new_log.execution_minute,
                    };
                    TableType::LogTable(logs_table_back)
                } else {
                    logs_table_back.push(new_log);
                    TableType::LogTable(logs_table_back)
                }
            }
        }
    }
}
pub enum Configure{
    EnableTaskerService,
    DisableTaskerService
}

impl ConvertBytesToHumanReadable for Task {}
impl ConvertBytesToHumanReadable for LogEntry {}
impl NewRowTableBuilder for LogEntryEncrypted {}
impl NewRowTableBuilder for TaskEncrypted {}
