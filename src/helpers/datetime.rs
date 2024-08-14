use chrono::NaiveDateTime;

pub fn format(dt: Option<NaiveDateTime>) -> Option<String> {
    dt.map(|datetime| datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

#[cfg(test)]
mod datetime_test {
    use super::format;
    use chrono::{NaiveDateTime, Utc};

    #[test]
    fn format_datetime_test() {
        let time = NaiveDateTime::new(Utc::now().date_naive(), Utc::now().time());

        assert_eq!(
            format(Some(time)),
            Some(time.format("%Y-%m-%d %H:%M:%S").to_string())
        );
        assert_eq!(format(None), None);
    }
}
