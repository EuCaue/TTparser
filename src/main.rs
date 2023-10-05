use std::{collections::HashMap, env, fs};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Base16Colors {
    hex_name: String,
    color_name: String,
}

#[derive(Debug)]
struct Options {
    kitty_config_path: String,
    foot_output_folder: String,
    alacritty_output_folder: String,
    terminal_output: String,
    theme_name: String,
}

fn parse_args() -> Options {
    let home = env::var("HOME").ok().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut kitty_config_path: String = format!("{}/.config/kitty/current-theme.conf", home);
    let mut foot_output_folder: String = format!("{}/.config/foot", home);
    let mut alacritty_output_folder: String = format!("{}/.config/alacritty", home);
    let mut theme_name: String = "current-theme-port".to_string();
    let mut terminal_output: String = "all".to_string();

    for arg in args.iter() {
        let parts: Vec<&str> = arg.splitn(2, '=').collect();
        if parts.len() == 2 {
            println!("{}", parts[0]);
            match parts[0].to_lowercase().as_str() {
                "--kitty-colors-file" => {
                    kitty_config_path = parts[1].to_string().to_lowercase();
                }
                "--theme-name" => {
                    theme_name = parts[1].to_string().to_lowercase();
                }
                "--foot-folder" => {
                    foot_output_folder = parts[1].to_string().to_lowercase();
                }
                "--alacritty-folder" => {
                    alacritty_output_folder = parts[1].to_string().to_lowercase();
                }
                "--terminal-output" => {
                    terminal_output = parts[1].to_string().to_lowercase();
                }
                _ => {
                    println!("This option not recognized: {}", parts[0].to_lowercase());
                }
            }
        } else {
            match parts[0].to_lowercase().as_str() {
                "--help" | "-h" => {
                    println!(
                        "
                    usage: 
                    --kitty-colors-file=PATH (default $HOME/.config/kitty/current-theme.conf)
                    --foot-folder=PATH (default $HOME/.config/foot)
                    --alacritty-folder=PATH (default $HOME/.config/alacritty)
                    --terminal-output=all/foot/alacritty (default all)
                    "
                    );
                    std::process::exit(0);
                }
                _ => {
                    println!("This option not recognized: {}", parts[0].to_lowercase());
                }
            }
        }
    }

    Options {
        kitty_config_path,
        foot_output_folder,
        terminal_output,
        alacritty_output_folder,
        theme_name,
    }
}

fn create_foot_theme(
    kitty_colors: &HashMap<String, String>,
    foot_path: &String,
    theme_name: &String,
) -> Result<(), String> {
    let file_conf_str = format!(
        "
    # -*- conf -*-
    # {:}
    [cursor]
    color=000000 {}

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
        kitty_colors.get("cursor").unwrap(),
        kitty_colors.get("background").unwrap(),
        kitty_colors.get("foreground").unwrap(),
        kitty_colors.get("color0").unwrap(),
        kitty_colors.get("color1").unwrap(),
        kitty_colors.get("color2").unwrap(),
        kitty_colors.get("color3").unwrap(),
        kitty_colors.get("color4").unwrap(),
        kitty_colors.get("color5").unwrap(),
        kitty_colors.get("color6").unwrap(),
        kitty_colors.get("color7").unwrap(),
        kitty_colors.get("color8").unwrap(),
        kitty_colors.get("color9").unwrap(),
        kitty_colors.get("color10").unwrap(),
        kitty_colors.get("color11").unwrap(),
        kitty_colors.get("color12").unwrap(),
        kitty_colors.get("color13").unwrap(),
        kitty_colors.get("color14").unwrap(),
        kitty_colors.get("color15").unwrap(),
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
    kitty_colors: &HashMap<String, String>,
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
        kitty_colors.get("background").unwrap(),
        kitty_colors.get("foreground").unwrap(),
        kitty_colors.get("color0").unwrap(),
        kitty_colors.get("color1").unwrap(),
        kitty_colors.get("color2").unwrap(),
        kitty_colors.get("color3").unwrap(),
        kitty_colors.get("color4").unwrap(),
        kitty_colors.get("color5").unwrap(),
        kitty_colors.get("color6").unwrap(),
        kitty_colors.get("color7").unwrap(),
        kitty_colors.get("color8").unwrap(),
        kitty_colors.get("color9").unwrap(),
        kitty_colors.get("color10").unwrap(),
        kitty_colors.get("color11").unwrap(),
        kitty_colors.get("color12").unwrap(),
        kitty_colors.get("color13").unwrap(),
        kitty_colors.get("color14").unwrap(),
        kitty_colors.get("color15").unwrap(),
    );

    let write_path = format!("{}/{}.yml", alacritty_path, theme_name.to_lowercase());

    println!("{}", file_conf_str);
    let result = fs::write(write_path, file_conf_str.trim());
    match result {
        Err(_) => Err("Error writing file".to_string()),
        Ok(_) => Ok(()),
    }
}

fn kitty_colors_to_base16_colors(kitty_colors_path: &String) -> HashMap<String, String> {
    let kitty_colors_file = fs::read_to_string(kitty_colors_path).unwrap();
    let mut kitty_colors: HashMap<String, String> = std::collections::HashMap::new();

    for line in kitty_colors_file.lines().into_iter() {
        match line.starts_with("color")
            || line.starts_with("cursor")
            || line.starts_with("background")
            || line.starts_with("foreground")
        {
            true => {
                if line.contains("cursor_text_color") {
                    continue;
                };

                let line_values = line.split("#").collect::<Vec<&str>>();
                kitty_colors.insert(
                    line_values[0].trim().to_string(),
                    line_values[1].trim().to_string(),
                );
            }
            false => (),
        }
    }

    return kitty_colors;
}

fn create_theme(term_name: &String, args: &Options) -> Result<(), String> {
    let kitty_colors = kitty_colors_to_base16_colors(&args.kitty_config_path);
    let mut result_foot: Result<(), String> = Ok(());
    let mut result_alacritty: Result<(), String> = Ok(());

    match term_name.as_str() {
        "all" => {
            result_foot =
                create_foot_theme(&kitty_colors, &args.foot_output_folder, &args.theme_name);
            result_alacritty = create_alacritty_theme(
                &kitty_colors,
                &args.alacritty_output_folder,
                &args.theme_name,
            );
            println!("all");
        }
        "foot" => {
            result_foot =
                create_foot_theme(&kitty_colors, &args.foot_output_folder, &args.theme_name);
            println!("foot");
        }

        "alacritty" => {
            result_alacritty = create_alacritty_theme(
                &kitty_colors,
                &args.alacritty_output_folder,
                &args.theme_name,
            );
            println!("alacritty");
        }
        &_ => {}
    }

    match (result_foot, result_alacritty) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(err_foot), _) => Err(err_foot),
        (_, Err(err_alacritty)) => Err(err_alacritty),
    }
}

fn main() {
    let args: Options = parse_args();
    let result_create_theme = create_theme(&args.terminal_output, &args);
    match result_create_theme {
        Err(err) => Err(err),
        Ok(_) => {
            println!("Theme {} parsed", args.theme_name);
            Ok(())
        }
    };
}
