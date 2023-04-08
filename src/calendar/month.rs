use super::DayContainer;

#[allow(dead_code)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn new(month: u32) -> Result<Self, String> {
        match month {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(format!("Invalid month value: {}", month)),
        }
    }

    pub fn days_in_month(&self) -> u32 {
        match self {
            Month::January => 31,
            Month::February => 28,
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    pub fn month(&self) -> u32 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}

pub struct MonthContainer {
    day_storage: Vec<DayContainer>,
    month: Month,
}

impl MonthContainer {
    pub fn new(month: Month) -> Self {
        let mut day_storage = Vec::default();
        for days in 0..month.days_in_month() {
            day_storage.push(DayContainer::new(days));
        }
        Self { day_storage, month }
    }

    pub fn schedule(&mut self, day: usize, time: u32) -> String {
        if self.day_storage[day].set_hour(true, time) {
            String::from("Your time has been scheduled at for day {} \n you will be receiving an email shortly thank you \n")
        } else {
            String::from(
                "That hour has been booked try another hour .. might give recommend hour \n",
            )
        }
    }
}
