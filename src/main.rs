mod args;
use regex::Regex;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use args::{parse_args, Options};

//  TODO: do something with this in the future.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Base16Colors {
    hex_name: String,
    color_name: String,
}

fn create_foot_theme(
    base16_colors: &HashMap<String, String>,
    foot_path: &String,
    theme_name: &String,
) -> Result<(), String> {
    println!("base16_colors: from foot_theme {:#?}", base16_colors);
    let file_conf_str = format!(
        "
    # -*- conf -*-
    # {:}
    [cursor]
    color={} {}

    [colors]
    background={}
    foreground={}

    # Normal/regular colors (color palette 0-7)
    regular0={} # black
    regular1={} # red
    regular2={} # green
    regular3={} # yellow
    regular4={} # blue
    regular5={} # magenta
    regular6={} # cyan
    regular7={} # white

    # Bright colors (color palette 8-15)
    bright0={} # black
    bright1={} # red
    bright2={} # green
    bright3={} # yellow
    bright4={} # blue
    bright5={} # magenta
    bright6={} # cyan
    bright7={} # white
    ",
        theme_name,
        base16_colors
            .get("cursor_fg")
            .unwrap_or(&"000000".to_string()),
        base16_colors.get("cursor").unwrap(),
        base16_colors.get("background").unwrap(),
        base16_colors.get("foreground").unwrap(),
        base16_colors.get("color0").unwrap(),
        base16_colors.get("color1").unwrap(),
        base16_colors.get("color2").unwrap(),
        base16_colors.get("color3").unwrap(),
        base16_colors.get("color4").unwrap(),
        base16_colors.get("color5").unwrap(),
        base16_colors.get("color6").unwrap(),
        base16_colors.get("color7").unwrap(),
        base16_colors.get("color8").unwrap(),
        base16_colors.get("color9").unwrap(),
        base16_colors.get("color10").unwrap(),
        base16_colors.get("color11").unwrap(),
        base16_colors.get("color12").unwrap(),
        base16_colors.get("color13").unwrap(),
        base16_colors.get("color14").unwrap(),
        base16_colors.get("color15").unwrap(),
    );

    let write_path = format!("{}/{}", foot_path, theme_name.to_lowercase());

    println!("{}", file_conf_str);
    let result = fs::write(write_path, file_conf_str.trim());
    match result {
        Err(_) => Err("Error writing file".to_string()),
        Ok(_) => Ok(()),
    }
}

fn create_alacritty_theme(
    base16_colors: &HashMap<String, String>,
    alacritty_path: &String,
    theme_name: &String,
) -> Result<(), String> {
    let file_conf_str = format!(
        "
        # {}

colors:
  primary:
    background: \"#{}\"
    foreground: \"#{}\"

  cursor: 
    text: \"#{}\"
    cursor: \"#{}\"

  normal:
    black: \"#{}\"
    red: \"#{}\"
    green: \"#{}\"
    yellow: \"#{}\"
    blue: \"#{}\"
    magenta: \"#{}\"
    cyan: \"#{}\"
    white: \"#{}\"

  bright:
    black: \"#{}\"
    red: \"#{}\"
    green: \"#{}\"
    yellow: \"#{}\"
    blue: \"#{}\"
    magenta: \"#{}\"
    cyan: \"#{}\"
    white: \"#{}\"
",
        theme_name,
        base16_colors.get("background").unwrap(),
        base16_colors.get("foreground").unwrap(),
        base16_colors
            .get("cursor_fg")
            .unwrap_or(&"000000".to_string()),
        base16_colors.get("cursor").unwrap(),
        base16_colors.get("color0").unwrap(),
        base16_colors.get("color1").unwrap(),
        base16_colors.get("color2").unwrap(),
        base16_colors.get("color3").unwrap(),
        base16_colors.get("color4").unwrap(),
        base16_colors.get("color5").unwrap(),
        base16_colors.get("color6").unwrap(),
        base16_colors.get("color7").unwrap(),
        base16_colors.get("color8").unwrap(),
        base16_colors.get("color9").unwrap(),
        base16_colors.get("color10").unwrap(),
        base16_colors.get("color11").unwrap(),
        base16_colors.get("color12").unwrap(),
        base16_colors.get("color13").unwrap(),
        base16_colors.get("color14").unwrap(),
        base16_colors.get("color15").unwrap(),
    );

    let write_path = format!("{}/{}.yml", alacritty_path, theme_name.to_lowercase());

    println!("{}", file_conf_str);
    let result = fs::write(write_path, file_conf_str.trim());
    match result {
        Err(_) => Err("Error writing file".to_string()),
        Ok(_) => Ok(()),
    }
}

