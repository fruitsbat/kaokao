use emojis::SkinTone;

/// adds additional info to a unicode description
pub fn get_description(emoji: &emojis::Emoji) -> String {
    let default_emoji = emoji.with_skin_tone(SkinTone::Default).unwrap_or(emoji);
    let additional_description = match default_emoji.name() {
        "person in suit levitating" => Some("ska guy"),
        _ => None,
    };
    match additional_description {
        None => emoji.name().into(),
        Some(value) => format!("{}, {}", emoji.name(), value),
    }
}
