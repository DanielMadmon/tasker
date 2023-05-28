//! the main part of the daemon which is responsible for:
//! getting tasks -> filtering -> executing -> logging -> repeat(sleeps until next update time or until next task)

use crate::{
    shell,
    structs::{CurrentTime, LogEntry, LogEntryEncrypted, Task, TaskEncrypted},
};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::{thread, time::Duration};

fn get_logs_table(key: Vec<u8>) -> Option<Vec<LogEntry>> {
    let log_table = LogEntryEncrypted::read_logs(key.clone());
    if let Some(logs_table) = log_table.clone() {
        Some(logs_table)
    } else {
        None
    }
}
fn get_tasks_table(key: Vec<u8>) -> Option<Vec<Task>> {
    let tasks_vec: Vec<Task> = TaskEncrypted::read_db(key.clone());
    if tasks_vec.is_empty() {
        None
    } else {
        Some(tasks_vec)
    }
}
///our main entry point:
/// responsible for:
/// 0.getting both tasks and logs tables
/// 1.filtering them 
/// 2.passing the filtered tasks table for execution
/// sleeping until next task or one hour whichever is smaller
fn entry_point(key: Vec<u8>) {
    loop {
        let mut time_sleep: i32 = 3_600;
        let logs_table: Option<Vec<LogEntry>> = get_logs_table(key.clone());
        let tasks_table: Option<Vec<Task>> = get_tasks_table(key.clone());
        if let Some(tasks_table_opt) = tasks_table.clone() {
            if let Some(logs_table_opt) = logs_table {
                time_sleep = filter_table(tasks_table_opt, Some(logs_table_opt), key.clone());
            } else if logs_table.is_none() && tasks_table.is_some() {
                time_sleep = filter_table(tasks_table_opt, None, key.clone());
            }
        }
        thread::sleep(Duration::from_secs(time_sleep as u64));
    }
}
///naive cmp of task field and current time
/// returns -> bool 
fn should_execute_now(task: Task) -> bool {
    let now = CurrentTime::now();
    let mut month: bool = false;
    let mut day_of_month: bool = false;
    let mut day_of_week: bool = false;
    let mut hour: bool = false;
    let mut minute: bool = false;
    let mut to_execute: bool = true;

    if task.month.is_some() {
        if task.month.unwrap() == now.month {
            month = true;
        }
    } else if task.month.is_none() {
        month = true;
    }
    if task.day_of_month.is_some() {
        if task.day_of_month.unwrap() == now.day_of_month {
            day_of_month = true;
        }
    } else if task.day_of_month.is_none() {
        day_of_month = true;
    }

    if task.day_of_week.is_some() {
        if task.day_of_week.unwrap() == now.day_of_week_num {
            day_of_week = true;
        }
    } else if task.day_of_week.is_none() {
        day_of_week = true;
    }
    if task.hour.is_some() {
        if task.hour.unwrap() == now.hour {
            hour = true;
        }
    } else if task.hour.is_none() {
        hour = true;
    }
    if task.minute.is_some() {
        if task.minute.unwrap() == now.min {
            minute = true;
        }
    } else if task.minute.is_none() {
        minute = true;
    }
    let truth_table = [month, day_of_month, day_of_week, hour, minute];
    for time in truth_table {
        if time == false {
            to_execute = false
        }
    }
    if to_execute {
        true
    } else {
        false
    }
}
///checks if task which should have executed daily, have been missed today,
/// necessary due to daily tasks having a really small short time,which can be
/// easily missed when using only the above naive approach 
fn is_daily_task_missed_logged(task: &Task, log: &LogEntry) -> bool {
    if task.month.is_none() && task.day_of_month.is_none() && task.day_of_week.is_none() {
        if task.hour.is_some() || task.hour.is_some() && task.minute.is_some() {
            if log.execution_day_of_month != CurrentTime::now().day_of_month {
                return true;
            }
        }
    }

    false
}
///filtering function which takes in the log/tasks table and uses the above functions, to get
/// tasks which should execute now.
/// tasks are the passed for execution, and the function returns time_until_next_task:i32 
/// to be used by the main loop
fn filter_table(tasks_table: Vec<Task>, log_table: Option<Vec<LogEntry>>, key: Vec<u8>) -> i32 {
    let mut execution_table: Vec<Task> = Vec::new();
    //in case the log exist
    if log_table.is_some() {
        let log_table_unwarped = log_table.clone().unwrap();
        for task in tasks_table.clone() {
            let mut task_in_logs: bool = false;
            for log in log_table_unwarped.clone() {
                if task.name == Some(log.clone().name) {
                    task_in_logs = true;
                    let to_execute = should_execute_now(task.clone());
                    let daily_missed: bool = is_daily_task_missed_logged(&task, &log);
                    if to_execute
                        && !was_task_executed(&task, &log)
                        && !execution_table.contains(&task)
                        || daily_missed
                    {
                        execution_table.push(task.clone());
                    }
                }
            }
            if log_table.is_some()
                && !task_in_logs
                && should_execute_now(task.clone())
                && !execution_table.contains(&task)
            {
                execution_table.push(task.clone());
            }
        }
        //in case there are no logs
    }
    if log_table.is_none() {
        for task in tasks_table.clone() {
            let to_execute = should_execute_now(task.clone());
            if to_execute && !execution_table.contains(&task) {
                execution_table.push(task.clone());
            }
        }
    }
    execute_tasks(execution_table.clone(), key);
    time_until_next_task(tasks_table)
}

