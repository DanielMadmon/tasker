//! the crate main DataType:
//! Task struct is used for editing/adding/removing/etc whilst it's encrypted version is used for
//! writing to our local storage(it's a lot easier to operate on non encrypted structs)
//! the same logic is applied to the LogEntry struct and it's encrypted version.
//! CurrentTime as it's name suggests is added by default to every new task/log for logging and for execution
//! logic.

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub name: Option<String>,
    pub shell: Option<String>,
    pub command: Option<String>,
    pub comment: Option<String>,
    pub month: Option<i32>,
    pub day_of_month: Option<i32>,
    pub day_of_week: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
    pub year_added: i32,
    pub month_added: i32,
    pub day_of_month_added: i32,
    pub hour_added: i32,
    pub minute_added: i32,
}
#[derive(Clone, Debug)]
pub struct TaskEncrypted {
    pub name: Vec<u8>,
    pub shell: Vec<u8>,
    pub command: Vec<u8>,
    pub comment: Vec<u8>,
    pub month: Option<Vec<u8>>,
    pub day_of_month: Option<Vec<u8>>,
    pub day_of_week: Option<Vec<u8>>,
    pub hour: Option<Vec<u8>>,
    pub minute: Option<Vec<u8>>,
    pub year_added: Vec<u8>,
    pub month_added: Vec<u8>,
    pub day_of_month_added: Vec<u8>,
    pub hour_added: Vec<u8>,
    pub minute_added: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub name: String,
    pub command: String,
    pub output: Option<String>,
    pub execution_year: i32,
    pub execution_month: i32,
    pub execution_week: i32,
    pub execution_day_of_month: i32,
    pub execution_day_of_week: i32,
    pub execution_hour: i32,
    pub execution_minute: i32,
}
pub struct LogEntryEncrypted {
    pub name: Vec<u8>,
    pub command: Vec<u8>,
    pub output: Option<Vec<u8>>,
    pub execution_year: Vec<u8>,
    pub execution_month: Vec<u8>,
    pub execution_week: Vec<u8>,
    pub execution_day_of_month: Vec<u8>,
    pub execution_day_of_week: Vec<u8>,
    pub execution_hour: Vec<u8>,
    pub execution_minute: Vec<u8>,
}

#[derive(Debug)]
pub struct CurrentTime {
    pub year: i32,
    pub month: i32,
    pub day_of_month: i32,
    pub week: i32,
    pub day_of_week: chrono::Weekday,
    pub day_of_week_num: i32,
    pub hour: i32,
    pub min: i32,
}

