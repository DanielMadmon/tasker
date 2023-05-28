use notify_rust::{self, Notification};

pub(crate) fn notify(task_name:&String)->Result<notify_rust::NotificationHandle, notify_rust::error::Error>{
    let res_cat = "executed task, name:".to_string() + &task_name;
    let full_body = res_cat.as_str();
    let noti: Result<notify_rust::NotificationHandle, notify_rust::error::Error> =  Notification::new()
    .summary("Tasker Service")
    .body(full_body)
    .show();
    noti

}