fn time_until_next_task(tasks_table: Vec<Task>) -> i32 {
    let mut sleep_month: Option<i32> = None;
    let mut sleep_dom: Option<i32> = None;
    let mut sleep_dow: Option<i32> = None;
    let mut sleep_hour: Option<i32> = None;
    let mut sleep_min: Option<i32> = None;

    let mut seconds_until_task_cmp: i32 = 64_281_600; //two years in seconds

    for task in tasks_table {
        let mut seconds_until_task: i32 = 0;
        let now = CurrentTime::now();
        let task_arr = [
            task.month,
            task.day_of_month,
            task.day_of_week,
            task.hour,
            task.minute,
        ];
        let mut idx: i32 = -1;
        for field in task_arr {
            idx += 1;
            if field.is_some() {
                if idx == 0 {
                    if sleep_month.is_some() {
                        if sleep_month.unwrap() > task.month.unwrap() {
                            sleep_month = Some(task.month.unwrap());
                        }
                    } else if sleep_month.is_none() {
                        sleep_month = Some(task.month.unwrap());
                    }
                    let month = task.month.unwrap();
                    if month > now.month {
                        let time_until_in_month: i32 = month - now.month;
                        let month_in_secs: i32 = time_until_in_month * 2_628_000;
                        seconds_until_task += month_in_secs;
                    } else if month < now.month {
                        let time_until_month: i32 = 12 - now.month + month;
                        let month_in_seconds: i32 = time_until_month * 2_628_000;
                        seconds_until_task += month_in_seconds;
                    }
                } else if idx == 1 {
                    if sleep_dom.is_some() {
                        if sleep_dom.unwrap() > task.day_of_month.unwrap() {
                            sleep_dom = Some(task.day_of_month.unwrap())
                        }
                    } else if sleep_dom.is_none() {
                        sleep_dom = Some(task.day_of_month.unwrap());
                    }
                    let dom: i32 = task.day_of_month.unwrap();
                    if dom > now.day_of_month {
                        //let time_until_dom: i32 = dom - now.day_of_month;
                        let dom_secs: i32 = dom * 86_400;
                        seconds_until_task += dom_secs;
                    } else if dom < now.day_of_month {
                        //FIXME:naive approach assuming 29 days in all months
                        let time_until_dom: i32 = 29 - now.day_of_month + dom;
                        let dom_secs: i32 = time_until_dom * 86_400;
                        seconds_until_task += dom_secs;
                    }
                } else if idx == 2 {
                    if sleep_dow.is_some() {
                        if sleep_dow.unwrap() > task.day_of_week.unwrap() {
                            sleep_dow = Some(task.day_of_week.unwrap());
                        }
                    } else if sleep_dow.is_none() {
                        sleep_dow = Some(task.day_of_week.unwrap());
                    }
                    let dow: i32 = task.day_of_week.unwrap();
                    if dow > now.day_of_week_num {
                        let time_until_dow: i32 = dow - now.day_of_week_num;
                        let dow_secs: i32 = time_until_dow * 86_400;
                        seconds_until_task += dow_secs;
                    } else if dow < now.day_of_week_num {
                        let time_until_dow: i32 = 7 - now.day_of_week_num + dow;
                        let dow_secs: i32 = time_until_dow * 86_400;
                        seconds_until_task += dow_secs;
                    }
                } else if idx == 3 {
                    if sleep_hour.is_some() {
                        if sleep_hour.unwrap() > task.hour.unwrap() {
                            sleep_hour = Some(task.hour.unwrap());
                        }
                    } else if sleep_hour.is_none() {
                        sleep_hour = Some(task.hour.unwrap());
                    }
                    let hour: i32 = task.hour.unwrap();
                    if hour > now.hour {
                        let time_until_hour: i32 = hour - now.hour;
                        let hour_secs: i32 = time_until_hour * 3_600;
                        seconds_until_task += hour_secs;
                    } else if hour < now.hour {
                        let time_until_hour: i32 = 24 - now.hour + hour;
                        let hour_secs: i32 = time_until_hour * 36_000;
                        seconds_until_task += hour_secs;
                    }
                } else if idx == 4 {
                    if sleep_min.is_some() {
                        if sleep_min.unwrap() > task.minute.unwrap() {
                            sleep_min = Some(task.minute.unwrap());
                        }
                    } else if sleep_min.is_none() {
                        sleep_min = Some(task.minute.unwrap());
                    }
                    let minute: i32 = task.minute.unwrap();
                    if minute > now.min {
                        let time_until_min: i32 = minute - now.min;
                        let min_secs: i32 = time_until_min * 60;
                        seconds_until_task += min_secs;
                    } else if minute < now.min {
                        let time_until_min: i32 = 60 - now.min + minute;
                        let min_secs: i32 = time_until_min * 60;
                        seconds_until_task += min_secs;
                    }
                }
            }
        }
        if seconds_until_task < seconds_until_task_cmp {
            seconds_until_task_cmp = seconds_until_task;
        }
    }
    if seconds_until_task_cmp > 3_600 {
        //1 hour between updates
        3_600
    } else if seconds_until_task_cmp == 0 {
        60
    } else {
        seconds_until_task_cmp
    }
}

