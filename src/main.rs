extern crate opsgenie_rs;

use opsgenie_rs::apis::configuration::{ApiKey, Configuration};
use opsgenie_rs::apis::schedule_api::{/*list_schedules,*/ get_schedule_timeline};
use opsgenie_rs::models::GetScheduleTimelineResponse;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;

use csv::WriterBuilder;

use chrono::{DateTime, NaiveTime, TimeZone};
use chrono_tz::Europe::Berlin;
use chrono_tz::Etc::UTC;
use chrono_tz::Tz;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Shift {
    employee: String,
    start_date: DateTime<Tz>,
    end_date: DateTime<Tz>,
}

impl Shift {
    /// Creates a new Shift.
    fn new(employee: Option<String>, start_date: Option<String>, end_date: Option<String>) -> Option<Shift> {
        Some(Shift {employee: employee?,
                start_date: berlin_time(start_date?),
                end_date: berlin_time(end_date?),
        })
    }

    fn is_valid_shift(&mut self) -> bool {
        let valid_start_time:NaiveTime = NaiveTime::from_hms(18, 00, 00);
        let valid_end_time:NaiveTime = NaiveTime::from_hms(10, 00, 00);
        if  self.start_date.time() >= valid_start_time &&
            self.end_date.time() <= valid_end_time  { 
            return true
        }
        return false
    } 
}

fn berlin_time(date: String) -> DateTime<Tz> {
    let rfc3339 = DateTime::parse_from_rfc3339(&date.to_owned()).unwrap();
    let tz_aware = UTC.from_local_datetime(&rfc3339.naive_utc()).unwrap();
    return tz_aware.with_timezone(&Berlin);
}

fn main() {
    let api_key = env::var("OPSGENIE_APIKEY").unwrap();

    let configuration = Configuration {
        base_path: "https://api.eu.opsgenie.com".to_owned(),
        user_agent: None, //Some("OpenAPI-Generator/2.0.0/rust".to_owned()),
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: Some(ApiKey {
            prefix: Some("GenieKey".to_string()),
            key: api_key.to_string(),
        }),
    };

    // let schedules = list_schedules(&configuration, None);
    // println!("scheduleet: {:#?}", schedules);

    let timelines = get_schedule_timeline(&configuration, "ab27ac93-c7ab-45f6-b497-7ed4ad5e794a", Some("id"), Some(vec!["base".to_string(),"override".to_string()]), Some(2), Some("months"), Some("2021-04-21T00:00:00Z".to_string()));
    println!("timeline: {:#?}", timelines);

    let shifts = transform_to_map(timelines.unwrap()).unwrap();
    println!("map: {:#?}", shifts);

    println!("csv: {:#?}", transform_to_csv(shifts));
    // configuration: &configuration::Configuration, identifier: &str, identifier_type: Option<&str>, expand: Option<Vec<String>>, interval: Option<i32>, interval_unit: Option<&str>, date: Option<String>

    // let opsgenie = OpsGenie::new("api.opsgenie.com", "5a822714-d7e2-480a-834c-feacabd98629");
}

fn transform_to_map(response: GetScheduleTimelineResponse) -> Option<HashMap<String, Vec<Shift>>> {
    let data = response.data?.final_timeline?.rotations?;

    let mut shifts:HashMap<String, Vec<Shift>> = HashMap::new();
    for rotation in data {
        println!("{:#?}", rotation);
        match rotation.periods {
            Some(periods) => {
                for time_line_period in periods {
                    let name = time_line_period.recipient?.name?;
                    let values: &mut Vec<Shift> = match shifts.entry(name.clone()) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(Vec::new())
                    };
                    let mut shift = Shift::new(Some(name), time_line_period.start_date, time_line_period.end_date)?;
                    if shift.is_valid_shift() {
                        values.push(shift);
                    }
                }
            }
            None => {}
        }
    }
    Some(shifts)
}

fn transform_to_csv(shifts: HashMap<String, Vec<Shift>>) -> Option<String> {
    
    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .from_writer(vec![]);

    for (_, shifts) in shifts.into_iter() {
        for shift in shifts {
            wtr.serialize([shift.employee, shift.start_date.to_string(), shift.end_date.to_string()]).unwrap();
        }
    }
    return Some(String::from_utf8(wtr.into_inner().unwrap()).unwrap());
}