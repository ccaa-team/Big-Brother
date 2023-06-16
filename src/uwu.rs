use rand::prelude::*;

const EMOJIS_SIZE: usize = 11;

static EMOJIS: [&str; EMOJIS_SIZE] = [
    " OwO",
    " UwU",
    " >w<",
    " :3",
    " o///o",
    " nyaa\\~\\~",
    "~",
    " >_<",
    " uguu..,",
    " -.-",
    " 〜☆"
];

fn random_emoji() -> String {
    let idx = rand::thread_rng().gen_range(1..EMOJIS.len());
    EMOJIS[idx].to_string()
}

static VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];

fn uwu_word(word: &str) -> Option<String> {
    if word.is_empty() {
        return None;
    } else if word.starts_with("http") {
        return Some(word.to_string());
    };

    let last_char = word.chars().last().unwrap();

    let mut out = word.replace(['l', 'r'], "w");

    for vowel in VOWELS.iter() {
        let mut from = String::from("n");
        let mut to = String::from("ny");

        from.push(*vowel);
        to.push(*vowel);

        out = out.replace(&from, &to);
    }

    let end = {
        if rand::thread_rng().gen_ratio(1, 6) {
            random_emoji()
        } else {
            "".to_string()
        }
    };

    let first_char = out.chars().next().unwrap();

    if out.len() > 2 && first_char.is_alphanumeric() && rand::thread_rng().gen_ratio(1, 3) {
        let mut tmp = String::from("");
        tmp.push(first_char);
        tmp.push('-');
        for chr in out.chars() {
            tmp.push(chr);
        }
        out = tmp;
    }

    Some(out + &end)
}

pub fn uwuify(text: String) -> String {
    let low = text.to_lowercase();

    low.split(' ')
        .map(uwu_word)
        .fold(String::new(), |a, b| a + " " + &b.unwrap_or("".to_string()))
}
