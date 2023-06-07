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
         if let Some(crontab_path) = options.cron {
            sync_with_crontab(crontab_path);
            println!("synced tasks from crontab successfully")
         }
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
    #[arg(long="cron")]
    cron:Option<String>
}
#[derive(Debug,PartialEq,Args,Clone)]
struct TaskName{
    name:String
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



pub mod cron_syncer{
    use tasker_lib::taskerctl::add_task;
    use cronparse::CrontabFile;
    use cronparse::crontab::UserCrontabEntry;
    pub fn sync_with_crontab(path_to_crontab:String){
        let tasks_from_tab: Vec<Task> = get_crontab(path_to_crontab);
        for task in tasks_from_tab{
            add_task(task);
        }
    }
    fn get_crontab(path_to_crontab:String) -> Vec<Task>{
        type CronFile =  CrontabFile<UserCrontabEntry>;
        type CronEntries =  CrontabFile::<UserCrontabEntry>;
        type CronMonthOption<'a> = Option<& 'a cronparse::interval::Interval<cronparse::schedule::Month>>;
        type CronDomOption<'b> = Option<& 'b cronparse::interval::Interval<cronparse::schedule::Day>>;
        type CronDowOption<'c> = Option<& 'c cronparse::interval::Interval<cronparse::schedule::DayOfWeek>>;
        type CronHrsOption<'d> = Option<& 'd cronparse::interval::Interval<cronparse::schedule::Hour>>;
        type CronMinsOptions<'e> = Option<& 'e cronparse::interval::Interval<cronparse::schedule::Minute>>;
        type CromMonth = cronparse::schedule::Month;
        let mut tasks_to_add: Vec<Task> = Vec::new();

        let crontab: CronFile = CronEntries::new(path_to_crontab).expect("error parsing file!");
        for entry in crontab{
            if let Ok(entry_ok) = entry{
                if let Some(cal) = entry_ok.calendar(){
                    let mut task: Task = Task::new();
                    if let Some(command) = entry_ok.command(){
                        task.command = Some(command.to_string());
                    }
                   let cron_month: CronMonthOption  = cal.mons.0.first();
                   if let Some(month) = cron_month{
                        match month{
                            cronparse::interval::Interval::Value(month_parsed) => {
                                let month_cronie: CromMonth = month_parsed.to_owned();
                                match month_cronie{
                                    CromMonth::January => task.month = Some(1),
                                    CromMonth::February => task.month = Some(2),
                                    CromMonth::March => task.month = Some(3),
                                    CromMonth::April => task.month = Some(4),
                                    CromMonth::May => task.month = Some(5),
                                    CromMonth::June => task.month = Some(6),
                                    CromMonth::July => task.month = Some(7),
                                    CromMonth::August => task.month = Some(8),
                                    CromMonth::September => task.month = Some(9),
                                    CromMonth::October => task.month = Some(10),
                                    CromMonth::November => task.month = Some(11),
                                    CromMonth::December => task.month = Some(12),
                                }
                            }
                            cronparse::interval::Interval::Range(_, _, _) => {}
                            cronparse::interval::Interval::Full(_) => {}
                        }
                   }
                   let day_of_month: CronDomOption = cal.days.0.first();
                   if let Some(dom_cronie) = day_of_month{
                    match dom_cronie{
                        cronparse::interval::Interval::Value(dom) => {
                            let dom: i32 = dom.0 as i32;
                            task.day_of_month = Some(dom);
                        }
                        cronparse::interval::Interval::Range(_, _, _) => {}
                        cronparse::interval::Interval::Full(_) => todo!{}
                    }
                   }
                   let day_of_week: CronDowOption = cal.dows.0.first();
                   if let Some(dow_cronie) = day_of_week{
                    match dow_cronie{
                        cronparse::interval::Interval::Value(dow) => {
                            let dow = dow.to_owned();
                            match dow {
                                cronparse::schedule::DayOfWeek::Sunday => task.day_of_week = Some(1),
                                cronparse::schedule::DayOfWeek::Monday => task.day_of_week = Some(2),
                                cronparse::schedule::DayOfWeek::Tuesday => task.day_of_week = Some(3),
                                cronparse::schedule::DayOfWeek::Wednesday => task.day_of_week = Some(4),
                                cronparse::schedule::DayOfWeek::Thursday => task.day_of_week = Some(5),
                                cronparse::schedule::DayOfWeek::Friday => task.day_of_week = Some(6),
                                cronparse::schedule::DayOfWeek::Saturday => task.day_of_week = Some(7),
                            }
                        }
                        cronparse::interval::Interval::Range(_, _, _) => {}
                        cronparse::interval::Interval::Full(_) => {}
                    }
                   }
                   let hour: CronHrsOption = cal.hrs.0.first();
                   if let Some(hrs_cronie) = hour {
                    match hrs_cronie {
                        cronparse::interval::Interval::Value(hour) => {
                            let hour: i32 = hour.0 as i32;
                            task.hour = Some(hour);
                        }
                        cronparse::interval::Interval::Range(_, _, _) => {}
                        cronparse::interval::Interval::Full(_) => {}
                    }
                   }
                   let minute: CronMinsOptions = cal.mins.0.first();
                   if let Some(mins_cronie) = minute {
                       match mins_cronie{
                        cronparse::interval::Interval::Value(minutes) => {
                            let minute:i32 = minutes.0 as i32;
                            task.minute = Some(minute);
                        }
                        cronparse::interval::Interval::Range(_, _, _) => {}
                        cronparse::interval::Interval::Full(_) => {}
                    }
                   }
                   tasks_to_add.push(task);
                }
            }
        }
        return tasks_to_add;
    }
}