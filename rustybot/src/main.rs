extern crate irc;

use std::env;
use irc::client::prelude::*;


fn main() {
    let oauth_token = env::var("TWITCH_OAUTH_TOKEN").unwrap();

    let cfg = Config {
        nickname: Some("RustyBot".to_owned()),  // Bot name
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6667),
        password: Some(oauth_token),    // Token from oauth token to tmi
        channels: Some(vec!["#morpho_one".to_owned()]),
        ..Default::default()
    };

    let irc_client = IrcClient::from_config(cfg).unwrap();

    irc_client.identify().unwrap();

    irc_client.for_each_incoming(|irc_msg| {
        println!("Incoming <<< {}", irc_msg);
    }).unwrap();
}
