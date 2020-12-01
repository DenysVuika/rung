use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use rung::{angular, logger, serve};
use std::path::PathBuf;

fn angular_config_arg<'a>() -> Arg<'a> {
    Arg::new("config")
        .long("config")
        .short('c')
        .value_name("PATH")
        .takes_value(true)
        .default_value("angular.json")
}

fn template_arg<'a>() -> Arg<'a> {
    Arg::new("template")
        .about("template file")
        .long("template")
        .short('t')
        .value_name("TEMPLATE")
        .takes_value(true)
        .multiple(true)
        .required(true)
}

fn input_file_arg<'a>() -> Arg<'a> {
    Arg::new("file")
        .about("input file")
        .long("file")
        .short('f')
        .value_name("FILE")
        .takes_value(true)
        .required(true)
}

fn directory_arg<'a>() -> Arg<'a> {
    Arg::new("directory")
        .about("The directory name to create the workspace in.")
        .long("directory")
        .short('d')
        .value_name("DIR")
        .takes_value(true)
}

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
                        .arg(input_file_arg())
                        .arg(template_arg()),
                )
                .subcommand(
                    App::new("json")
                        .version(crate_version!())
                        .about("Validates a JSON file matches the JSON Schema file")
                        .arg(input_file_arg())
                        .arg(template_arg()),
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
                .arg(angular_config_arg())
                .subcommand(
                    App::new("apps")
                        .about("List all applications")
                        .arg(angular_config_arg()),
                )
                .subcommand(
                    App::new("libs")
                        .about("List all libraries")
                        .arg(angular_config_arg()),
                ),
        )
        .subcommand(
            App::new("new")
                .version(crate_version!())
                .about("Creates a new workspace and an initial Angular application.")
                .arg(
                    Arg::new("name")
                        .about("The name of the new workspace and initial project.")
                        .required(true)
                        .index(1),
                )
                .arg(directory_arg()),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("check", check_matches)) => match check_matches.subcommand() {
            Some(("header", header_matches)) => rung::check_files_headers(header_matches),
            Some(("json", json_matches)) => rung::validate_json(json_matches),
            _ => unreachable!(),
        },
        Some(("serve", serve_matches)) => serve::run(serve_matches).unwrap(),
        Some(("ls", ls_matches)) => match ls_matches.subcommand() {
            Some(("apps", apps_matches)) => {
                let config = rung::get_workspace_config(apps_matches)?;
                angular::list_projects_by_type(&config, angular::ProjectType::Application)?
            }
            Some(("libs", libs_matches)) => {
                let config = rung::get_workspace_config(libs_matches)?;
                angular::list_projects_by_type(&config, angular::ProjectType::Library)?
            }
            _ => {
                let config = rung::get_workspace_config(ls_matches)?;
                angular::list_projects(&config)?;
            }
        },
        Some(("new", new_matches)) => {
            let name = new_matches.value_of("name").unwrap();
            let path = match new_matches.value_of("directory") {
                Some(value) => PathBuf::from(value),
                None => std::env::current_dir()?,
            };
            angular::new_application(name, &path)?;
        }
        None => println!("No subcommand was used."),
        _ => unreachable!(),
    }

    Ok(())
}
