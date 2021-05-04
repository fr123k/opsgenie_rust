#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
    use chrono::format::ParseError;
    use chrono_tz::Europe::Berlin;
    use chrono_tz::Etc::UTC;
    use chrono::Datelike;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);

        let dt1 = DateTime::parse_from_rfc3339("2021-05-28T08:00:00Z").unwrap();
        let rfc3339 = DateTime::parse_from_rfc3339("2021-05-24T08:00:00Z").unwrap();
        let tz_aware = UTC.from_local_datetime(&rfc3339.naive_utc()).unwrap();
        println!("{}", rfc3339);
        // let naive_date_time = rfc3339.naive_utc();
        let berlin_time = tz_aware.with_timezone(&Berlin);
        println!("{}", berlin_time);

        println!("{}", berlin_time.weekday());

        let mut dt = rfc3339;
        while dt <= dt1 {
            if dt.weekday() == Weekday::Mon {
                println!("Week has started: {:?}", dt);
            }
            println!("date: {:?}, day: {:?}", dt, dt.weekday());
            dt = dt + Duration::days(1);
        }
    }
}