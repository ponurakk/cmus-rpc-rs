pub(crate) mod cli_options;

use clap::{App, Arg};

pub fn get_args_app() -> App<'static> {
    App::new("cmus-rpc-rs")
        .version("0.1.0")
        .author("Anas Elgarhy <anas.elgarhy.dev@gmail.com>")
        .about("A Discord Rich Presence for cmus player")
        .args(&[
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Enable debug mode")
                .takes_value(false),
            Arg::with_name("link")
                .long("link")
                .short('l')
                .help("Linking with cmus (close the program if cmus is not running)")
                .takes_value(false),
            Arg::with_name("config_path")
                .long("config_path")
                .short('c')
                .help("Set custom path to config_path file")
                .takes_value(true),
            Arg::with_name("interval")
                .long("interval")
                .short('i')
                .help("Set custom interval for updating the presence")
                .takes_value(true),
            Arg::with_name("sleep")
                .long("sleep")
                .short('s')
                .help("Set sleep when there is no activity")
                .takes_value(true),
            Arg::with_name("part_one_format")
                .long("partOneFormat")
                .alias("p1f")
                .help("Set custom format for part one of presence")
                .takes_value(true),
            Arg::with_name("part_two_format")
                .long("partTwoFormat")
                .alias("p2f")
                .help("Set custom format for part two of presence")
                .takes_value(true),
        ])
}
