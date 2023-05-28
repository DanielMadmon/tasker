//!the library exports two modules the first is taskerctl for configuring tasks and reading logs/tasks, table.
//! ##taskerctl
//! #usage example: 
//! ```
//! use tasker::taskerctl::{read_tasks_db,read_logs_db,rm_task,Task,add_task};
//!     let task_db: Vec<Task> = read_tasks_db(); //read tasks database
//!     let logs = read_logs_db();                //read logs database
//!     let mut new_task = Task::new();           //get new Task struct
//!     new_task.name = Some(String::from("new_task")) //set name , and the rest of the values
//!     ...
//!     add_task(new_task);                        //add the new task to the tasks database
//!```
//! tasker_service:
//! the second module is used for starting the tasker_service daemon which runs in the background, and 
//! execute tasks, based on our taskerctl configurations.
//! usage example:
//! ```
//! use tasker::tasker_service;
//!fn main() {
//!    tasker_service::main_service(); //starting the service
//!  }
//! ```

#![warn(clippy::correctness)]
#![warn(clippy::perf)]

mod config;
mod connect_db;
mod enums;
mod error_handler;
mod log_db;
mod secret;
mod secure;
mod shell;
mod structs;
mod tasker_logic;
mod tasks_db;
mod notify;
mod root_key_request;
mod tests;
pub mod time;


pub mod taskerctl {
    use random_string;
    use crate::{shell, secret, root_key_request};
    pub use crate::structs::{CurrentTime, Task};
    pub use crate::enums::Configure;
    use crate::structs::{LogEntry, LogEntryEncrypted, TaskEncrypted};
    use crate::config::is_root;
//TODO:Update is root in here!
    pub fn execute_command_install(enable_disable: Configure){
        shell::execute_for_install(enable_disable);    
    }
    
    pub fn read_tasks_db() -> Vec<Task> {
        TaskEncrypted::read_db(get_keys())
    }
    pub fn read_logs_db() -> Option<Vec<LogEntry>> {
        LogEntryEncrypted::read_logs(get_keys())
    }
     pub(crate) fn get_keys() -> Vec<u8> {
        if !is_root(){
            secret::get_key_linux()
        }else{
            root_key_request::get_key_root()
        }
        
    }
    pub fn add_task(task: Task) {
        TaskEncrypted::add_task(task, get_keys());
    }
    pub fn rm_task(task_name: String){
        TaskEncrypted::rm_task(task_name, get_keys());
    }
    impl Task{
        pub fn new()->Task{
            let az: String = "abcdefghijklmnopqrstuvwxyz".to_string();
            let name_holder = random_string::generate(6, az);
            let now = CurrentTime::now();
            Task { name: Some(name_holder),
                 shell: None, 
                 command: None, 
                 comment: None, 
                 month: None, 
                 day_of_month: None, 
                 day_of_week: None, 
                 hour: None, 
                 minute: None, 
                 year_added: now.year, 
                 month_added: now.month, 
                 day_of_month_added: now.day_of_month, 
                 hour_added: now.hour, 
                 minute_added: now.min }
        }
    }
}

pub mod tasker_service {
    use crate::{tasker_logic, taskerctl::get_keys,root_key_request::get_key_root,config::is_root};
    pub fn main_service() {
        if is_root(){
            tasker_logic::main_execution_loop(get_key_root());
        }else if !is_root(){
            tasker_logic::main_execution_loop(get_keys());
        }
    }
}
