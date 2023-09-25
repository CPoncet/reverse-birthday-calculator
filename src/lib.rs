use chrono::{prelude::*, Duration, LocalResult};
use std::cmp::Ordering;

#[derive(Debug)]
struct Error;

pub struct AgeInput {
    pub years: String,
    pub months: String,
    pub days: String,
}

pub fn calc_birthday(age: &AgeInput, date: &String, leap: &bool) -> Result<DateTime<Utc>, String> {
    let splitted_date: Vec<&str> = date.split('/').collect::<Vec<&str>>();
    let date_year = splitted_date[0].parse().unwrap();
    let date_month = splitted_date[1].parse().unwrap();
    let date_day = splitted_date[2].parse().unwrap();

    let formatted_date_result: LocalResult<DateTime<Utc>> =
        Utc.with_ymd_and_hms(date_year, date_month, date_day, 0, 0, 0);

    let formatted_date = match formatted_date_result {
        LocalResult::Single(date) => date,
        LocalResult::Ambiguous(date1, date2) => {
            return Err(format!("Ambiguous result: {} or {}", date1, date2))
        }
        LocalResult::None => return Err("Wrong date".to_owned()),
    };

    let age_years: i32 = age.years.parse().unwrap();
    let age_months = age.months.parse().unwrap();
    let age_days: u32 = age.days.parse().unwrap();

    let year = formatted_date.year() - age_years;
    let month = substract_month(formatted_date.month(), age_months);

    let new_date_result = Utc.with_ymd_and_hms(
        year + month.1,
        month.0,
        formatted_date.day() - age_days,
        0,
        0,
        0,
    );

    let new_date = match new_date_result {
        LocalResult::Single(date) => date,
        LocalResult::Ambiguous(date1, date2) => {
            return Err(format!("Ambiguous result: {} or {}", date1, date2))
        }
        LocalResult::None => {
            return Ok(Utc
                .with_ymd_and_hms(
                    year + month.1,
                    month.0,
                    formatted_date.day() - (age_days + 1),
                    0,
                    0,
                    0,
                )
                .unwrap())
        }
    };

    if *leap {
        let mut num_leap = 0;

        let list_of_years: Vec<i32> = (year..formatted_date.year()).collect();

        for single_year in list_of_years {
            match is_leap_year(single_year) {
                true => num_leap = num_leap + 1,
                false => (),
            }
        }

        Ok(new_date - Duration::days(num_leap))
    } else {
        Ok(new_date)
    }
}

fn is_leap_year(year: i32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}

fn substract_month(date_month: u32, age_months: u32) -> (u32, i32) {
    match date_month.cmp(&age_months) {
        Ordering::Less => {
            let remainder = age_months - date_month;
            (12 - remainder, -1)
        }
        Ordering::Equal => (12, 0),
        Ordering::Greater => (date_month - age_months, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_birthday() {
        let age = AgeInput {
            years: "27".to_string(),
            months: "4".to_string(),
            days: "12".to_string(),
        };

        let date = String::from("1839/7/30");

        assert_eq!(
            calc_birthday(&age, &date, &false),
            Ok(Utc.with_ymd_and_hms(1812, 3, 18, 0, 0, 0).unwrap())
        )
    }

    #[test]
    fn test_invalid_final_date() {
        let age = AgeInput {
            years: "35".to_string(),
            months: "10".to_string(),
            days: "0".to_string(),
        };

        let date = String::from("1781/07/31");

        println!("calc: {:?}", calc_birthday(&age, &date, &false));

        assert_eq!(
            calc_birthday(&age, &date, &false),
            Ok(Utc.with_ymd_and_hms(1745, 09, 30, 0, 0, 0).unwrap())
        )
    }
}
