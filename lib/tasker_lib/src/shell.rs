use crate::enums::{OS,ErrorType, Configure};
use crate::structs::{LogEntry, LogEntryEncrypted, Task};
use crate::taskerctl::CurrentTime;
use std::borrow::Cow;
use std::env;
use std::process::Command;

pub fn execute_and_log(task: Task, key: Vec<u8>) -> bool {
    let mut task_shell = String::new();
    let mut task_command = String::new();
    match task.shell.clone() {
        Some(shell_name) => task_shell = shell_name,
        None => {
            ErrorType::error(&ErrorType::ShellNotFound);
        }
    }
    match task.command.clone() {
        Some(command) => task_command = command,
        None => {
            ErrorType::error(&ErrorType::CommandError);
        }
    }
    let mut conf_command = Command::new(task_shell);
    let mut tmp_non_utf: &str = "";
    let mut tmp_string: String = String::new();

    let os = OS::get_os();
    match os {
        OS::Linux => conf_command.arg("-c"),
        OS::Mac => conf_command.arg("-c"),
        OS::Windows => conf_command.arg("/c"),
        OS::Unknown => conf_command.arg("-c"),
    };
    conf_command.arg(task_command);
    let command_stdout = conf_command.output();
    match command_stdout {
        Ok(stdout) => {
            let output_string = stdout.stdout;
            let output_string: Cow<str> = String::from_utf8_lossy(output_string.as_slice());
            match output_string {
                Cow::Borrowed(non_utf8_error) => {
                    tmp_non_utf = non_utf8_error;
                }
                Cow::Owned(result) => {
                    tmp_string = result;
                }
            }
            let now = CurrentTime::now();
            let output: String = tmp_non_utf.to_owned() + &tmp_string;
            let new_row = LogEntry {
                name: task.clone().name.unwrap(), //TODO:remove unwrap
                command: task.clone().command.unwrap(),
                output: Some(output),
                execution_year: now.year,
                execution_month: now.month,
                execution_week: now.week,
                execution_day_of_month: now.day_of_month,
                execution_day_of_week: now.day_of_week_num,
                execution_hour: now.hour,
                execution_minute: now.min,
            };
            let res = LogEntryEncrypted::add_log(new_row, key.clone());
            return res;
        }
        Err(_failed_to_execute) => {
            let now = CurrentTime::now();
            let new_row = LogEntry {
                name: task.clone().name.unwrap(), //name must be given!
                command: task.clone().command.unwrap(),
                output: None,
                execution_year: now.year,
                execution_month: now.month,
                execution_week: now.week,
                execution_day_of_month: now.day_of_month,
                execution_day_of_week: now.day_of_week_num,
                execution_hour: now.hour,
                execution_minute: now.min,
            };
            let res = LogEntryEncrypted::add_log(new_row, key.clone());
            return res;
        }
    }
}
pub fn execute_for_install(com_type:Configure){
    let key:&str = "SHELL";
    let mut shell:String = String::from("bash"); //TODO:add detection of os.
    if let Some (shell_os) = env::var_os(key) {
        let res: Option<&str> = shell_os.to_str().to_owned();
        if let Some(shell_res) = res {
            shell = shell_res.to_string();
        } 
    }
    match com_type {
        Configure::EnableTaskerService => {
            let mut command = Command::new(shell);
            command.arg("-c"); //TODO:add os detection here
            command.arg("systemctl --user enable tasker.service");
            let mut res = command.spawn().expect("error running systemctl command!");
            res.wait().expect("error running systemctl command!");
        }
        Configure::DisableTaskerService => {
            let mut command = Command::new(shell);
            command.arg("-c");//TODO:add os detection here
            command.arg("systemctl --user disable tasker.service");
            let mut res = command.spawn().expect("error running systemctl command!");
            res.wait().expect("error running systemctl command!");
        }
    }
}
