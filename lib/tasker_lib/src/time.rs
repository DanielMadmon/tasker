use chrono::{DateTime, Datelike, Local, Timelike, Weekday};

use crate::structs::CurrentTime;

impl CurrentTime {
    pub fn now() -> Self {
        let full_date_time: DateTime<Local> = Local::now();
        let year: i32 = full_date_time.year().try_into().unwrap();
        let month = full_date_time.month() as i32;
        let day_of_month = full_date_time.day() as i32;
        let week = full_date_time.iso_week().week() as i32;
        let day_of_week = full_date_time.weekday();
        let day_of_week_num: i32 = super::time::CurrentTime::convert_weekday_to_int(day_of_week);
        let hour = full_date_time.hour() as i32;
        let min: i32 = full_date_time.minute() as i32;

        CurrentTime {
            year,
            month,
            day_of_month,
            week,
            day_of_week,
            day_of_week_num,
            hour,
            min,
        }
    }
    #[allow(unused_assignments)]
    fn convert_weekday_to_int(day: Weekday) -> i32 {
        let mut day_int: i32 = 0;
        match &day {
            Weekday::Sun => day_int = 1,
            Weekday::Mon => day_int = 2,
            Weekday::Tue => day_int = 3,
            Weekday::Wed => day_int = 4,
            Weekday::Thu => day_int = 5,
            Weekday::Fri => day_int = 6,
            Weekday::Sat => day_int = 7,
        }
        day_int
    }
}
