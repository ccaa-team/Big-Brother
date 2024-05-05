use crate::{Data, Error};
use poise::Command;

macro_rules! cmd {
    ($($u:tt), *) => {
        $(
            mod $u;
            pub use $u::$u;
        )*
        pub fn list() -> Vec<Command<Data, Error>> {
            vec![$($u(),)*]
        }
    };
}

cmd!(autoreply, average, uptime, sql, help, markov);
