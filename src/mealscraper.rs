use chrono::{Date, Local, TimeZone};
use encoding::all::WINDOWS_1254; // for iso 8859-9 decoding
use encoding::{DecoderTrap, Encoding};
use reqwest;
use scraper::selector::Selector;
use scraper::{ElementRef, Html};
use utils;

use std::io::Read;

const URL: &str = "http://kafemud.bilkent.edu.tr/monu_tr.html";
pub fn get_document() -> Option<Html> {
    let mut resp = reqwest::get(URL).ok()?;
    assert!(resp.status().is_success(), "Return code wasn't 200");
    let mut buffer = Vec::new();
    resp.read_to_end(&mut buffer).unwrap();
    let decoded = WINDOWS_1254.decode(&buffer, DecoderTrap::Strict).unwrap();
    let document = Html::parse_document(&decoded);

    Some(document)
}

pub fn get_table(doc: &Html) -> Option<ElementRef> {
    let selector = Selector::parse(".icerik").unwrap();
    doc.select(&selector).next()
}

pub fn scrap_fix<'a>(icerik: &ElementRef<'a>) -> Option<Vec<ElementRef<'a>>> {
    let mut fix = Vec::new();
    let selector = Selector::parse("tr:nth-child(2) > td > table tr").unwrap();
    fix.extend(icerik.select(&selector));
    println!("fix length: {}", fix.len());
    Some(fix)
}

pub fn scrap_alt<'a>(icerik: &ElementRef<'a>) -> Option<Vec<ElementRef<'a>>> {
    let mut alt = Vec::new();
    let selector = Selector::parse("tr:nth-child(3) > td > table tr").unwrap();
    alt.extend(icerik.select(&selector));
    println!("alt length: {}", alt.len());
    Some(alt)
}

pub fn scrap_date(icerik: &ElementRef) -> Option<Date<Local>> {
    let year = 2018;    // To Do: Scrap it from data
    let selector = Selector::parse("p.style6 > b").unwrap();
    let p = icerik.select(&selector).next()?;
    let text = p.text().next()?.trim();
    let index = text.find('.').expect("Couldn't find");
    let (day, month) = text.split_at(index);

    Some(Local.ymd(year, month[1..].parse().ok()?, day.parse().ok()?))
}

pub fn extract_meal_names(block: &ElementRef) -> Option<Vec<(String, String)>> {
    // style18 class is the meal contents
    let selector = Selector::parse("td.style18").unwrap();
    let td = block.select(&selector).next()?;
    //assert_eq!(td.value().name(), "td");
    let doc = td.html();
    let lines = doc.split("<br>")
        .skip(if is_fix(&td) { 1 } else { 0 });
    let result = lines.map(|l| Html::parse_fragment(l))
        .map(|doc| {
            let text = parse_text(&doc);
            scrap_meal_names(&text)
        })
        .filter_map(|t| t)
        .collect();
    Some(result)
    /* for line in lines {
        let doc = Html::parse_fragment(line);
        let text = parse_text(&doc);
        if let Some(splitted) = scrap_meal_names(&text) {
            result.push(splitted);
        }
    } */
}

fn parse_text(elem: &Html) -> String {
    let selector = Selector::parse("*").unwrap();
    let text: String = elem.select(&selector)
        .take(1)    // Take first element only which is outmost
        .flat_map(|n| n.text())
        .flat_map(|s| s.split('\n'))
        .collect();

    text.split_whitespace()
        .map(|s| format!("{} ", s))
        .collect()
}

#[inline]
fn is_fix(block: &ElementRef) -> bool {
    block.text().any(|s| s.contains("Yemeği"))
}

fn scrap_meal_names(text: &String) -> Option<(String, String)> {
    if text.contains(|c| c != ' ') {
        let text = utils::fix_spaces(text);
        let seperator = "veya / or";
        if let Some(index) = text.find(seperator) {
            let (lo, hi) = text.split_at(index);
            let (tr1, en1) = utils::split(&lo);
            let (tr2, en2) = utils::split(&hi[seperator.len()..]); // Start with the end of the seperator
            Some((
                format!("{} {} {}", tr1, "veya", tr2),
                format!("{} {} {}", en1, "or", en2),
            ))
        } else {
            Some(utils::split(&text))
        }
    } else {
        None
    }
}