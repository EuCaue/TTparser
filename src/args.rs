use clap::{App, Arg};
use std::env;

#[derive(Debug)]
pub struct Options {
    pub term_input: String,
    pub term_input_file: String,
    pub foot_output_folder: String,
    pub kitty_output_folder: String,
    pub alacritty_output_folder: String,
    pub terminal_output: String,
    pub theme_name: String,
}

pub fn parse_args() -> Options {
    let home = env::var("HOME").ok().unwrap();
    let matches = App::new("TTparser")
        .arg(
            Arg::with_name("term-input")
                .short('i')
                .long("--term-input")
                .possible_values(["kitty", "alacritty"])
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("term-input-file")
                .short('f')
                .help(Some("The theme file for the terminal input."))
                .required(true)
                .long("--term-input-file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("theme-name")
                .default_value("Theme ported with TTParser.")
                .short('n')
                .long("--theme-name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("foot-output-folder")
                .default_value(format!("{}/.config/foot", home).as_str())
                .long("--foot-output-folder")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("alacritty-output-folder")
                .default_value(format!("{}/.config/alacritty", home).as_str())
                .long("--alacritty-output-folder")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("kitty-output-folder")
                .default_value(format!("{}/.config/kitty", home).as_str())
                .long("--kitty-output-folder")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("terminal-output")
                .default_value("all")
                .possible_values(["all", "alacritty", "foot", "kitty"])
                .short('o')
                .long("--terminal-output")
                .takes_value(true),
        )
        .get_matches();

    let term_input = matches.value_of("term-input").unwrap().to_lowercase();
    let term_input_file = matches.value_of("term-input-file").unwrap().to_lowercase();
    let theme_name = matches.value_of("theme-name").unwrap().to_lowercase();
    let kitty_output_folder = matches
        .value_of("kitty-output-folder")
        .unwrap()
        .to_lowercase();
    let foot_output_folder = matches
        .value_of("foot-output-folder")
        .unwrap()
        .to_lowercase();
    let alacritty_output_folder = matches
        .value_of("alacritty-output-folder")
        .unwrap()
        .to_lowercase();
    let terminal_output = matches.value_of("terminal-output").unwrap().to_lowercase();

    Options {
        term_input,
        term_input_file,
        foot_output_folder,
        kitty_output_folder,
        terminal_output,
        alacritty_output_folder,
        theme_name,
    }
}
