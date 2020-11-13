mod logger;

use clap::{App, AppSettings, Arg};
use log::{error, info};
use rung::check_headers;

fn main() {
    logger::init_logger();

    let matches = App::new("rung")
        .version("0.1.0")
        .author("Denys Vuika <denys.vuika@gmail.com>")
        .about("Rust tools for Angular projects")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("check")
                .about("checks things")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("header")
                        .about("validates the file header matches the template")
                        .arg(
                            Arg::new("file")
                                .about("input file(s) to validate")
                                .long("file")
                                .short('f')
                                .value_name("FILE")
                                .takes_value(true)
                                .multiple(true)
                                .required(true),
                        )
                        .arg(
                            Arg::new("template")
                                .about("header template(s) to use for validation")
                                .long("template")
                                .short('t')
                                .value_name("TEMPLATE")
                                .takes_value(true)
                                .multiple(true)
                                .required(true),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("check", check_matches)) => match check_matches.subcommand() {
            Some(("header", header_matches)) => {
                let files: Vec<_> = header_matches.values_of("file").unwrap().collect();
                let templates: Vec<_> = header_matches.values_of("template").unwrap().collect();

                match check_headers(&files, &templates) {
                    Some(val) => {
                        if val {
                            info!("checks succeeded");
                        } else {
                            error!("{}", "check failed");
                            std::process::exit(1);
                        }
                    }
                    None => {
                        error!("{}", "check failed");
                        std::process::exit(1);
                    }
                }
            }
            _ => unreachable!(),
        },
        None => println!("No subcommand was used."),
        _ => unreachable!(),
    }
}
