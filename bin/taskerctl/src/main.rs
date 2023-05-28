use std::{fs::{self, File},io::Write};
use sysinfo::{System, SystemExt, ProcessExt};
use clap::{Parser,Subcommand,Args};
use tasker_lib::taskerctl::{read_tasks_db,read_logs_db,rm_task,Task,add_task, execute_command_install,Configure};
use comfy_table::{Table,Row};
use terminal_text_styler::TerminalStyle;

fn main(){
    
    let input = ArgsData::parse();
    let mut new_task = Task::new();
    match input.input{
        ArgsInput::Add(options) => {
         if let Some(name) = options.name{
            new_task.name = Some(name);
         }
         if let Some(shell) = options.shell{
            new_task.shell = Some(shell);
         }
         if let Some(command) = options.command{
            new_task.command = Some(command);
         }
         new_task.comment = options.comment;
         new_task.month = options.month;
         new_task.day_of_month = options.month;
         new_task.day_of_week = options.day_of_week;
         new_task.hour = options.hour;
         new_task.minute = options.minute;
         add_task(new_task);
         println!("new task added successfully.");
        }
        ArgsInput::Status => {
            status();
        }
        ArgsInput::List => {
            show_list();
        }
        ArgsInput::Remove(task_name) =>{
            remove_task(task_name.name);
        }
        ArgsInput::Logs =>{
            show_logs();
        }
        ArgsInput::Output=>{
            logs_output();
        }
        ArgsInput::Enable=>{
            enable();
        }
        ArgsInput::Disable => {
            disable();
        }
    }
}
    
#[derive(Debug,Parser,PartialEq)]
#[clap(author,version,about)]
#[command(author, version, about, long_about = None)]
struct ArgsData{
    #[clap(subcommand)]
    input:ArgsInput
}

#[derive(Debug, Subcommand,PartialEq,Clone)]
#[command(author, version, about, long_about = None)]
enum ArgsInput{
    ///enable tasker-service
    Enable,
    ///disable tasker-service
    Disable,
    ///show tasker-service status
    Status,
    ///show all the tasks
    List,
    ///Add new task
    Add(AddOptions),
    ///remove task by passing task's name
    Remove(TaskName),
    ///show logs
    Logs,
    ///Output
    Output
}

#[derive(Args,Debug,PartialEq,Clone)]
struct  AddOptions{
    ///add task name (required)
    #[clap(short = 'n',long = "name")]
    name:Option<String>,
    ///add shell to run the command with (required)
    #[clap(short = 's', long = "shell")]
    shell:Option<String>,
    ///add task command (required)
    #[arg(short='c',long="command")]
    command: Option<String>,
    ///add comment
    #[arg(long = "comment")]
    comment: Option<String>,
    ///add month of execution (optional)
    #[arg(short='m',long="month")]
    month: Option<i32>,
    ///add day of month for execution (optional)
    #[arg(short='d',long="day_month")]
    day_of_month: Option<i32>,
    ///add day of week for execution (optional)
    #[arg(short='w',long="day_week")]
    day_of_week: Option<i32>,
    ///add hour of execution (optional)
    #[arg(short='u',long="hour")]
    hour: Option<i32>,
    ///add minute of execution (optional)
    #[arg(short='t',long="minute")]
    minute: Option<i32>,
}
#[derive(Debug,PartialEq,Args,Clone)]
struct TaskName{
    name:String
}

