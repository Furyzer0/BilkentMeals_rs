use chrono::{Date, Datelike, Local, Weekday};
use scraper::ElementRef;
use std::fmt::{Display, Formatter};

use mealscraper::{extract_meal_names, scrap_alt, scrap_date, scrap_fix};

pub struct DailyMeal {
    pub date: Date<Local>,
    pub day: Weekday,
    pub lunch: Vec<(String, String)>,
    pub dinner: Vec<(String, String)>,
    pub alternative: Vec<(String, String)>,
}

pub struct Week(Vec<DailyMeal>);

impl Week {
    pub fn from_table(table: &ElementRef) -> Result<Self, &'static str> {
        let fix = scrap_fix(table).ok_or("failed to scrap fix table")?;
        // println!("fix scrapped");
        let alt = scrap_alt(table).ok_or("failed to scrap alt table")?;
        // println!("alt scrapped");
        let date = scrap_date(table).ok_or("failed to scrap date")?;
        // println!("date: {}", &date);
        let meals: Result<_, &'static str> = (1..8).map(|i| {
            let day = date.weekday();
            let lunch = extract_meal_names(&fix[(2 * i) - 1]).ok_or("failed on lunch")?;
            let dinner = extract_meal_names(&fix[2 * i]).ok_or("failed on dinner")?;
            let alternative = extract_meal_names(&alt[i]).ok_or("failed on alternative")?;
            Ok(DailyMeal {
                date,
                day,
                lunch,
                dinner,
                alternative,
            })
        }).collect();
        Ok(Week(meals?))
    }

    pub fn daily_from_number(&self, day: usize) -> Option<&DailyMeal> {
        self.0.get(day)
    }

    pub fn daily_from_weekday(&self, day: Weekday) -> &DailyMeal {
        self.0.get(day.num_days_from_monday() as usize).expect(&format!("expected day {:#?}", day))
    }
}

impl Display for DailyMeal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "day {:?} {}", self.day, self.date)?;
        for (tr, en) in self.lunch.iter() {
            writeln!(f, "{} / {}", tr, en)?;
        }
        Ok(())
    }
}