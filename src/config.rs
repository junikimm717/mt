use crate::time::{Time, RE, RE2};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use toml::from_str;

use chrono::prelude::*;
use chrono::Weekday;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    time: u32,
    browser: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    monday: Option<HashMap<String, String>>,
    tuesday: Option<HashMap<String, String>>,
    wednesday: Option<HashMap<String, String>>,
    thursday: Option<HashMap<String, String>>,
    friday: Option<HashMap<String, String>>,
    saturday: Option<HashMap<String, String>>,
    sunday: Option<HashMap<String, String>>,
}

impl Schedule {
    fn check_hashmap(mp: &Option<HashMap<String, String>>) -> Result<(), String> {
        if mp.is_some() {
            for s in mp.as_ref().unwrap().keys() {
                if !RE.is_match(s) && !RE2.is_match(s) {
                    return Err(s.clone());
                }
            }
        }
        Ok(())
    }
    /// check if all times on a schedule match regexes.
    pub fn check_schedule(&self) -> Result<(), String> {
        let days = vec![
            &self.monday,
            &self.tuesday,
            &self.wednesday,
            &self.thursday,
            &self.friday,
            &self.saturday,
            &self.sunday,
        ];
        for day in &days {
            let res = Schedule::check_hashmap(day);
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meeting {
    url: String,
    aliases: Option<Vec<String>>,
    monday: Option<String>,
    tuesday: Option<String>,
    wednesday: Option<String>,
    thursday: Option<String>,
    friday: Option<String>,
    saturday: Option<String>,
    sunday: Option<String>,
}

impl Meeting {
    /// get a URL for a meeting given the day of week.
    pub fn get_url(&self, w: &chrono::Weekday) -> String {
        let weekday_specific = match w {
            chrono::Weekday::Sun => &self.sunday,
            chrono::Weekday::Mon => &self.monday,
            chrono::Weekday::Tue => &self.tuesday,
            chrono::Weekday::Wed => &self.wednesday,
            chrono::Weekday::Thu => &self.thursday,
            chrono::Weekday::Fri => &self.friday,
            chrono::Weekday::Sat => &self.saturday,
        };
        if let Some(url) = weekday_specific {
            return url.clone();
        } else {
            return (&self.url).clone();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    settings: Settings,
    schedule: Schedule,
    meetings: Option<HashMap<String, Meeting>>,
}

impl Config {
    /// Creates a default configuration struct.
    pub fn default() -> Self {
        let mut sample_meeting = HashMap::new();
        sample_meeting.insert(
            String::from("sample"),
            Meeting {
                url: String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
                aliases: Some(vec!["s".to_string(), "sp".to_string()]),
                monday: None,
                tuesday: None,
                wednesday: None,
                thursday: None,
                friday: None,
                saturday: None,
                sunday: None,
            },
        );
        Config {
            settings: Settings {
                browser: (String::from("firefox")),
                time: (5),
            },
            schedule: Schedule {
                monday: Some(HashMap::default()),
                tuesday: Some(HashMap::default()),
                wednesday: Some(HashMap::default()),
                thursday: Some(HashMap::default()),
                friday: Some(HashMap::default()),
                saturday: Some(HashMap::default()),
                sunday: Some(HashMap::default()),
            },
            meetings: Some(sample_meeting),
        }
    }
    pub fn browser(&self) -> String {
        self.settings.browser.clone()
    }
    pub fn time_threshold(&self) -> u32 {
        self.settings.time
    }
    pub fn from(s: &str) -> Result<Self, toml::de::Error> {
        return from_str::<Config>(s);
    }

    /// vector of all meetings (and URLs) that are available today
    pub fn meetings_today(&self) -> Vec<(u32, String)> {
        let weekday = chrono::Local::now().weekday();
        let weekday_specific = match weekday {
            chrono::Weekday::Sun => &self.schedule.sunday,
            chrono::Weekday::Mon => &self.schedule.monday,
            chrono::Weekday::Tue => &self.schedule.tuesday,
            chrono::Weekday::Wed => &self.schedule.wednesday,
            chrono::Weekday::Thu => &self.schedule.thursday,
            chrono::Weekday::Fri => &self.schedule.friday,
            chrono::Weekday::Sat => &self.schedule.saturday,
        };
        if let Some(w) = &weekday_specific {
            return w
                .iter()
                .map(|x| (Time::from(x.0.clone()).to_int(), x.1.clone()))
                .collect();
        } else {
            return vec![];
        }
    }

    /// turns the schedule into a searchable hashmap for syntax checking
    pub fn schedule_to_hashmap(&self) -> HashMap<Weekday, HashMap<String, String>> {
        let mut res = HashMap::default();
        if let Some(x) = &self.schedule.monday {
            res.insert(Weekday::Mon, x.clone());
        }
        if let Some(x) = &self.schedule.tuesday {
            res.insert(Weekday::Tue, x.clone());
        }
        if let Some(x) = &self.schedule.wednesday {
            res.insert(Weekday::Wed, x.clone());
        }
        if let Some(x) = &self.schedule.thursday {
            res.insert(Weekday::Thu, x.clone());
        }
        if let Some(x) = &self.schedule.friday {
            res.insert(Weekday::Fri, x.clone());
        }
        if let Some(x) = &self.schedule.saturday {
            res.insert(Weekday::Sat, x.clone());
        }
        if let Some(x) = &self.schedule.sunday {
            res.insert(Weekday::Sun, x.clone());
        }
        return res;
    }
    /// turn aliases into a hashmap to URLs.
    pub fn aliases_to_hashmap(&self) -> HashMap<String, String> {
        let weekday = chrono::Local::now().weekday();
        let mut res = HashMap::<String, String>::default();
        if let Some(meetings) = self.meetings() {
            for (name, meeting) in meetings.iter() {
                res.insert(name.clone(), meeting.get_url(&weekday));
                if let Some(aliases) = &meeting.aliases {
                    for alias in aliases {
                        res.insert(alias.clone(), meeting.get_url(&weekday));
                    }
                }
            }
        }
        return res;
    }

    /// check if all meetings outlined in the schedule actually exist.
    fn check_meetings(&self) -> Result<(), (String, String)> {
        let mp = self.aliases_to_hashmap();
        let hs = mp.keys().collect::<HashSet<&String>>();
        let schedule = self.schedule_to_hashmap();
        for (weekday, map) in &schedule {
            for x in map.values() {
                if !hs.contains(x) {
                    return Err((weekday.to_string(), x.clone()));
                }
            }
        }
        Ok(())
    }

    /// check if aliases conflict with one another
    fn check_aliases(&self) -> Result<(), String> {
        let hs = HashSet::<String>::default();
        if let Some(mt) = &self.meetings {
            for v in mt.values() {
                if let Some(aliases) = &v.aliases {
                    for alias in aliases {
                        if hs.contains(alias) {
                            return Err(alias.clone());
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
    pub fn meetings(&self) -> &Option<HashMap<String, Meeting>> {
        &self.meetings
    }
    pub fn check_syntax(&self) {
        if let Err(s) = self.check_aliases() {
            eprintln!("Duplicated Alias '{}'", s);
            return;
        }
        if let Err((day, meeting)) = self.check_meetings() {
            eprintln!(
                "Invalid meeting at weekday {} for meeting name {}",
                day, meeting
            );
            return;
        }
        if let Err(s) = self.schedule().check_schedule() {
            eprintln!("Invalid Time regex {}", s);
            return;
        }
        eprintln!("No errors found.");
    }
}

#[test]
fn test_1() {
    println!(
        "{:?}",
        from_str::<Config>(&toml::to_string::<Config>(&Config::default()).unwrap()[..]).unwrap()
    );
}
