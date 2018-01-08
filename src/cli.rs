use std::time;

use clap::{Arg, App};

pub struct DelexOpts {
    pub channel_name: String,
    pub delay: time::Duration,
    pub dry_run: bool,
    pub json_files: Vec<String>,
}

impl DelexOpts {
    pub fn parse_opts() -> DelexOpts {
        let matches = App::new("slack-delex")
            .arg(Arg::with_name("channel-name")
                 .short("c")
                 .long("channel-name")
                 .value_name("CHANNEL_NAME")
                 .help("Specify channel name")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::with_name("dry-run")
                 .short("n")
                 .long("dry-run"))
            .arg(Arg::with_name("delay")
                 .short("d")
                 .long("delay")
                 .value_name("DELAY")
                 .help("Specify delay (ms) after one deletion")
                 .default_value("900")
                 .takes_value(true))
            .arg(Arg::with_name("JSON_FILE")
                 .help("Specify JSON file exported from Slack")
                 .multiple(true)
                 .required(true)
                 .index(1))
            .get_matches();

        let channel_name = matches.value_of("channel-name").unwrap();
        let dry_run = matches.is_present("dry-run");
        let delay = time::Duration::from_millis(value_t_or_exit!(matches, "delay", u64));
        let json_files = matches.values_of("JSON_FILE").unwrap().map(|s| s.to_string()).collect();

        DelexOpts {
            channel_name: channel_name.to_string(),
            delay: delay,
            dry_run: dry_run,
            json_files: json_files,
        }
    }
}
