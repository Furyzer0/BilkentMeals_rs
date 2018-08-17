pub fn split(text: &str) -> (String, String) {
    let i = text
        .find('/')
        .expect(&format!("Couldn't find '/' in the text: {}", text));
    let (tr, en) = text.split_at(i);
    //Don't include '/'
    (String::from(tr.trim()), String::from(en[1..].trim()))
}

pub fn fix_spaces(text: &String) -> String {
    let mut result = String::new();
    for t in text.split_whitespace() {
        result.push_str(&format!("{} ", t));
    }
    String::from(result.trim())
}
