use clap::{App, Arg, AppSettings};

fn main() {
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
                                .about("input file to validate")
                                .required(true)
                        )
                        .arg(
                            Arg::new("template")
                                .about("header template")
                                .required(true)
                        )
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("check", check_matches)) => {
            match check_matches.subcommand() {
                Some(("header", header_matches)) => {
                  println!(
                      "checking header of {} with template {}",
                      header_matches.value_of("file").unwrap(),
                      header_matches.value_of("template").unwrap()
                  );
                },
                _ => unreachable!()
            }
        },
        None => println!("No subcommand was used."),
        _ => unreachable!()
    }
}
