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

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn uwu_word(word: &str) -> String {
    if word.starts_with("http") || word.is_empty() {
        return word.to_string();
    };

    let mut out = word
        .replace(['l', 'r'], "w")
        .replace("th", "f")
        .replace('d', "t");

    let mut n = String::from("n.");
    let mut ny = String::from("ny.");
    for vowel in VOWELS.iter() {
        n.pop();
        n.push(*vowel);
        ny.pop();
        ny.push(*vowel);

        out = out.replace(&n, &ny);
    }

    let end = {
        if rand::thread_rng().gen_ratio(1, 8) {
            random_emoji()
        } else {
            "".to_string()
        }
    };

    let first_char = out.chars().next().expect("guh??");

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

    out + &end
}

pub fn uwuify(text: String) -> String {
    let low = text.to_lowercase();

    low.split(' ').map(uwu_word).collect::<Vec<_>>().join(" ")
}
