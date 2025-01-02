use crate::cmus;
use crate::config::config;
use crate::debug::debugger::Debugger;
use crate::discord::formatter;
use discord_rich_presence::DiscordIpc;
use std::error::Error;

pub struct DiscordController {
    drpc: discord_rich_presence::DiscordIpcClient,
}

impl DiscordController {
    pub fn new(app_id: &str, debugger: &Debugger) -> DiscordController {
        let mut controller = DiscordController {
            drpc: discord_rich_presence::DiscordIpcClient::new(app_id).unwrap(),
        };

        match controller.drpc.connect() {
            Ok(_) => debugger.log("Connected to Discord"),
            Err(e) => {
                debugger.log_error(&format!("Failed to connect to Discord: {}", e));
                std::process::exit(1);
            }
        }
        debugger.log("Discord RPC client started");

        controller
    }

    pub fn update_presence(
        &mut self,
        cmus_response: cmus::responce::Response,
        debugger: &Debugger,
        configs: &config::Config,
        buttons_vec: &Vec<discord_rich_presence::activity::Button>,
    ) {
        debugger.log("Updating presence");

        let part_1 = formatter::format(configs.part_one_format.as_str(), &cmus_response);
        debugger.log(format!("part_1: {}", part_1).as_str());
        let part_2 = formatter::format(configs.part_two_format.as_str(), &cmus_response);
        debugger.log(format!("part_2: {}", part_2).as_str());
        let album = formatter::format(configs.album.as_str(), &cmus_response);
        let title = formatter::format(configs.title.as_str(), &cmus_response);
        let default = String::from("cmus");
        debugger.log(format!("album: {}", album,).as_str());

        let large_image = configs
            .covers
            .get(&album)
            .unwrap_or(configs.covers.get(&title).unwrap_or(&default));

        let activity = discord_rich_presence::activity::Activity::new()
            .state(part_2.as_str())
            .details(part_1.as_str())
            .assets(
                discord_rich_presence::activity::Assets::new()
                    .large_image(large_image)
                    .large_text(configs.large_text.as_str())
                    .small_image(match cmus_response.state {
                        cmus::responce::State::PLAYING => configs.playing_image.as_str(),
                        _ => configs.paused_image.as_str(),
                    })
                    .small_text(match cmus_response.state {
                        cmus::responce::State::PLAYING => configs.playing_text.as_str(),
                        _ => configs.paused_text.as_str(),
                    }),
            )
            .buttons(buttons_vec.to_vec());

        for _ in 0..3 {
            match self.drpc.set_activity(activity.clone()) {
                Ok(_) => debugger.log("Activity updated"),
                Err(e) => {
                    debugger.log_error(&format!("Error updating activity: {}", e));
                    match self.drpc.reconnect() {
                        Ok(_) => debugger.log("Reconnected successfully"),
                        Err(e) => {
                            debugger.log_error(&format!("Failed to reconnect to Discord: {}", e))
                        }
                    }
                }
            }
        }
    }

    pub fn remove_activity(&mut self) -> Result<(), Box<dyn Error>> {
        self.drpc.close()
    }
}
