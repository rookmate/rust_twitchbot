extern crate irc;
extern crate log;
extern crate log4rs;

mod log_setup {
    use log4rs::config::{
        Appender,
        Config, ConfigBuilder,
        Logger, LoggerBuilder,
        Root, RootBuilder,
    };

    pub(crate) fn logger_builder() -> LoggerBuilder {
        Logger::builder()
            .appender("stdout")
            .appender("file")
            .additive(false)
    }

    pub(crate) fn root_builder() -> RootBuilder {
        Root::builder().appender("stdout")
    }

    pub(crate) fn config_builder() -> ConfigBuilder {
        use log4rs::{append::console::ConsoleAppender, encode::pattern::PatternEncoder};

        let pattern = PatternEncoder::new("{d(%Y-%m-%dT%H:%M:%S%.3f%Z)} {highlight({l:5.5})} {t} - {m}{n}");

        Config::builder().appender(
            Appender::builder().build(
                "stdout",
                Box::new(
                    ConsoleAppender::builder()
                        .encoder(Box::new(pattern))
                        .build(),
                ),
            ),
        )
    }
}

fn setup_logger() {
    use self::log_setup::{config_builder, logger_builder, root_builder};
    use log::LevelFilter;
    use log4rs::{
        append::rolling_file::{RollingFileAppender, policy::compound},
        config::Appender,
        encode::pattern::PatternEncoder
    };

    // ROLLING WINDOW CONFIGS //
    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = compound::roll::fixed_window::FixedWindowRoller::builder()
        .build("log/rusty{}.log",window_size)
        .unwrap();

    let size_limit = 1000 * 1024; // 1MB as max log file size to roll
    let size_trigger = compound::trigger::size::SizeTrigger::new(size_limit);

    let compound_policy = compound::CompoundPolicy::new(Box::new(size_trigger),Box::new(fixed_window_roller));

    // LOG FILE CONFIG //
    let pattern = PatternEncoder::new("{d(%Y-%m-%dT%H:%M:%S%.3f%Z)} {l:5.5} {t} - {m}{n}");

    let mut config = config_builder().appender(
        Appender::builder().build(
            "file",
            Box::new(
                RollingFileAppender::builder()
                    .encoder(Box::new(pattern))
                    .build("log/rusty.log", Box::new(compound_policy))
                    .unwrap(),
            ),
        ),
    );

    config = config.logger(logger_builder().build("rustybot", LevelFilter::Info));
    let final_config = config.build(root_builder().build(LevelFilter::Off)).unwrap();
    log4rs::init_config(final_config).unwrap();
}

fn main() {
    use std::env;
    use irc::client::prelude::*;
    use log::{info};

    setup_logger();

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

    reactor.register_client_with_handler(irc_client, |client, raw_msg| {
        info!("Incoming <<< {}", raw_msg);
        if let Command::PRIVMSG(ref channel, ref msg) = raw_msg.command {
            info!("{}: {}", raw_msg.source_nickname().unwrap(), msg);
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
