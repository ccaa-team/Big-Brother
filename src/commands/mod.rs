macro_rules! cmd {
    ($($u:tt), *) => {
        $(
            mod $u;
            pub use $u::*;
        )*
    };
}

cmd!(autoreply, calc, moyai, neko, scan, uptime, uwu);
