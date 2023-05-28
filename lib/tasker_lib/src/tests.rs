//TODO:configure actual tests


#[cfg(test)]
use crate::secret;
#[cfg(test)]
use crate::structs::{CurrentTime, LogEntry, LogEntryEncrypted, Task, TaskEncrypted};
#[cfg(test)]
use rayon::prelude::*;
#[test]
fn test_tasks_write_read() {
    let key: Vec<u8> = secret::get_key_linux();
    let num_arr = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 29,
    ]
    .par_iter();
    num_arr.for_each(|i| {
        let task = Task {
            name: Some(i.to_string()),
            shell: Some("fish".to_owned()),
            command: Some("echo fish".to_owned()),
            comment: Some("testing testing".to_owned()),
            month: Some(1),
            day_of_month: Some(3),
            day_of_week: None,
            hour: Some(12),
            minute: Some(12),
            year_added: 2023,
            month_added: 12,
            day_of_month_added: 12,
            hour_added: 12,
            minute_added: 11,
        };
        TaskEncrypted::add_task(task, key.clone());
    });

    let now_min = CurrentTime::now().min as f32;
    let now_hour = CurrentTime::now().hour as f32;

    dbg!("started reading at: hh: {}, mm: {}", now_hour, now_min);
    let res_read = TaskEncrypted::read_db(key.clone());

    let now_min = CurrentTime::now().min as f32;
    let now_hour = CurrentTime::now().hour as f32;

    dbg!("finished reading at: hh: {}, mm: {}", now_hour, now_min);

    if res_read.len() < 1 {
        panic!("error reading tasks from database");
    }
}

#[test]
fn test_log_write_read() {
    let mut log_vec: Vec<LogEntry> = vec![];
    let key: Vec<u8> = "testingtesting".as_bytes().to_vec();
    for i in 0..5 {
        let log = LogEntry {
            name: i.to_string(),
            command: "no command".to_owned(),
            output: None,
            execution_year: 2022,
            execution_month: 2,
            execution_week: 3,
            execution_day_of_month: 4,
            execution_day_of_week: 5,
            execution_hour: 6,
            execution_minute: 7,
        };
        log_vec.push(log);
    }

    for log in log_vec {
        let res_add = LogEntryEncrypted::add_log(log, key.clone());
        if res_add == false {
            panic!("error adding log to database!");
        }
    }
    let res_read = LogEntryEncrypted::read_logs(key.clone());
    if let None = res_read {
        panic!("error reading from log database")
    }
}
//TODO:add test for execution module
#[test]
fn test_tasker_logic() {}
#[test]
fn test_notify(){
    use crate::notify;
    let string = "some_string".to_string();
    let _res_noti: Result<notify_rust::NotificationHandle, notify_rust::error::Error> = 
    notify::notify(&string);
}