fn create_kitty_theeme(
    base16_colors: &HashMap<String, String>,
    kitty_path: &String,
    theme_name: &String,
) -> Result<(), String> {
    let file_conf_str = format!(
        "
## {}

    background #{}
    foreground #{}
    cursor #{}
    cursor_text_color #{}
    color0 #{}
    color1 #{}
    color2 #{}
    color3 #{}
    color4 #{}
    color5 #{}
    color6 #{}
    color7 #{}
    color8 #{}
    color9 #{}
    color10 #{}
    color11 #{}
    color12 #{}
    color13 #{}
    color14 #{}
    color15 #{}
",
        theme_name,
        base16_colors.get("background").unwrap(),
        base16_colors.get("foreground").unwrap(),
        base16_colors.get("cursor").unwrap(),
        base16_colors.get("cursor_fg").unwrap(),
        base16_colors.get("color0").unwrap(),
        base16_colors.get("color1").unwrap(),
        base16_colors.get("color2").unwrap(),
        base16_colors.get("color3").unwrap(),
        base16_colors.get("color4").unwrap(),
        base16_colors.get("color5").unwrap(),
        base16_colors.get("color6").unwrap(),
        base16_colors.get("color7").unwrap(),
        base16_colors.get("color8").unwrap(),
        base16_colors.get("color9").unwrap(),
        base16_colors.get("color10").unwrap(),
        base16_colors.get("color11").unwrap(),
        base16_colors.get("color12").unwrap(),
        base16_colors.get("color13").unwrap(),
        base16_colors.get("color14").unwrap(),
        base16_colors.get("color15").unwrap(),
    );

    let write_path = format!("{}/{}.conf", kitty_path, theme_name.to_lowercase());

    println!("{}", file_conf_str);
    let result = fs::write(write_path, file_conf_str.trim());
    match result {
        Err(_) => Err("Error writing file".to_string()),
        Ok(_) => Ok(()),
    }
}

fn alacritty_colors_to_base16_colors(alacritty_colors_path: &str) -> HashMap<String, String> {
    let file = File::open(alacritty_colors_path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut base16_colors: HashMap<String, String> = HashMap::new();
    let mut i: i32 = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim();
            if let Some(index) = trimmed.find(":") {
                let (key, value) = trimmed.split_at(index);
                let key = key;
                let value = value
                    .trim()
                    .trim_matches(|c: char| c == '"' || c == ':' || c.is_whitespace())
                    .replace("#", "")
                    .replace("'", "");
                match key {
                    "background" => {
                        base16_colors.insert("background".to_string(), value.to_string());
                    }
                    "foreground" => {
                        base16_colors.insert("foreground".to_string(), value.to_string());
                    }
                    "cursor" => {
                        base16_colors.insert("cursor".to_string(), value.to_string());
                    }
                    "text" => {
                        base16_colors.insert("cursor_fg".to_string(), value.to_string());
                    }
                    _ => {
                        if key.starts_with("color")
                            || key.starts_with("primary")
                            || key.starts_with("cursor")
                            || key.starts_with("normal")
                            || key.starts_with("bright")
                        {
                            continue;
                        }
                        base16_colors.insert(format!("color{}", i), value.to_string());
                        i += 1;
                    }
                }
            }
        }
    }
    return base16_colors;
}

