use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone, Copy)]
pub struct MonthInfo {
    pub month: u32,
    pub name: &'static str,
    pub short_name: &'static str,
    pub days: u32,
}

impl MonthInfo {
    pub fn from_month(month: u32) -> Self {
        let (name, short_name, days) = match month {
            1 => ("January", "Jan", 31),
            2 => ("February", "Feb", 28),
            3 => ("March", "Mar", 31),
            4 => ("April", "Apr", 30),
            5 => ("May", "May", 31),
            6 => ("June", "Jun", 30),
            7 => ("July", "Jul", 31),
            8 => ("August", "Aug", 31),
            9 => ("September", "Sep", 30),
            10 => ("October", "Oct", 31),
            11 => ("November", "Nov", 30),
            12 => ("December", "Dec", 31),
            _ => ("", "", 0),
        };
        MonthInfo {
            month,
            name,
            short_name,
            days,
        }
    }

    pub fn from_date(date: NaiveDate) -> Self {
        Self::from_month(date.month())
    }

    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn days_in_month(month: u32, year: i32) -> u32 {
        if month == 2 && Self::is_leap_year(year) {
            29
        } else {
            Self::from_month(month).days
        }
    }
}

const DAYS_IN_WEEK: i64 = 7;

#[derive(Debug, Clone)]
pub struct WeekLayout {
    pub dates: Vec<NaiveDate>,
    pub month_start_idx: Option<(usize, u32)>,
    pub month_end_idx: Option<(usize, u32)>,
    pub year_boundary_idx: Option<usize>,
}

impl WeekLayout {
    pub fn new(start_date: NaiveDate) -> Self {
        let dates: Vec<NaiveDate> = (0..DAYS_IN_WEEK)
            .map(|day_offset| {
                start_date
                    .checked_add_signed(chrono::Duration::days(day_offset))
                    .unwrap()
            })
            .collect();

        let month_start_idx = dates
            .iter()
            .enumerate()
            .find(|(_, date)| date.day() == 1)
            .map(|(idx, date)| (idx, date.month()));

        let month_end_idx = Self::find_month_end(&dates);
        let year_boundary_idx = Self::find_year_boundary(&dates);

        WeekLayout {
            dates,
            month_start_idx,
            month_end_idx,
            year_boundary_idx,
        }
    }

    fn find_month_end(dates: &[NaiveDate]) -> Option<(usize, u32)> {
        for (idx, &date) in dates.iter().enumerate() {
            if idx < dates.len() - 1 {
                let next_date = dates[idx + 1];
                if date.month() != next_date.month() || date.year() != next_date.year() {
                    return Some((idx, date.month()));
                }
            }
        }
        None
    }

    fn find_year_boundary(dates: &[NaiveDate]) -> Option<usize> {
        for (idx, &date) in dates.iter().enumerate() {
            if idx > 0 {
                let prev_date = dates[idx - 1];
                if date.year() != prev_date.year() {
                    return Some(idx);
                }
            }
        }
        None
    }

    pub fn get_date(&self, idx: usize) -> Option<NaiveDate> {
        self.dates.get(idx).copied()
    }

    pub fn get_first_date(&self) -> NaiveDate {
        self.dates[0]
    }

    pub fn get_last_date(&self) -> NaiveDate {
        self.dates[self.dates.len() - 1]
    }

    pub fn contains_month_start(&self) -> bool {
        self.month_start_idx.is_some()
    }

    pub fn contains_month_end(&self) -> bool {
        self.month_end_idx.is_some()
    }

    pub fn is_in_current_month(&self, idx: usize, year: i32, current_month: Option<u32>) -> bool {
        if let Some(date) = self.get_date(idx) {
            date.year() == year && Some(date.month()) == current_month
        } else {
            false
        }
    }

    pub fn was_prev_in_month(&self, idx: usize, year: i32, current_month: Option<u32>) -> bool {
        if idx > 0 {
            if let Some(prev_date) = self.get_date(idx - 1) {
                return prev_date.year() == year && Some(prev_date.month()) == current_month;
            }
        }
        false
    }

    pub fn will_next_be_in_month(&self, idx: usize, year: i32, current_month: Option<u32>) -> bool {
        if idx < 6 {
            if let Some(next_date) = self.get_date(idx + 1) {
                return next_date.year() == year && Some(next_date.month()) == current_month;
            }
        }
        false
    }

    pub fn count_days_in_month(&self, month: u32) -> usize {
        self.dates.iter().filter(|d| d.month() == month).count()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpacingConfig {
    pub idx: usize,
    pub in_month: bool,
    pub prev_in_month: bool,
    pub next_in_month: bool,
    pub first_out_of_month: bool,
}

impl SpacingConfig {
    pub fn new(
        idx: usize,
        in_month: bool,
        prev_in_month: bool,
        next_in_month: bool,
        first_out_of_month: bool,
    ) -> Self {
        Self {
            idx,
            in_month,
            prev_in_month,
            next_in_month,
            first_out_of_month,
        }
    }

    pub fn is_last_in_week(&self) -> bool {
        self.idx >= 6
    }

    pub fn is_first_in_week(&self) -> bool {
        self.idx == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpacingCalculator;

impl SpacingCalculator {
    pub fn date_spacing(config: SpacingConfig) -> &'static str {
        match (
            config.is_last_in_week(),
            config.in_month,
            config.prev_in_month,
            config.next_in_month,
            config.first_out_of_month,
        ) {
            (true, true, _, _, _) => "   ",
            (true, false, _, _, _) => "",
            (false, true, false, _, _) if config.is_first_in_week() => "    ",
            (false, true, _, _, _) => "   ",
            (false, false, _, true, _) => "  ",
            (false, false, _, false, true) => "    ",
            _ => "   ",
        }
    }

    pub fn date_spacing_legacy(
        idx: usize,
        in_month: bool,
        prev_in_month: bool,
        next_in_month: bool,
        first_out_of_month: bool,
    ) -> &'static str {
        let config = SpacingConfig::new(
            idx,
            in_month,
            prev_in_month,
            next_in_month,
            first_out_of_month,
        );
        Self::date_spacing(config)
    }

    pub fn border_width_before(bar_idx: usize) -> usize {
        if bar_idx == 0 {
            0
        } else if bar_idx == 1 {
            5
        } else {
            6 + (bar_idx - 2) * 5 + 4
        }
    }

    pub fn border_width_after(bar_idx: usize) -> usize {
        (7 - bar_idx) * 5
    }
}

#[derive(Debug, Clone)]
pub struct BorderState {
    pub before_width: usize,
    pub after_width: usize,
    pub has_boundary: bool,
    pub boundary_position: Option<usize>,
}

impl BorderState {
    pub fn new(boundary_position: Option<usize>) -> Self {
        let (before_width, after_width, has_boundary) = if let Some(pos) = boundary_position {
            (
                SpacingCalculator::border_width_before(pos),
                SpacingCalculator::border_width_after(pos),
                true,
            )
        } else {
            (0, 0, false)
        };

        Self {
            before_width,
            after_width,
            has_boundary,
            boundary_position,
        }
    }

    pub fn total_width(&self) -> usize {
        self.before_width + self.after_width
    }
}
