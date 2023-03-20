/// adds additional info to a unicode description
pub fn get_description(unicode: &str) -> String {
    match unicode {
        "person in suit levitating" => format!("{}, ska guy", unicode),
        _ => unicode.into(),
    }
}
