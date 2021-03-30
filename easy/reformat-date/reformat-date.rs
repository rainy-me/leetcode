#![feature(fn_traits)]

use utils::setup;

setup!();

#[cfg(test)]
impl Solution {
    pub fn reformat_date(date: String) -> String {
        let mut parts = date.split_ascii_whitespace();
        let day_before = parts.next().unwrap();
        let day = &day_before[..day_before.len() - 2];
        let month = match parts.next().unwrap() {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => unreachable!(),
        };
        format!("{}-{}-{:0>2}", parts.next().unwrap(), month, day)
    }
}

test! {
    reformat_date,
    ("20th Oct 2052".to_string(),) => "2052-10-20";
    ("6th Jun 1933".to_string(),) => "1933-06-06";
    ("26th May 1960".to_string(),) =>  "1960-05-26";
}