fn was_task_executed(task: &Task, log: &LogEntry) -> bool {
    let now = CurrentTime::now();
    if task.month == Some(log.execution_month) && Some(log.execution_year) == Some(now.year) {
        return true;
    } else if task.day_of_month == Some(log.execution_day_of_month)
        && Some(log.execution_week) == Some(now.week)
    {
        return true;
    } else if task.day_of_week == Some(log.execution_day_of_week)
        && Some(log.execution_week) == Some(now.week)
    {
        return true;
    } else if task.hour == Some(log.execution_hour)
        && log.execution_week == now.week
        && log.execution_day_of_month == now.day_of_month
        && log.execution_month == now.month
    {
        return true;
    } else if task.minute == Some(log.execution_minute)
        && log.execution_week == now.week
        && log.execution_day_of_month == now.month
        && log.execution_hour == now.hour
    {
        return true;
    } else {
        false
    }
}
///executes each task in parralel, mostly done in order to avoid blocking the main thread until getting commands
/// output
fn execute_tasks(mut tasks: Vec<Task>, key: Vec<u8>) {
    let tasks_iter = tasks.par_iter_mut().enumerate();
    tasks_iter.for_each(|task| {
        shell::execute_and_log(task.1.to_owned(), key.clone());
        let _res 
        = crate::notify::notify(&task.1.clone().name.unwrap());
    });
}
///the main entry for the tasker_service daemon
pub fn main_execution_loop(key: Vec<u8>) {
    entry_point(key);
}
