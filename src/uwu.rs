use rand::prelude::*;

static EMOJIS: [&str; 32] = [
    " rawr x3",
    " OwO",
    " UwU",
    " o.O",
    " -.-",
    " >w<",
    " (⑅˘꒳˘)",
    " (ꈍᴗꈍ)",
    " (˘ω˘)",
    " (U ᵕ U❁)",
    " σωσ",
    " òωó",
    " (///ˬ///✿)",
    " (U ﹏ U)",
    " ( ͡o ω ͡o )",
    " ʘwʘ",
    " :3",
    " :3",
    " XD",
    " nyaa\\~\\~",
    " mya",
    " >_<",
    " 😳",
    " 🥺",
    " 😳😳😳",
    " rawr",
    " ^^",
    " ^^;;",
    " (ˆ ﻌ ˆ)♡",
    " ^•ﻌ•^",
    " /(^•ω•^)",
    " (✿oωo)",
];

fn random_emoji() -> String {
    let idx = rand::thread_rng().gen_range(1..32);
    return EMOJIS[idx].to_string();
}

static VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];

fn uwu_word(word: &str) -> Option<String> {
    if word.len() == 0 {
        return None;
    } else if word.starts_with("http") {
        return Some(word.to_string());
    };

    let last_char = word.chars().last().unwrap();

    let mut out = word.replace("l", "w").replace("r", "w");

    for vowel in VOWELS.iter() {
        let mut from = String::from("n");
        let mut to = String::from("ny");

        from.push(*vowel);
        to.push(*vowel);

        out = out.replace(&from, &to);
    }

    let end = match last_char {
        '.' | '!' | '?' | '|' => random_emoji(),
        _ => "".to_string(),
    };

    let first_char = out.chars().next().unwrap();

    if out.len() > 2 && first_char.is_alphanumeric() && rand::thread_rng().gen_range(0..4) == 0 {
        let mut tmp = String::from("");
        tmp.push(first_char);
        tmp.push('-');
        for chr in out.chars() {
            tmp.push(chr);
        }
        out = tmp;
    }

    Some(out + &end + " ")
}

pub fn uwuify(text: String) -> String {
    let low = text.to_lowercase();

    let split = low.split(" ");
    let mut out = String::from("");

    for word in split.into_iter() {
        let uwud = uwu_word(word).unwrap();

        out += &uwud;
    }

    out
}
