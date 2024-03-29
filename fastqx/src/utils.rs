//! file: utils.rs
//! author: Jacob Xie
//! date: 2023/09/13 17:29:15 Wednesday
//! brief:

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

// ================================================================================================
// Pub helpers
// ================================================================================================

// pub fn gen_local_datetime_unchecked(
//     year: i32,
//     month: u32,
//     day: u32,
//     hour: u32,
//     minute: u32,
//     second: u32,
// ) -> DateTime<Local> {
//     Local
//         .from_local_datetime(&gen_naive_datetime_unchecked(
//             year, month, day, hour, minute, second,
//         ))
//         .unwrap()
// }

// pub fn gen_local_datetime_str_unchecked(
//     year: i32,
//     month: u32,
//     day: u32,
//     hour: u32,
//     minute: u32,
//     second: u32,
// ) -> String {
//     gen_local_datetime_unchecked(year, month, day, hour, minute, second).to_rfc3339()
// }

// pub fn gen_current_local_datetime_str_unchecked() -> String {
//     let local: DateTime<Local> = Local::now();

//     local.to_rfc3339()
// }

// pub fn gen_naive_date_unchecked(year: i32, month: u32, day: u32) -> NaiveDate {
//     NaiveDate::from_ymd_opt(year, month, day).unwrap()
// }

// pub fn gen_naive_time_unchecked(hour: u32, min: u32, sec: u32) -> NaiveTime {
//     NaiveTime::from_hms_opt(hour, min, sec).unwrap()
// }

// pub fn gen_naive_datetime_unchecked(
//     year: i32,
//     month: u32,
//     day: u32,
//     hour: u32,
//     minute: u32,
//     second: u32,
// ) -> NaiveDateTime {
//     NaiveDate::from_ymd_opt(year, month, day)
//         .unwrap()
//         .and_hms_opt(hour, minute, second)
//         .unwrap()
// }

// ================================================================================================
// ChronoHelper
// ================================================================================================

pub struct ChronoHelper;

impl ChronoHelper {
    // "20240101.092500"
    pub fn gen_local_datetime_unchecked(&self, s: &str) -> DateTime<Local> {
        Local
            .from_local_datetime(&self.gen_naive_datetime_unchecked(s))
            .unwrap()
    }

    // "20240101"
    pub fn gen_naive_date_unchecked(&self, s: &str) -> NaiveDate {
        let year = s[0..4].parse().unwrap();
        let month = s[4..6].parse().unwrap();
        let day = s[6..8].parse().unwrap();
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    // "092500"
    pub fn gen_naive_time_unchecked(&self, s: &str) -> NaiveTime {
        let hour = s[..2].parse().unwrap();
        let minute = s[2..4].parse().unwrap();
        let second = s[4..].parse().unwrap();
        NaiveTime::from_hms_opt(hour, minute, second).unwrap()
    }

    // "20240101.092500"
    pub fn gen_naive_datetime_unchecked(&self, s: &str) -> NaiveDateTime {
        let year = s[..4].parse().unwrap();
        let month = s[4..6].parse().unwrap();
        let day = s[6..8].parse().unwrap();
        // split by '.'
        let hour = s[9..11].parse().unwrap();
        let minute = s[11..13].parse().unwrap();
        let second = s[13..].parse().unwrap();
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }
}
