macro_rules! cmd {
    ($($u:tt), *) => {
        $(
            mod $u;
            pub use $u::$u;
        )*
    };
}

cmd!(autoreply, calc, neko, uptime, uwu, nerd);
//cmd!(autoreply, calc, moyai, neko, scan, uptime, uwu, nerd);
