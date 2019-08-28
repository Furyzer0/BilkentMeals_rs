extern crate chrono;
extern crate encoding;
extern crate reqwest;
extern crate scraper;

mod mealscraper;
pub mod models;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn it_works() {
        println!("{}", "Bu yazı türkçe karakterler içeriyor.");
    }

    #[test]
    fn test_get_table() {
        scrap_meals().map(|week| {
            let day1 = week.daily_from_weekday(Weekday::Mon);
            println!("{}", day1);
        }).unwrap()
    }
}

use mealscraper::{get_document, get_table};
use models::Week;

pub fn scrap_meals() -> Result<Week, &'static str> {
    let doc = get_document().ok_or("failed to get document")?;
    let table = get_table(&doc).ok_or("failed to scrap table")?;

    Week::from_table(&table)
}
