use clap::{arg, command};
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

    let matches = command!("TTparser")
        .arg(
            arg!(-i --"term-input" <"kitty,alacritty"> "Terminal input name").required(true)
        )
        .arg(
            arg!(-f --"term-input-file" <term_input_file> "The theme file for the terminal input")
                .required(true)
        )
        .arg(
            arg!(-n --"theme-name" <theme_name> "The theme name")
                .default_value("Theme ported with TTParser."),
        )
        .arg(
            arg!(--"foot-output-folder" <foot_output_folder> "The output folder for foot [default: $HOME/.config/foot]")
        )
        .arg(
            arg!(--"alacritty-output-folder" <alacritty_output_folder> "The output folder for alacritty [default: $HOME/.config/alacritty]")
        )
        .arg(
            arg!(--"kitty-output-folder" <kitty_output_folder> "The output folder for kitty [default: $HOME/.config/kitty]")
                    
        )
        .arg(
            arg!(-o --"terminal-output" <"all, alacritty, foot, kitty"> "The terminal output")
                .default_value("all")
                    
        )
        .get_matches();

    let term_input = matches.get_one::<String>("term-input").unwrap().to_lowercase();
    let term_input_file = matches.get_one::<String>("term-input-file").unwrap().to_lowercase();
    let theme_name = matches.get_one::<String>("theme-name").unwrap().to_lowercase();
    let kitty_output_folder = matches.get_one::<String>("kitty-output-folder").unwrap_or(&format!("{}/.config/kitty", home)).to_lowercase();
    let foot_output_folder = matches.get_one::<String>("foot-output-folder").unwrap_or(&format!("{}/.config/foot", home)).to_lowercase();
    let alacritty_output_folder = matches.get_one::<String>("alacritty-output-folder").unwrap_or(&format!("{}/.config/alacritty", home)).to_lowercase();
    let terminal_output = matches.get_one::<String>("terminal-output").unwrap().to_lowercase();

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
