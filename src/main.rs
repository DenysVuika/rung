mod logger;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use log::{error, info};
use rung::{check_headers, validate_json};
use std::path::Path;
use std::process;

fn main() {
    logger::init_logger();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("check")
                .about("checks things")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("header")
                        .version(crate_version!())
                        .about("Validates that the file(s) header matches the template(s)")
                        .arg(
                            Arg::new("file")
                                .about("input file to validate")
                                .long("file")
                                .short('f')
                                .value_name("FILE")
                                .takes_value(true)
                                .multiple(true)
                                .required(true),
                        )
                        .arg(
                            Arg::new("template")
                                .about("template file")
                                .long("template")
                                .short('t')
                                .value_name("TEMPLATE")
                                .takes_value(true)
                                .multiple(true)
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("json")
                        .version(crate_version!())
                        .about("Validates a JSON file matches the JSON Schema file")
                        .arg(
                            Arg::new("file")
                                .about("input file to validate")
                                .long("file")
                                .short('f')
                                .value_name("FILE")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::new("template")
                                .about("template file")
                                .long("template")
                                .short('t')
                                .value_name("TEMPLATE")
                                .takes_value(true)
                                .required(true),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("check", check_matches)) => match check_matches.subcommand() {
            Some(("header", header_matches)) => {
                let files: Vec<_> = header_matches
                    .values_of("file")
                    .unwrap()
                    .map(|path| Path::new(path))
                    .collect();
                let templates: Vec<_> = header_matches
                    .values_of("template")
                    .unwrap()
                    .map(|path| Path::new(path))
                    .collect();

                let result = check_headers(&files, &templates);
                if result {
                    info!("Validation succeeded");
                    process::exit(0);
                } else {
                    error!("Validation failed");
                    process::exit(1);
                }
            }
            Some(("json", json_matches)) => {
                let file = json_matches.value_of("file").unwrap();
                let file_path = Path::new(file);
                let template = json_matches.value_of("template").unwrap();
                let template_path = Path::new(template);

                match validate_json(file_path, template_path) {
                    Ok(true) => {
                        info!("Validation succeeded");
                        process::exit(0);
                    }
                    Ok(false) => {
                        info!("Validation failed");
                        process::exit(1);
                    }
                    Err(err) => {
                        error!("{}", err);
                        process::exit(1);
                    }
                }
            }
            _ => unreachable!(),
        },
        None => println!("No subcommand was used."),
        _ => unreachable!(),
    }
}
