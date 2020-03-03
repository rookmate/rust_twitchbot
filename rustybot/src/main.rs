extern crate irc;

use std::env;
use irc::client::prelude::*;


fn main() {
    let oauth_token = env::var("TWITCH_OAUTH_TOKEN").unwrap();
    // We can also load the Config at runtime via Config::load("path/to/config.toml")
    let cfg = Config {
        nickname: Some("rusty_morpho".to_owned()),  // Bot name
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6667),
        password: Some(oauth_token),    // Token from oauth token to tmi
        channels: Some(vec!["#morpho_one".to_owned()]),
        ..Default::default()
    };

    let mut reactor = IrcReactor::new().unwrap();
    let irc_client = reactor.prepare_client_and_connect(&cfg).unwrap();
    irc_client.identify().unwrap();

    reactor.register_client_with_handler(irc_client, |client, message| {
        println!("Incoming <<< {}", message);
        if let Command::PRIVMSG(ref channel, ref msg) = message.command {
            if msg.contains(client.current_nickname()) {
                client.send_privmsg(channel, "Reporting for duty!").unwrap();
            } else if msg.contains("!help") {
                client.send_privmsg(channel, "I wish I could help you sir.").unwrap();
            }
        }

        Ok(())
    });

    reactor.run().unwrap();
}
