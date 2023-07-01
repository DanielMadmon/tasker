use std::rc::Rc;
use slint::{Model, VecModel, SharedString};
use tasker_lib::taskerctl::{read_tasks_db,add_task,Task};

struct TaskFmt{
    name:String,
    shell:String,
    command:String,
    comment:String,
    month:Option<i32>,
    day_of_month:Option<i32>,
    day_of_week:Option<i32>,
    hour:Option<i32>,
    minute:Option<i32>
}
impl TaskFmt{
    fn new()->Self{
        Self{
            name: "".to_string(),
            shell: "".to_string(),
            command: "".to_string(),
            comment: "".to_string(),
            month: None,
            day_of_month: None,
            day_of_week: None,
            hour: None,
            minute: None,
        }
    }
}
slint::slint!{
    import { App } from "bin/tasker_gui/src/main_window.slint";
}
pub fn main(){ 
    
    let app = App::new().unwrap();
    let mut rows:Vec<TableRow> = app.get_table().iter().collect();

    let current_table_db: Vec<TaskFmt> = get_tasks_fmt();
    rows.extend(rows.clone());
    for task in current_table_db{
        let  name:SharedString = SharedString::from(task.name);
        let  shell:SharedString =SharedString::from(task.shell);
        let  command:SharedString = SharedString::from(task.command);
        let  comment:SharedString = SharedString::from(task.comment);
        let mut month:SharedString = SharedString::new();
        let mut day_of_month:SharedString = SharedString::new();
        let mut day_of_week:SharedString = SharedString::new();
        let mut hour:SharedString = SharedString::new();
        let mut minute: SharedString = SharedString::new();
        match task.month{
            Some(task_month)=>{
                month = SharedString::from(task_month.to_string());
            }
            None=>{
                month = SharedString::from("none");
            }
        }
        match task.day_of_month{
            Some(task_dom)=>{
                day_of_month = SharedString::from(task_dom.to_string());
            }
            None=>{
                day_of_month = SharedString::from("none");
            }
        }
        match task.day_of_week{
            Some(task_dow)=>{
                day_of_week = SharedString::from(task_dow.to_string());
            }
            None=>{
                day_of_week = SharedString::from("none");
            }
        }
        match task.hour{
            Some(task_hour)=>{
                hour = SharedString::from(task_hour.to_string());
            }
            None=>{
                hour = SharedString::from("none");
            }
        }
        match task.minute{
            Some(task_minute)=>{
                minute = SharedString::from(task_minute.to_string());
            }
            None=>{
                minute = SharedString::from("none");
            }
        }
        let new_row = TableRow{
            name,
            shell,
            command,
            comment,
            month,
            day_of_month,
            day_of_week,
            hour,
            minute
        };
        rows.push(new_row);
    }
    if !rows.is_empty(){
        let table_model: Rc<VecModel<TableRow>> = Rc::new(VecModel::from(rows));
        app.set_table(table_model.into());
        app.run().unwrap();
    }else{
        app.run().unwrap();
    }
}
fn get_tasks_fmt() -> Vec<TaskFmt>{
    let current_tasks: Vec<Task> = read_tasks_db();
    let mut current_tasks_fmt: Vec<TaskFmt> = Vec::new();
    for task in current_tasks{
        let mut task_fmt: TaskFmt = TaskFmt::new();
        if let Some(name) = task.name{
            task_fmt.name = name;
        }
        if let Some(shell) = task.shell{
            task_fmt.shell = shell;
        }
        if let Some(command) = task.command{
            task_fmt.command = command;
        }
        if let Some(comment) = task.comment{
            task_fmt.comment = comment;
        }
        if let Some(month) = task.month{
            task_fmt.month = Some(month);
        }
        if let Some(day_of_month) = task.day_of_month{
            task_fmt.day_of_month = Some(day_of_month);
        }
        if let Some(day_of_week) = task.day_of_week{
            task_fmt.day_of_week = Some(day_of_week);
        }
        if let Some(hour) = task.hour {
            task_fmt.hour = Some(hour);
        }
        if let Some(minute) = task.minute{
            task_fmt.minute = Some(minute);
        }
        current_tasks_fmt.push(task_fmt);
    }
    current_tasks_fmt
}
fn set_tasks(){

}
