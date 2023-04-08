use super::{week, Week, WeekContainer};

pub struct Year_Container {
    year_weeks: Vec<WeekContainer>,
    year: i32,
}

impl Year_Container {
    pub fn new(year: i32) -> Self {
        let mut year_weeks: Vec<WeekContainer> = Vec::default();
        for weeks in 0..53 {
            let first_monday_year = 2 + 7 * weeks;
            year_weeks.push(WeekContainer::new(weeks, first_monday_year));
        }

        Self {
            year_weeks: year_weeks,
            year: year,
        }
    }

    pub fn get_all_year(&self) -> &str {}
}
