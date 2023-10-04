use std::{collections::HashMap, env, error, fs};

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
    theme_name: String,
}

fn parse_args() -> Options {
    let home = env::var("HOME").ok().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut kitty_config_path: String = format!("{}/.config/kitty/current-theme.conf", home);
    let mut foot_output_folder: String = format!("{}/.config/foot", home);
    let mut alacritty_output_folder: String = format!("{}/.config/alacritty", home);
    let mut theme_name: String = "current-theme-port".to_string();

    for arg in args.iter() {
        let parts: Vec<&str> = arg.splitn(2, '=').collect();
        if parts.len() == 2 {
            println!("{}", parts[0]);
            match parts[0] {
                "--kitty-colors-file" => {
                    kitty_config_path = parts[1].to_string();
                }
                "--theme-name" => {
                    theme_name = parts[1].to_string();
                }
                "--foot-folder" => {
                    foot_output_folder = parts[1].to_string();
                }

                "--alacritty-folder" => {
                    foot_output_folder = parts[1].to_string();
                }
                _ => {
                    println!("This option not recognized: {}", parts[0]);
                }
            }
        } else {
            match parts[0] {
                "--help" | "-h" => {
                    println!(
                        "
                    usage: 
                    --kitty-colors-file=PATH (default $HOME/.config/kitty/current-theme.conf)
                    --foot-folder=PATH (default $HOME/.config/foot)
                    "
                    );
                    std::process::exit(0);
                }
                _ => {
                    println!("This option not recognized: {}", parts[0]);
                }
            }
        }
    }

    Options {
        kitty_config_path,
        foot_output_folder,
        alacritty_output_folder,
        theme_name,
    }
}

//  TODO: make this return a option
fn create_foot_theme(
    kitty_colors: HashMap<String, String>,
    foot_path: String,
    theme_name: String,
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
    kitty_colors: HashMap<String, String>,
    alacritty_path: String,
    theme_name: String,
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

fn kitty_colors_to_base16_colors(kitty_colors_path: String) -> HashMap<String, String> {
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

fn main() {
    let config: Options = parse_args();

    let kitty_colors = kitty_colors_to_base16_colors(config.kitty_config_path);
    println!("{:#?}", kitty_colors);

    // let result = create_foot_theme(kitty_colors, config.foot_output_folder, config.theme_name);
    // if result.is_err() {
    //     println!("Error: {}", result.unwrap_err());
    // }

    let result = create_alacritty_theme(
        kitty_colors,
        config.alacritty_output_folder,
        config.theme_name,
    );
    if result.is_err() {
        println!("Error: {}", result.unwrap_err());
    }
}
