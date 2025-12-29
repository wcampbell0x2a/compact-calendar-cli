use crate::models::{DateDetail, DateRange};
use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CalendarConfig {
    #[serde(default)]
    pub dates: HashMap<String, RawDateDetail>,
    #[serde(default)]
    pub ranges: Vec<RawDateRange>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawDateDetail {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawDateRange {
    pub start: String,
    pub end: String,
    pub color: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl CalendarConfig {
    pub fn parse_dates(&self) -> HashMap<NaiveDate, DateDetail> {
        self.dates
            .iter()
            .filter_map(|(date_str, detail)| {
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .ok()
                    .map(|date| {
                        (
                            date,
                            DateDetail {
                                description: detail.description.clone(),
                                color: detail.color.clone(),
                            },
                        )
                    })
            })
            .collect()
    }

    pub fn parse_dates_for_year(&self, year: i32) -> HashMap<NaiveDate, DateDetail> {
        self.dates
            .iter()
            .flat_map(|(date_str, detail)| {
                if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    return vec![(
                        date,
                        DateDetail {
                            description: detail.description.clone(),
                            color: detail.color.clone(),
                        },
                    )];
                }
                if let Ok(md) =
                    chrono::NaiveDate::parse_from_str(&format!("{}-{}", year, date_str), "%Y-%m-%d")
                {
                    return vec![(
                        md,
                        DateDetail {
                            description: detail.description.clone(),
                            color: detail.color.clone(),
                        },
                    )];
                }

                vec![]
            })
            .collect()
    }

    pub fn parse_ranges(&self) -> Vec<DateRange> {
        self.ranges
            .iter()
            .filter_map(|range| {
                let start = NaiveDate::parse_from_str(&range.start, "%Y-%m-%d").ok()?;
                let end = NaiveDate::parse_from_str(&range.end, "%Y-%m-%d").ok()?;
                Some(DateRange {
                    start,
                    end,
                    color: range.color.clone(),
                    description: range.description.clone(),
                })
            })
            .collect()
    }

    pub fn parse_ranges_for_year(&self, year: i32) -> Vec<DateRange> {
        self.ranges
            .iter()
            .filter_map(|range| {
                if let (Ok(start), Ok(end)) = (
                    NaiveDate::parse_from_str(&range.start, "%Y-%m-%d"),
                    NaiveDate::parse_from_str(&range.end, "%Y-%m-%d"),
                ) {
                    return Some(DateRange {
                        start,
                        end,
                        color: range.color.clone(),
                        description: range.description.clone(),
                    });
                }
                if let (Ok(start), Ok(end)) = (
                    NaiveDate::parse_from_str(&format!("{}-{}", year, &range.start), "%Y-%m-%d"),
                    NaiveDate::parse_from_str(&format!("{}-{}", year, &range.end), "%Y-%m-%d"),
                ) {
                    return Some(DateRange {
                        start,
                        end,
                        color: range.color.clone(),
                        description: range.description.clone(),
                    });
                }

                None
            })
            .collect()
    }
}
