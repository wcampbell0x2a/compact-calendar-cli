pub mod config;
pub mod formatting;
pub mod models;
pub mod rendering;

use config::CalendarConfig;
use models::{Calendar, CalendarOptions};
use std::fs;
use std::path::PathBuf;

pub fn load_config(config_path: &PathBuf) -> CalendarConfig {
    if !config_path.exists() {
        eprintln!(
            "Config file not found at {:?}, using empty configuration",
            config_path
        );
        return CalendarConfig {
            dates: Default::default(),
            ranges: Default::default(),
        };
    }

    let contents = fs::read_to_string(config_path).unwrap_or_else(|e| {
        eprintln!("Failed to read config file {:?}: {}", config_path, e);
        std::process::exit(1);
    });

    toml::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("Failed to parse TOML config: {}", e);
        std::process::exit(1);
    })
}

pub fn build_calendar(year: i32, options: CalendarOptions, config: CalendarConfig) -> Calendar {
    let details = config.parse_dates_for_year(year);
    let ranges = config.parse_ranges_for_year(year);
    Calendar::new(year, options, details, ranges)
}
