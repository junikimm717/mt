use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq)]
enum AMPM {
    AM,
    PM,
}

lazy_static! {
    pub static ref RE: Regex = Regex::new(r"(1[0-2]|[0-9])\s*:\s*([0-5][0-9])\s*(AM|PM)").unwrap();
    pub static ref RE2: Regex = Regex::new(r"(1[0-2]|[0-9])\s*(AM|PM)").unwrap();
}

pub struct Time {
    hour: u32,
    minute: u32,
    ampm: AMPM,
}

impl Time {
    /// converts time to minutes since midnight
    pub fn to_int(&self) -> u32 {
        // if it's 12AM, it should be 0
        let hour = self.hour % 12 + if self.ampm == AMPM::PM { 12 } else { 0 };
        hour * 60
            + self.minute
            + match &self.ampm {
                AMPM::AM => 0,
                AMPM::PM => 12 * 60,
            }
    }
    pub fn now() -> u32 {
        let now = chrono::Local::now();
        (now.hour() * 60 + now.minute()) as u32
    }
    /// Time struct from a string
    pub fn from(s: String) -> Self {
        let mut modified_str = s.clone();
        // purge whitespace
        modified_str.retain(|c| !c.is_whitespace());
        // match regexes
        if RE.is_match(&modified_str) {
            // hour and minutes are both supplied.
            let captured = RE.captures_iter(&modified_str).next().unwrap();
            return Time {
                hour: (&captured[1]).parse::<u32>().unwrap(),
                minute: (&captured[2]).parse::<u32>().unwrap(),
                ampm: match &captured[3] {
                    "AM" => AMPM::AM,
                    "PM" => AMPM::PM,
                    _ => panic!("Not possible"),
                },
            };
        } else if RE2.is_match(&modified_str) {
            // only hour is supplied
            let captured = RE2.captures_iter(&modified_str).next().unwrap();
            return Time {
                hour: (&captured[1]).parse::<u32>().unwrap(),
                minute: 0 as u32,
                ampm: match &captured[2] {
                    "AM" => AMPM::AM,
                    "PM" => AMPM::PM,
                    _ => panic!("Not possible"),
                },
            };
        }
        panic!("Unparsable Time Regex")
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Time::from(String::from("9 : 30 AM")).to_int(),
        Time {
            hour: 9,
            minute: 30,
            ampm: AMPM::AM
        }
        .to_int()
    );
}

#[test]
fn test_2() {
    assert_eq!(
        Time::from(String::from("11:15 AM")).to_int(),
        Time {
            hour: 11,
            minute: 15,
            ampm: AMPM::AM
        }
        .to_int()
    );
}

#[test]
fn test_3() {
    assert_eq!(
        Time::from(String::from(" 12 PM ")).to_int(),
        Time {
            hour: 12,
            minute: 0,
            ampm: AMPM::PM
        }
        .to_int()
    );
}
