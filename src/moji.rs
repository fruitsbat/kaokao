use emojis;

pub struct Moji {
    pub value: String,
    pub description: String,
}

impl Moji {
    pub fn from_emoji(emoji: &emojis::Emoji) -> Self {
        Moji {
            value: emoji.as_str().into(),
            description: emoji.name().into(),
        }
    }
}

pub fn moji_to_string(moji: &Vec<Moji>) -> String {
    let mut s = String::new();
    for m in moji {
        s.push_str(&format!(
            "{}{} {}",
            if s.len() == 0 { "" } else { "\n" },
            m.value,
            m.description
        ));
    }
    s
}

pub fn get_moji_list() -> Vec<Moji> {
    let mojis = emojis::iter()
        .map(|e| Moji::from_emoji(e))
        .collect::<Vec<Moji>>();
    mojis
}
