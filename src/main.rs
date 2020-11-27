use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use rung::angular;

mod check_header;
mod check_json;
mod logger;
mod serve;
mod utils;

fn main() -> Result<()> {
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
        .subcommand(
            App::new("serve")
                .version(crate_version!())
                .about("Runs a lightweight web server")
                .arg(Arg::new("dir").required(true).index(1))
                .arg(
                    Arg::new("host")
                        .about("Host address")
                        .long("host")
                        .short('h')
                        .value_name("HOST")
                        .takes_value(true)
                        .default_value("127.0.0.1"),
                )
                .arg(
                    Arg::new("port")
                        .about("Port number")
                        .long("port")
                        .short('p')
                        .value_name("PORT")
                        .takes_value(true)
                        .default_value("8080"),
                ),
        )
        .subcommand(
            App::new("ls")
                .version(crate_version!())
                .about("List all projects")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .short('c')
                        .value_name("PATH")
                        .takes_value(true)
                        .default_value("angular.json"),
                )
                .subcommand(
                    App::new("apps").about("List all applications").arg(
                        Arg::new("config")
                            .long("config")
                            .short('c')
                            .value_name("PATH")
                            .takes_value(true)
                            .default_value("angular.json"),
                    ),
                )
                .subcommand(
                    App::new("libs").about("List all libraries").arg(
                        Arg::new("config")
                            .long("config")
                            .short('c')
                            .value_name("PATH")
                            .takes_value(true)
                            .default_value("angular.json"),
                    ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("check", check_matches)) => match check_matches.subcommand() {
            Some(("header", header_matches)) => check_header::run(header_matches),
            Some(("json", json_matches)) => check_json::run(json_matches),
            _ => unreachable!(),
        },
        Some(("serve", serve_matches)) => serve::run(serve_matches).unwrap(),
        Some(("ls", ls_matches)) => match ls_matches.subcommand() {
            Some(("apps", apps_matches)) => {
                let config = angular::get_workspace_config(apps_matches)?;
                angular::list_projects_by_type(&config, angular::ProjectType::Application)?
            }
            Some(("libs", libs_matches)) => {
                let config = angular::get_workspace_config(libs_matches)?;
                angular::list_projects_by_type(&config, angular::ProjectType::Library)?
            }
            _ => {
                let config = angular::get_workspace_config(ls_matches)?;
                angular::list_projects(&config)?;
            }
        },
        None => println!("No subcommand was used."),
        _ => unreachable!(),
    }

    Ok(())
}
