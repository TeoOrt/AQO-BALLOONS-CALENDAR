use std::collections::HashMap;

/*

Day Container:
@Input : Week enums which is Monday...Sunday
@Output: Hourly Schedule hashmap
@Functions new() -> creates 24 hour hashmap and sets the type of day
get_booking which returns if hour is booked
set_hour
 */

pub struct DayContainer {
    hours: HashMap<u32, bool>, // Bool is to check if the hour is empty or not
    day_num: u32,
}

impl DayContainer {
    pub fn new(day_num: u32) -> Self {
        let mut hours: HashMap<u32, bool> = HashMap::default();
        for hour in 0..25 {
            hours.insert(hour, false);
        }

        Self {
            hours: hours,
            day_num,
        }
    }
    pub fn get_booking(&self, hour: u32) -> bool {
        if self.hours[&hour] {
            return true;
        } else {
            false
        }
    }

    pub fn set_hour(&mut self, set_schedule: bool, hour: u32) -> bool {
        if self.get_booking(hour) {
            return false;
        } else {
            *self.hours.get_mut(&hour).unwrap() = set_schedule;

            return true;
        }
    }

    pub fn get_day(&self) -> String {
        self.day_num.to_string()
    }
}