fn show_list(){

    let task_db: Vec<Task> = read_tasks_db();
    let mut tasks_table: Table = Table::new();
    let mut rows: Vec<Row> = vec![];
    let mut month:i32 = 0;
    let mut day_of_month:i32 = 0;
    let mut day_of_week:i32 = 0;
    let mut hour:i32 = 0;
    let mut minute:i32 = 0;
    let mut comment:String = String::from("None");
    for task in task_db{
        if let Some(month_res) = task.month{
            month = month_res;
        }
        if let Some(dom_res) = task.day_of_month {
            day_of_month = dom_res;
        }
        if let Some(dow_res) = task.day_of_week{
            day_of_week = dow_res;
        }
        match task.hour{
            Some(hour_get)=>{
                hour = hour_get;
            }
            None => {
                hour = 0;
            }
        }
        if let Some(min_res) = task.minute {
            minute = min_res;
        }
         if let Some(comment_res) = task.comment {
            comment = comment_res;
        }
        rows.push(Row::from(vec![
            task.name.unwrap(),
            task.shell.unwrap(),
            task.command.unwrap(),
            month.to_string(),
            day_of_month.to_string(),
            day_of_week.to_string(),
            hour.to_string(),
            minute.to_string(),
            comment.clone()
        ]));
    }
    tasks_table 
        .set_header(vec![
        "name",
        "shell",
        "command",
        "month",
        "day_of_month",
        "day_of_week",
        "hour",
        "minute",
        "comment"])
        .add_rows(rows)
        ;
    
    println!("{tasks_table}");
}
fn show_logs(){
    let logs = read_logs_db();
    let mut logs_table:Table = Table::new();
    let mut rows: Vec<Row> = vec![];
    if logs.is_some(){
        for log in logs.unwrap(){
            rows.push(Row::from(vec![
                log.name,
                log.command,
                log.execution_month.to_string(),
                log.execution_day_of_month.to_string(),
                log.execution_hour.to_string(),
                log.execution_minute.to_string()
            ]));
        }
        logs_table
            .set_header(vec![
                "name",
                "command",
                "execution_month",
                "execution_day_of_month",
                "execution_hour",
                "execution_minute"
                ])
            .add_rows(rows)
                ;
            logs_table.trim_fmt();
            println!("{logs_table}");
    }
}
fn remove_task(task_name:String){
    rm_task(task_name);
    println!("task deleted");
}
fn logs_output(){
    let yellow: TerminalStyle = TerminalStyle::yellow_background();
    let italic = TerminalStyle::italic_white();
    let logs = read_logs_db();
    if let Some(logs) = logs{
        for log in logs{
            let name = log.name;
            println!("Task Name: {}",yellow.wrap(&name));
            if let Some(out) = log.output{
                let output_lines = out.lines();
                for line in output_lines{
                    println!("{}",italic.wrap(line));
                }
            }
           
            
        }
    }
}
fn enable(){
    let res_name = hostname::get().expect("error! can't set service file");
    let res_name = res_name.to_str().expect("error! can't set up service file");
    let mut home_dir = simple_home_dir::home_dir().expect("error! can't get user home directory");

    home_dir.push(".config/systemd/user/");

    let header_1:&str = "[Unit]";
    let desc:&str = "Description=tasker service";
    let header_2:&str = "[Service]";
    let user:String = "user=".to_string() + res_name;
    let exec_start:String = "ExecStart=/home/".to_string() + res_name + "/.cargo/bin/tasker_service";
    let header_3: &str = "[Install]";
    let last_line:&str = "WantedBy=default.target";

    let _res_create_service = fs::create_dir_all(&home_dir);
    home_dir.push("tasker.service");
    let service_file = File::create(&home_dir);
    match service_file{
        Ok(_service_file) => {
            let mut service_writer = 
            File::options().append(true).open(home_dir)
            .expect("error! can't edit service file .please check permissions in ~.config/systemd/user/");
        let lines = [header_1, desc, header_2, 
            user.as_str(), exec_start.as_str(), header_3, last_line];
            for line in lines{
                writeln!(& mut service_writer, "{line}")
                .expect("error! can't edit service file .please check permissions in ~.config/systemd/user/");
            }
            let enable = Configure::EnableTaskerService;
            execute_command_install(enable);
            println!("tasker service enabled Successfully!")
        }
        Err(err)=>{
            eprintln!("error, can't create service file, info {err:#?}")
        }
    }
}
fn disable(){
    execute_command_install(Configure::DisableTaskerService);
    println!("tasker service disabled Successfully")
}
fn status(){
    let mut is_running:bool = false;
    let mut pid_num: String = String::new();
    let system: System = System::new_all();
    for (pid,procces) in system.processes(){
        if procces.name() == "tasker_service"{
            is_running = true;
            pid_num = pid.to_string();
            break;
        }
    }
    match is_running{
        true => {
            println!("tasker service is up and running!, pid num: {pid_num}");
        }
        false => {
            println!("tasker service is not running");
        }
    }
}