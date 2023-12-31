use crate::theme_parser::Base16Colors;
use std::fs;

pub fn create_foot_theme(
    base16_colors: &Base16Colors,
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
        base16_colors.get("cursor").unwrap_or(&"FFFFFF".to_string()),
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

pub fn create_alacritty_theme(
    base16_colors: &Base16Colors,
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
        base16_colors.get("cursor").unwrap_or(&"FFFFFF".to_string()),
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

pub fn create_kitty_theeme(
    base16_colors: &Base16Colors,
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
        base16_colors.get("cursor").unwrap_or(&"FFFFFF".to_string()),
        base16_colors
            .get("cursor_fg")
            .unwrap_or(&"000000".to_string()),
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
    println!("kitty_colors: from foot_theme {:#?}", write_path);

    println!("{}", file_conf_str);
    let result = fs::write(write_path, file_conf_str.trim());
    match result {
        Err(_) => Err("Error writing file".to_string()),
        Ok(_) => Ok(()),
    }
}
