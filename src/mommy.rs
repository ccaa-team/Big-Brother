use std::ops::Deref;

use poise::serenity_prelude::Context;
use poise::serenity_prelude::Message;
use poise::serenity_prelude::ReactionType;
use rand::prelude::*;

use rand::thread_rng;
use serde::Deserialize;

use crate::Data;

const MOOD: &str = "thirsty";
const REPLACEMENTS: [(&str, &[&str]); 6] = [
    ("{role}", &["mommy"]),
    ("{pronoun}", &["her"]),
    ("{affectionate_term}", &["girl"]),
    (
        "{denigrating_term}",
        &["slut", "toy", "pet", "pervert", "whore"],
    ),
    ("{part}", &["milk"]),
    ("~", &["\\~"]),
];
const EMOTES: [&str; 5] = [
    ":heart:",
    ":revolving_hearts:",
    ":sparkling_heart:",
    ":heartbeat:",
    ":two_hearts:",
];
static mut RESPONSES: Option<&'static Responses> = None;

#[derive(Deserialize)]
struct Responses {
    positive: Vec<ChunkList>,
    negative: Vec<ChunkList>,
    #[serde(rename = "beg_first")]
    beg: Vec<ChunkList>,
}
struct ChunkList(Vec<Chunk>);
impl<'de> Deserialize<'de> for ChunkList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        Ok(Self(str.split(" ").map(Chunk::from).collect()))
    }
}
impl Deref for ChunkList {
    type Target = Vec<Chunk>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct Chunk(String);
impl<T> From<T> for Chunk
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let mut out = String::from(value.as_ref());
        for (from, to) in REPLACEMENTS.iter() {
            out = out.replace(from, to.choose(&mut thread_rng()).unwrap());
        }
        Self(out)
    }
}
impl Deref for Chunk {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn get_responses() -> &'static Responses {
    // Why do all this? I don't like the idea of parsing json on every command call.
    unsafe {
        if RESPONSES.is_none() {
            let json: serde_json::Value =
                serde_json::from_str(include_str!("responses.json")).unwrap();

            let resp: Box<Responses> = Box::from(
                serde_json::from_value::<Responses>(json["moods"][MOOD].clone()).unwrap(),
            );

            RESPONSES = Some(Box::leak(resp));
        }

        RESPONSES.unwrap_unchecked()
    }
}

fn substitute(s: &[Chunk]) -> String {
    let mut out = String::new();
    s.iter().map(|c| c.to_string()).for_each(|s| {
        out += &s;
        out += " ";
    });
    let mut rng = thread_rng();
    if rng.gen_bool(0.5) {
        out += EMOTES.choose(&mut thread_rng()).unwrap()
    }
    out
}

pub fn praise() -> String {
    let mut rng = thread_rng();
    substitute(get_responses().positive.choose(&mut rng).unwrap())
}

pub fn negative() -> String {
    let mut rng = thread_rng();
    substitute(get_responses().negative.choose(&mut rng).unwrap())
}

pub fn beg() -> String {
    let mut rng = thread_rng();
    substitute(get_responses().beg.choose(&mut rng).unwrap())
}

pub async fn message(ctx: &Context, data: &Data, msg: &Message) -> anyhow::Result<()> {
    let mut data = data.has_to_beg.lock().await;
    if data.get(&msg.author.id).is_some() && msg.content.to_lowercase().starts_with("please") {
        data.take(&msg.author.id);
        drop(data);
        // revolving hearts emoji
        msg.react(&ctx.http, ReactionType::Unicode("💞".to_owned()))
            .await?;
    }

    Ok(())
}
