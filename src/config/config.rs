use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::args;
use crate::config::config_file;

pub struct Config {
    pub debug: bool,
    pub link: bool,
    pub config_path: String,
    pub interval: u32,
    pub sleep: u32,
    pub part_one_format: String,
    pub part_two_format: String,
    pub album: String,
    pub title: String,
    pub large_image: String,
    pub playing_image: String,
    pub paused_image: String,
    pub large_text: String,
    pub playing_text: String,
    pub paused_text: String,
    pub button_one: (String, String),
    pub button_two: (String, String),
    pub covers: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Config {
        let args_matches = args::args::get_args_app().get_matches();
        let parsed_config = match config_file::load(
            args_matches
                .value_of("config_path")
                .unwrap_or(Config::default().config_path.as_str())
                .to_string(),
        ) {
            Ok(config) => config,
            Err(e) => {
                println!("Error loading config_path file: {}", e);
                println!("Creating the default config file.");
                config_file::create_default()
            }
        };
        let configs = Config {
            debug: args_matches.is_present("debug") || parsed_config.debug,
            link: args_matches.is_present("link") || parsed_config.link,
            config_path: if args_matches.is_present("config_path") {
                args_matches.value_of("config_path").unwrap().to_string()
            } else {
                parsed_config.config_path
            },
            interval: if args_matches.is_present("interval") {
                args_matches
                    .value_of("interval")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                parsed_config.interval
            },
            sleep: if args_matches.is_present("sleep") {
                args_matches
                    .value_of("sleep")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                parsed_config.sleep
            },
            part_one_format: if args_matches.is_present("part_one_format") {
                args_matches
                    .value_of("part_one_format")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.part_one_format
            },
            part_two_format: if args_matches.is_present("part_two_format") {
                args_matches
                    .value_of("part_two_format")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.part_two_format
            },
            album: parsed_config.album,
            title: parsed_config.title,
            large_image: if args_matches.is_present("large_image") {
                args_matches.value_of("large_image").unwrap().to_string()
            } else {
                parsed_config.large_image
            },
            playing_image: if args_matches.is_present("playing_image") {
                args_matches.value_of("playing_image").unwrap().to_string()
            } else {
                parsed_config.playing_image
            },
            paused_image: if args_matches.is_present("paused_image") {
                args_matches.value_of("paused_image").unwrap().to_string()
            } else {
                parsed_config.paused_image
            },
            large_text: if args_matches.is_present("large_text") {
                args_matches.value_of("large_text").unwrap().to_string()
            } else {
                parsed_config.large_text
            },
            playing_text: if args_matches.is_present("playing_text") {
                args_matches.value_of("playing_text").unwrap().to_string()
            } else {
                parsed_config.playing_text
            },
            paused_text: if args_matches.is_present("paused_text") {
                args_matches.value_of("paused_text").unwrap().to_string()
            } else {
                parsed_config.paused_text
            },
            button_one: if args_matches.is_present("button_one_text") {
                (
                    args_matches
                        .value_of("button_one_text")
                        .unwrap()
                        .to_string(),
                    // TODO: Handle this better
                    args_matches.value_of("button_one_url").unwrap().to_string(),
                )
            } else {
                parsed_config.button_one
            },
            button_two: if args_matches.is_present("button_two_text") {
                (
                    args_matches
                        .value_of("button_two_text")
                        .unwrap()
                        .to_string(),
                    // TODO: Handle this better
                    args_matches.value_of("button_two_url").unwrap().to_string(),
                )
            } else {
                parsed_config.button_two
            },
            covers: {
                let matches_path = format!(
                    "{}/{}",
                    dirs::config_dir().unwrap_or_default().to_str().unwrap(),
                    "cmus-rps-rs/match.txt"
                );
                let mut file = File::open(matches_path).unwrap();
                let mut text: String = String::new();
                file.read_to_string(&mut text).unwrap();
                let mut map = HashMap::new();

                for line in text.lines() {
                    let mut key_value = line.splitn(2, ';');
                    if let (Some(key), Some(value)) = (key_value.next(), key_value.next()) {
                        map.insert(key.to_string(), value.to_string());
                    }
                }

                map
            },
        };

        configs
    }

    pub fn default() -> Config {
        let config_path = format!(
            "{}/{}",
            dirs::config_dir().unwrap_or_default().to_str().unwrap(),
            "cmus-rps-rs/config.conf"
        );
        Config {
            debug: false,
            link: false,
            config_path: config_path.to_string(),
            interval: 1,   // seconds
            sleep: 5 * 60, // 5 minutes
            part_one_format: "%artist% - %title%".to_string(),
            part_two_format: "%album%".to_string(),
            album: "%album%".to_string(),
            title: "%title%".to_string(),
            large_image: "cmus".to_string(),
            playing_image: "play_icon_2".to_string(),
            paused_image: "pause_icon_2".to_string(),
            large_text: "cmus yoooo ".to_string(),
            playing_text: "Playing ".to_string(),
            paused_text: "Paused ".to_string(),
            button_one: (
                "On github 😆".to_string(),
                "https://github.com/anas-elgarhy/cmus-rps-rs".to_string(),
            ),
            button_two: ("".to_string(), "".to_string()),
            covers: HashMap::new(),
        }
    }

    pub fn has_button_one(&self) -> bool {
        self.button_one.0 != "" && self.button_one.1 != ""
    }

    pub fn has_button_two(&self) -> bool {
        self.button_two.0 != "" && self.button_two.1 != ""
    }
}
