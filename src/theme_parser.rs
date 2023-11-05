use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

pub fn alacritty_colors_to_base16_colors(alacritty_colors_path: &str) -> HashMap<String, String> {
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

pub fn kitty_colors_to_base16_colors(kitty_colors_path: &String) -> HashMap<String, String> {
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
