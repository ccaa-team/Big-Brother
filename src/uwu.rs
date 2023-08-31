use rand::prelude::*;

const EMOJIS_SIZE: usize = 15;

static EMOJIS: [&str; EMOJIS_SIZE] = [
    " OwO",
    " UwU",
    " >w<",
    " :3",
    " o///o",
    " >///<",
    " nyaa\\~\\~",
    " >\\_<",
    " uguu..,",
    " -.-",
    " ^w^",
    " ^-^",
    " omo",
    "〜☆",
    "~",
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

    let mut out = word.replace(['l', 'r'], "w").replace("th", "f").replace('d', "t");

    for vowel in VOWELS.iter() {
        let mut from = String::from("n") + *vowel;
        let mut to = String::from("ny") + *vowel;

        out = out.replace(&from, &to);
    }

    let end = {
        if rand::thread_rng().gen_ratio(1, 8) {
            random_emoji()
        } else {
            "".to_string()
        }
    };

    let first_char = out.chars().next().unwrap();

    if out.len() > 2 && first_char.is_alphanumeric() && rand::thread_rng().gen_ratio(1, 3) {
        let stutters = (rand::thread_rng().gen_range(1..=5) - 3).clamp(1, 2);
        let mut tmp = String::from("");
        for _ in 1..=stutters {
            tmp.push(first_char);
            tmp.push('-');
        }
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
