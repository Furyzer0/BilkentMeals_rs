use chrono::{Date, Datelike, Local, Weekday};
use scraper::ElementRef;

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
        println!("fix scrapped");
        let alt = scrap_alt(table).ok_or("failed to scrap alt table")?;
        println!("alt scrapped");
        let date = scrap_date(table).ok_or("failed to scrap date")?;
        println!("date: {}", &date);
        let mut meals = Vec::new();
        for i in 1..8 {
            let day = date.weekday();
            let lunch = extract_meal_names(&fix[(2 * i) - 1]).ok_or("failed on lunch")?;
            let dinner = extract_meal_names(&fix[2 * i]).ok_or("failed on dinner")?;
            let alternative = extract_meal_names(&alt[i]).ok_or("failed on alternative")?;
            let daily = DailyMeal {
                date,
                day,
                lunch,
                dinner,
                alternative,
            };
            meals.push(daily);
        }
        Ok(Week(meals))
    }

    pub fn daily_from_number(&self, day: usize) -> Option<&DailyMeal> {
        if day < 7 {
            Some(&self.0[day])
        } else {
            None
        }
    }

    pub fn daily_from_weekday(&self, day: Weekday) -> &DailyMeal {
        &self.0[day.num_days_from_monday() as usize]
    }
}
