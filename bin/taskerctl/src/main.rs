use std::{fs::{self, File},io::Write};
use sysinfo::{System, SystemExt, ProcessExt};
use clap::{Parser,Subcommand,Args};
use tasker_lib::taskerctl::{read_tasks_db,read_logs_db,rm_task,Task,add_task, execute_command_install,Configure};
use comfy_table::{Table,Row};
use terminal_text_styler::TerminalStyle;
use cron_list_parser::{get_crontab,CronJobEntry};
use std::env;

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
        ArgsInput::Cron(sync_options) => {
            match sync_options {
                SyncOptions::Show => {
                    show_cronjobs_fmt();
                }
                SyncOptions::All => {
                    sync_all_crontab();
                }
                SyncOptions::Id(id) => {
                    sync_crontab_by_id(id.id_num);
                }
            }
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
    Output,
    ///sync with existing crontab
    #[clap(subcommand)]
    Cron(SyncOptions)
}
#[derive(Debug,PartialEq,Clone,Subcommand)]
enum SyncOptions{
    ///sync all cronjobs with tasker
    All,
    ///show cronjobs by id
    Show,
    ///sync only specific cronjob by id
    Id(Id)
}
#[derive(Args,Debug,PartialEq,Clone)]
struct Id{
    id_num:i32
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
fn sync_crontab_by_id(id_cronjob:i32){
    let crontab: Vec<CronJobEntry> = get_crontab();
    for job in crontab{
        if job.id == id_cronjob{
            let mut task = Task::new();
            task.shell = shell();
            task.minute = convert_options_u8_i32(job.minute);
            task.hour = convert_options_u8_i32(job.hour);
            task.day_of_month = convert_options_u8_i32(job.day_of_month);
            task.month = convert_options_u8_i32(job.month);
            task.day_of_week = convert_options_u8_i32(job.day_of_week);
            task.command = job.command;
            add_task(task);
            println!("synced task with crontab!");
        }
    }
}
fn shell()->Option<String>{
            let mut shell: Option<String> = Some(String::from("bash"));
            let def_shell = env::var_os("shell");

            if let Some(shell_var) = def_shell {
                let def_shell_string: Option<&str> = shell_var.to_str();
                if let Some(conv_ok) = def_shell_string{
                    shell = Some(conv_ok.to_string());
                }
            }
            shell
}
fn sync_all_crontab(){
    let crontab: Vec<CronJobEntry> = get_crontab();
    for job in crontab{
        let mut task = Task::new();
        task.shell = shell();
        task.minute = convert_options_u8_i32(job.minute);
        task.hour = convert_options_u8_i32(job.hour);
        task.day_of_month = convert_options_u8_i32(job.day_of_month);
        task.month = convert_options_u8_i32(job.month);
        task.day_of_week = convert_options_u8_i32(job.day_of_week);
        task.command = job.command;
        add_task(task);
    }
    println!("synced all tasks with crontab!");
}
fn convert_options_u8_i32(option_u8:Option<u8>) -> Option<i32>{
    let mut option_i32:Option<i32> = None;
    if let Some (number) = option_u8 {
        option_i32 = Some(number as i32);
    }
    return option_i32;
}

#[allow(unused_assignments)]
fn show_cronjobs_fmt(){
    let cronjobs_list: Vec<CronJobEntry> = get_crontab();
    let mut cronjobs_table:Table = Table::new();
    let mut rows:Vec<Row> = vec![];
    let mut id:i32 = 0;
    let mut minute:u8= 0;
    let mut hour:u8 = 0;
    let mut day_of_month:u8 = 0;
    let mut month:u8 = 0;
    let mut day_of_week:u8 = 0;
    let mut command = String::from(" ");

    for job in cronjobs_list{
        id = job.id;
        if let Some(minute_job) = job.minute{
            minute = minute_job;
        }
        if let Some(hour_job) = job.hour{
            hour = hour_job;
        }
        if let Some(day_of_month_job) = job.day_of_month{
            day_of_month = day_of_month_job;
        }
        if let Some(month_job) = job.month{
            month = month_job;
        }
        if let Some(day_of_week_job) = job.day_of_week{
            day_of_week = day_of_week_job;
        }
        if let Some(command_job) = job.command{
            command = command_job.trim_start().to_string();
        }
        rows.push(Row::from(vec![
            &id.to_string(),
            &minute.to_string(),
            &hour.to_string(),
            &day_of_month.to_string(),
            &month.to_string(),
            &day_of_week.to_string(),
            &command
        ]));
    }
    cronjobs_table
        .set_header(vec![
            "id",
            "minute",
            "hour",
            "day_of_month",
            "month",
            "day_of_week",
            "command"]).add_rows(rows);
            println!("{cronjobs_table}");
}

#[allow(unused_assignments)]
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
    //add option for root service 
    let mut home_dir = simple_home_dir::home_dir().expect("error! can't get user home directory");
    home_dir.push(".config/systemd/user/");
    let header_1:&str = "[Unit]";
    let desc:&str = "Description=tasker service";
    let header_2:&str = "[Service]";
    let exec_start:String = "ExecStart=/usr/bin/tasker_service".to_string() ;
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
            exec_start.as_str(), header_3, last_line];
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
    let mut pid_num: Vec<String> = vec![];
    let system: System = System::new_all();
    for (pid,procces) in system.processes(){
        if procces.name() == "tasker_service"{
            is_running = true;
            pid_num.push(pid.to_string());
        }
    }
    match is_running{
        true => {
            for pid in pid_num{
                println!("tasker service is up and running!, pid num: {pid}");
            }
            
        }
        false => {
            println!("tasker service is not running");
        }
    }
}




