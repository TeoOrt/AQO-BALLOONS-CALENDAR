// use super::DayContainer;

// #[derive(Clone)]
// pub enum Week {
//     Monday = 1,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday,
// }

// impl Week {
//     pub fn new(weekday: u32) -> Result<Self, String> {
//         match weekday {
//             1 => Ok(Week::Monday),
//             2 => Ok(Week::Tuesday),
//             3 => Ok(Week::Wednesday),
//             4 => Ok(Week::Thursday),
//             5 => Ok(Week::Friday),
//             6 => Ok(Week::Saturday),
//             7 => Ok(Week::Sunday),
//             _ => Err(format!("Invalid weekday value: {}", weekday)),
//         }
//     }

//     // pub fn week_type(&self) -> u32 {
//     //     match self {
//     //         Week::Monday => 1,
//     //         Week::Tuesday => 2,
//     //         Week::Wednesday => 3,
//     //         Week::Thursday => 4,
//     //         Week::Friday => 5,
//     //         Week::Saturday => 6,
//     //         Week::Sunday => 7,
//     //     }
//     // }

//     pub fn to_str(&self) -> &str {
//         match self {
//             Week::Monday => "Mon",
//             Week::Tuesday => "Tues",
//             Week::Wednesday => "Wed",
//             Week::Thursday => "Thurs",
//             Week::Friday => "Fri",
//             Week::Saturday => "Sat",
//             Week::Sunday => "Sun",
//         }
//     }

//     // pub fn iter() -> impl Iterator<Item = Week> {
//     //     [
//     //         Week::Monday,
//     //         Week::Tuesday,
//     //         Week::Wednesday,
//     //         Week::Thursday,
//     //         Week::Friday,
//     //         Week::Saturday,
//     //         Week::Sunday,
//     //     ]
//     //     .iter()
//     //     .cloned()
//     // }
// }

// /*
// @function: Create a list of days and contain them within a week
// Week num is important to know the day of the week
// reason for vector is that it is easier to implement
// o(n)

// @input: week_num
// @function, setter and getter for weekday
// @since it inherets days, it can do whatever it wants with DayContainerss
//  */
// pub struct WeekContainer {
//     days: Vec<DayContainer>,
//     week_num: u32,
// }

// impl WeekContainer {
//     pub fn new(week_num: u32, first_monday: u32) -> Self {
//         let mut days = Vec::default();
//         //weekdays starting from monday
//         for day in 1..8 {
//             let mut i = day - 1;
//             days.push(DayContainer::new(first_monday + i));
//         }
//         Self { days, week_num }
//     }

//     pub fn access_day(&self, position: usize) -> bool {
//         println!(
//             "{} ,{}",
//             self.days[position].get_day_type(),
//             self.days[position].get_day()
//         );

//         true
//     }
// }
