use rand::prelude::*;

fn random_emoji() -> String {
    let emojis: [String; 32] = [
        " rawr x3".to_owned(),
        " OwO".to_owned(),
        " UwU".to_owned(),
        " o.O".to_owned(),
        " -.-".to_owned(),
        " >w<".to_owned(),
        " (⑅˘꒳˘)".to_owned(),
        " (ꈍᴗꈍ)".to_owned(),
        " (˘ω˘)".to_owned(),
        " (U ᵕ U❁)".to_owned(),
        " σωσ".to_owned(),
        " òωó".to_owned(),
        " (///ˬ///✿)".to_owned(),
        " (U ﹏ U)".to_owned(),
        " ( ͡o ω ͡o )".to_owned(),
        " ʘwʘ".to_owned(),
        " :3".to_owned(),
        " :3".to_owned(),
        " XD".to_owned(),
        " nyaa\\~\\~".to_owned(),
        " mya".to_owned(),
        " >_<".to_owned(),
        " 😳".to_owned(),
        " 🥺".to_owned(),
        " 😳😳😳".to_owned(),
        " rawr".to_owned(),
        " ^^".to_owned(),
        " ^^;;".to_owned(),
        " (ˆ ﻌ ˆ)♡".to_owned(),
        " ^•ﻌ•^".to_owned(),
        " /(^•ω•^)".to_owned(),
        " (✿oωo)".to_owned(),
    ];
    let idx = rand::thread_rng().gen_range(1..32);
    return emojis[idx].clone();
}

static VOWELS: [char; 5] = ['a', 'e', 'i', 'u', 'o'];

fn uwu_word(word: &str) -> Option<String> {
    if word.starts_with("http") || word.len() == 0 {
        return None;
    }

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
