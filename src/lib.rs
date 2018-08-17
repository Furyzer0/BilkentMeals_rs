extern crate chrono;
extern crate encoding;
extern crate reqwest;
extern crate scraper;

mod mealscraper;
pub mod models;
mod utils;

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    #[test]
    fn it_works() {
        println!("{}", "Bu yazı türkçe karakterler içeriyor.");
    }

    #[test]
    fn test_get_table() {
        println!(
            "{}",
            match ::scrap_meals() {
                Ok(week) => {
                    let day1 = week.daily_from_weekday(Weekday::Tue);
                    println!("date: {}", day1.date);
                    for (tr, en) in &day1.lunch {
                        println!("{} - {}", tr, en);
                    }
                    "success"
                }
                Err(msg) => msg,
            }
        );
    }
}

use mealscraper::{get_document, get_table};
use models::Week;

pub fn scrap_meals() -> Result<Week, &'static str> {
    let doc = get_document().ok_or("failed to get document")?;
    let table = get_table(&doc).ok_or("failed to scrap table")?;

    Week::from_table(&table)
}
