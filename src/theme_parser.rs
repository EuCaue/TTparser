use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

type ColorName = String;
type ColorHex = String;

pub type Base16Colors = HashMap<ColorName, ColorHex>;

pub fn alacritty_colors_to_base16_colors(alacritty_colors_path: &str) -> Base16Colors {
    let file = File::open(alacritty_colors_path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut base16_colors: Base16Colors = HashMap::new();
    let mut base16_color_number: i32 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            // skipping comments
            if line.starts_with("#") {
                continue;
            }
            println!("line {}", line);
            let line_trimmed = line.trim();
            if let Some(index) = line_trimmed.find(":") {
                let (color_name, color_hex) = line_trimmed.split_at(index);
                let color_hex = color_hex
                    .trim()
                    .trim_matches(|c: char| c == '"' || c == ':' || c.is_whitespace())
                    .replace("#", "")
                    .replace("'", "");
                match color_name {
                    "background" => {
                        base16_colors.insert("background".to_string(), color_hex.to_string());
                    }
                    "foreground" => {
                        base16_colors.insert("foreground".to_string(), color_hex.to_string());
                    }
                    "cursor" => {
                        base16_colors.insert("cursor".to_string(), color_hex.to_string());
                    }
                    "text" => {
                        base16_colors.insert("cursor_fg".to_string(), color_hex.to_string());
                    }
                    _ => {
                        // skiping color fields
                        if color_name.starts_with("color")
                            || color_name.starts_with("primary")
                            || color_name.starts_with("cursor")
                            || color_name.starts_with("normal")
                            || color_name.starts_with("bright")
                        {
                            continue;
                        }
                        base16_colors.insert(
                            format!("color{}", base16_color_number),
                            color_hex.to_string(),
                        );
                        base16_color_number += 1;
                    }
                }
            }
        }
    }
    return base16_colors;
}

pub fn kitty_colors_to_base16_colors(kitty_colors_path: &String) -> Base16Colors {
    let kitty_colors_file = fs::read_to_string(kitty_colors_path).unwrap();
    let mut base16_colors: Base16Colors = HashMap::new();

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

                let index = line.find("#").unwrap();

                let (color_name, color_hex) = line.split_at(index);
                let color_hex = color_hex.trim().replace("#", "");
                println!("color_hex: {}", color_hex);

                //  checking if the color is a reference for another color token
                if check_color(color_hex.as_str()) {
                    if line.contains("cursor_text_color") {
                        base16_colors.insert(
                            "cursor_fg".to_string(),
                            base16_colors
                                .get(&color_name.to_string())
                                .unwrap()
                                .to_string(),
                        );
                        continue;
                    }
                    base16_colors.insert(
                        color_name.to_string(),
                        base16_colors
                            .get(&color_name.trim().to_string())
                            .unwrap()
                            .to_string(),
                    );
                    continue;
                }
                if line.contains("cursor_text_color") {
                    base16_colors.insert("cursor_fg".to_string(), color_hex);
                    continue;
                }
                base16_colors.insert(color_name.trim().to_string(), color_hex);
            }
            false => println!("Nop."),
        }
    }
    return base16_colors;
}
