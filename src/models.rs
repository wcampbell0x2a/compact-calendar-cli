use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DateDetail {
    pub description: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub color: String,
    pub description: Option<String>,
}

pub struct Calendar {
    pub year: i32,
    pub week_starts_monday: bool,
    pub no_dim_weekends: bool,
    pub work_mode: bool,
    pub no_strikethrough_past: bool,
    pub details: HashMap<NaiveDate, DateDetail>,
    pub ranges: Vec<DateRange>,
}

impl Calendar {
    pub fn new(
        year: i32,
        week_starts_monday: bool,
        no_dim_weekends: bool,
        work_mode: bool,
        no_strikethrough_past: bool,
        details: HashMap<NaiveDate, DateDetail>,
        ranges: Vec<DateRange>,
    ) -> Self {
        Calendar {
            year,
            week_starts_monday,
            no_dim_weekends,
            work_mode,
            no_strikethrough_past,
            details,
            ranges,
        }
    }

    pub fn get_weekday_num(&self, date: NaiveDate) -> u32 {
        if self.week_starts_monday {
            date.weekday().num_days_from_monday()
        } else {
            date.weekday().num_days_from_sunday()
        }
    }
}