fn kitty_colors_to_base16_colors(kitty_colors_path: &String) -> HashMap<String, String> {
    let kitty_colors_file = fs::read_to_string(kitty_colors_path).unwrap();
    let mut base16_colors: HashMap<String, String> = HashMap::new();
    fn check_color(line: &str) -> bool {
        line.contains("color")
            || line.contains("cursor")
            || Regex::new(r"\bbackground\b").unwrap().is_match(line)
            || Regex::new(r"\bforeground\b").unwrap().is_match(line)
    }

    for line in kitty_colors_file.trim().lines().into_iter() {
        match check_color(line) {
            true => {
                // skipping comments
                if line.starts_with("#") {
                    continue;
                };

                let line_values: Vec<String> = line
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.replace("#", ""))
                    .collect::<Vec<String>>();

                //  checking if the color is a reference for another color token
                if check_color(line_values[1].as_str()) {
                    if line.contains("cursor_text_color") {
                        base16_colors.insert(
                            "cursor_fg".to_string(),
                            base16_colors
                                .get(&line_values[1].trim().to_string())
                                .unwrap()
                                .to_string(),
                        );
                        continue;
                    }
                    base16_colors.insert(
                        line_values[0].trim().to_string(),
                        base16_colors
                            .get(&line_values[1].trim().to_string())
                            .unwrap()
                            .to_string(),
                    );
                    continue;
                }
                if line.contains("cursor_text_color") {
                    base16_colors
                        .insert("cursor_fg".to_string(), line_values[1].trim().to_string());
                    continue;
                }
                base16_colors.insert(
                    line_values[0].trim().to_string(),
                    line_values[1].trim().to_string(),
                );
            }
            false => println!("Nop."),
        }
    }
    return base16_colors;
}

fn create_base16_colors(
    term_input_name: &String,
    term_input_file: &String,
) -> HashMap<String, String> {
    if let "kitty" = term_input_name.as_str() {
        kitty_colors_to_base16_colors(term_input_file)
    } else {
        alacritty_colors_to_base16_colors(term_input_file)
    }
}

fn create_theme(term_output: &String, args: &Options) -> Result<(), String> {
    let base16_colors = create_base16_colors(&args.term_input, &args.term_input_file);
    let mut result_foot: Result<(), String> = Ok(());
    let mut result_alacritty: Result<(), String> = Ok(());
    let mut result_kitty: Result<(), String> = Ok(());
    match term_output.as_str() {
        "all" => {
            if args.term_input != "foot" {
                result_foot =
                    create_foot_theme(&base16_colors, &args.foot_output_folder, &args.theme_name);
            }

            if args.term_input != "alacritty" {
                result_alacritty = create_alacritty_theme(
                    &base16_colors,
                    &args.alacritty_output_folder,
                    &args.theme_name,
                );
            }

            if args.term_input != "kitty" {
                result_kitty = create_kitty_theeme(
                    &base16_colors,
                    &args.kitty_output_folder,
                    &args.theme_name,
                );
            }
        }
        "foot" => {
            result_foot =
                create_foot_theme(&base16_colors, &args.foot_output_folder, &args.theme_name);
        }

        "alacritty" => {
            result_alacritty = create_alacritty_theme(
                &base16_colors,
                &args.alacritty_output_folder,
                &args.theme_name,
            );
        }
        "kitty" => {
            result_kitty =
                create_kitty_theeme(&base16_colors, &args.kitty_output_folder, &args.theme_name);
        }
        &_ => {}
    }

    //  TODO: this need a refector
    match (result_foot, result_alacritty, result_kitty) {
        (Ok(()), Ok(()), Ok(())) => Ok(()),
        (Err(err_foot), _, _) => Err(err_foot),
        (_, Err(err_alacritty), _) => Err(err_alacritty),
        (_, _, Err(err_kitty)) => Err(err_kitty),
    }
}

fn main() {
    let args: Options = parse_args();
    println!("{:?}", args);

    let result_create_theme = create_theme(&args.terminal_output, &args);
    let _ = match result_create_theme {
        Err(err) => Err(err),
        Ok(_) => {
            println!("Theme {} parsed", args.theme_name);
            Ok(())
        }
    };
}
