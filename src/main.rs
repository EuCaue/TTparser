mod args;
mod theme_creator;
mod theme_parser;
use args::{parse_args, Options};
use theme_creator::{create_alacritty_theme, create_foot_theme, create_kitty_theeme};
use theme_parser::{
    alacritty_colors_to_base16_colors, kitty_colors_to_base16_colors, Base16Colors,
};

fn create_base16_colors(term_input_name: &String, term_input_file: &String) -> Base16Colors {
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
    println!("{:#?}", args);

    let result_create_theme = create_theme(&args.terminal_output, &args);
    let _ = match result_create_theme {
        Err(err) => Err(err),
        Ok(_) => {
            println!("Theme {} parsed", args.theme_name);
            Ok(())
        }
    };
}
