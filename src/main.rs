use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::{fs, vec};

fn main() {
    print_welcome();
    let home_path = env::var("HOME").expect("HOME not set");

    let selection = show_options(&home_path);
    let polybar_theme_type = show_polybar_options();
    let colors = replace_alacritty(&home_path, &selection);
    replace_polybar(&home_path, colors.to_owned(), &polybar_theme_type);
    replace_rofi(&home_path, colors.to_owned());
    replace_nvim_theme(&home_path, &selection);
    replace_init_lua_theme(&home_path, &selection);
    replace_dunst(&home_path, colors.to_owned());

    println!("Done!");
}

fn show_options(home_path: &str) -> String {
    let themes = load_available_themes(&home_path);

    println!("Themes found:");
    for (i, theme) in themes.iter().enumerate() {
        println!("{}. {}", i + 1, theme);
    }

    println!("Please select a theme by entering its corresponding number.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let selected_option: usize = input.trim().parse().expect("Invalid option");

    if selected_option > 0 && selected_option <= themes.len() {
        return themes[selected_option - 1].to_string();
    } else {
        return "".to_string();
    }
}

fn show_polybar_options() -> usize {
    println!("What kind of polybar do you want?");
    println!("1. Follow theme colors.");
    println!("2. Transparent black");
    println!("3. Transparent white");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input.trim().parse().expect("Invalid option");
}

fn replace_alacritty(home_path: &str, selection: &str) -> HashMap<String, String> {
    let alacritty_path = format!("{}/.alacritty.toml", home_path);
    let string_to_find = "~/.alacrittythemes/";
    let new_line_content = format!("import = [\"~/.alacrittythemes/{}\"]", selection);

    change_config_value(&alacritty_path, string_to_find, &new_line_content);

    let result = get_theme_colors(&format!("{}/.alacrittythemes/{}", &home_path, &selection));

    return result;
}

fn replace_polybar(home_path: &str, colors: HashMap<String, String>, theme_type: &usize) {
    let polybar_path = format!("{}/.config/polybar/config.ini", home_path);
    let string_to_find_bg = "background";
    let string_to_find_fg = "foreground";

    match theme_type {
        1 => {
            let new_line_content = format!("{}={}", string_to_find_bg, colors.get("bg").expect(""));
            change_config_value(&polybar_path, string_to_find_bg, &new_line_content);

            let new_line_content = format!("{}={}", string_to_find_fg, colors.get("fg").expect(""));
            change_config_value(&polybar_path, string_to_find_fg, &new_line_content);
        }
        2 => {
            let new_line_content = format!("{}={}", string_to_find_bg, "#30000000");
            change_config_value(&polybar_path, string_to_find_bg, &new_line_content);

            let new_line_content = format!("{}={}", string_to_find_fg, "#eaeaea");
            change_config_value(&polybar_path, string_to_find_fg, &new_line_content);
        }
        3 => {
            let new_line_content = format!("{}={}", string_to_find_bg, "#30eaeaea");
            change_config_value(&polybar_path, string_to_find_bg, &new_line_content);

            let new_line_content = format!("{}={}", string_to_find_fg, "#eaeaea");
            change_config_value(&polybar_path, string_to_find_fg, &new_line_content);
        }
        _ => println!("Invalid selection: {}", theme_type),
    }
}

fn replace_dunst(home_path: &str, colors: HashMap<String, String>) {
    let dunst_path = format!("{}/.config/dunst/dunstrc", home_path);
    let string_to_find_bg = "background";
    let string_to_find_fg = "foreground";

    let new_line_content = format!(
        "    {} = \"{}\"",
        string_to_find_bg,
        colors.get("bg").expect("")
    );
    change_config_value(&dunst_path, string_to_find_bg, &new_line_content);

    let new_line_content = format!(
        "    {} = \"{}\"",
        string_to_find_fg,
        colors.get("fg").expect("")
    );
    change_config_value(&dunst_path, string_to_find_fg, &new_line_content);
}

fn replace_rofi(home_path: &str, colors: HashMap<String, String>) {
    let polybar_path = format!("{}/.config/rofi/default.rasi", home_path);
    let string_to_find_bg_col = "bg-col";
    let string_to_find_bg_col_light = "bg-col-light";
    let string_to_find_border_col = "border-col";
    let string_to_find_selected_col = "selected-col";
    let string_to_find_selected_blue = "blue";
    let string_to_find_selected_fg_col = "fg-col";
    let string_to_find_selected_fg_col2 = "fg-col2";
    let string_to_find_selected_grey = "grey";

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_bg_col,
        colors.get("bg").expect("")
    );
    change_config_value(&polybar_path, string_to_find_bg_col, &new_line_content);

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_bg_col_light,
        colors.get("bg").expect("")
    );
    change_config_value(
        &polybar_path,
        string_to_find_bg_col_light,
        &new_line_content,
    );

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_border_col,
        colors.get("bg").expect("")
    );
    change_config_value(&polybar_path, string_to_find_border_col, &new_line_content);

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_selected_col,
        colors.get("bg").expect("")
    );
    change_config_value(
        &polybar_path,
        string_to_find_selected_col,
        &new_line_content,
    );

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_selected_blue,
        colors.get("blue").expect("")
    );
    change_config_value(
        &polybar_path,
        string_to_find_selected_blue,
        &new_line_content,
    );

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_selected_fg_col,
        colors.get("blue").expect("")
    );
    change_config_value(
        &polybar_path,
        string_to_find_selected_fg_col,
        &new_line_content,
    );

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_selected_fg_col2,
        colors.get("red").expect("")
    );
    change_config_value(
        &polybar_path,
        string_to_find_selected_fg_col2,
        &new_line_content,
    );

    let new_line_content = format!(
        "    {}:{};",
        string_to_find_selected_grey,
        colors.get("white").expect("")
    );
    change_config_value(
        &polybar_path,
        string_to_find_selected_grey,
        &new_line_content,
    );
}

fn replace_nvim_theme(home_path: &str, selection: &str) {
    let nvim_theme_path = format!("{}/.config/nvim/lua/theme.lua", home_path);
    let string_to_find_theme = "vim.cmd.colorscheme";

    let theme = match selection {
        "rose-pine-moon.toml" => "rose-pine-main",
        "catppuccin.toml" => "catppuccin",
        "gruvbox.toml" => "gruvbox",
        "gruvbox-material.toml" => "gruvbox-material",
        "tokyo-night.toml" => "tokyonight-moon",
        "kanagawa.toml" => "kanagawa",
        "everforest.toml" => "everforest",
        "poimandres.toml" => "poimandres",
        "nord.toml" => "nord",
        "zenbones.toml" => "zenbones",
        "dracula.toml" => "dracula",
        _ => "",
    };

    let new_line_content = format!("vim.cmd.colorscheme \"{}\"", theme);
    change_config_value(&nvim_theme_path, string_to_find_theme, &new_line_content);
}

fn replace_init_lua_theme(home_path: &str, selection: &str) {
    let init_lua_path = format!("{}/.config/nvim/lua/plugins/init.lua", home_path);
    let string_to_find_anchor = "    --theme";

    let contents = load_file(&init_lua_path);

    match contents {
        Ok(content) => {
            let anchor_line_index = get_line_number(&content, &string_to_find_anchor);

            if let Some(anchor_line_index) = anchor_line_index {
                let theme_line_below_index = anchor_line_index + 1;
                let theme_module = match selection {
                    "gruvbox.toml" => {
                        r#"    { "ellisonleao/gruvbox.nvim", priority = 1000, config = true, opts = ... },"#
                    }
                    "gruvbox-material.toml" => r#"    { "sainnhe/gruvbox-material" },"#,
                    "rose-pine-moon.toml" => r#"{ "rose-pine/neovim", name = "rose-pine" },"#,
                    "nord.toml" => r#"{ "shaunsingh/nord.nvim" },"#,
                    "catppuccin.toml" => {
                        r#"    { "catppuccin/nvim", name = "catppuccin", priority = 1000 },"#
                    }
                    "tokyo-night.toml" => {
                        r#"    { "folke/tokyonight.nvim", lazy = false, priority = 1000, opts = {} },"#
                    }
                    "kanagawa.toml" => r#"    { "rebelot/kanagawa.nvim" },"#,
                    "everforest.toml" => {
                        r#"    {"neanias/everforest-nvim", version = false, lazy = false, priority = 1000 },"#
                    }
                    "poimandres.toml" => {
                        r#"    { "olivercederborg/poimandres.nvim", priority = 1000 },"#
                    }
                    "zenbones.toml" => {
                        r#"    { "mcchrish/zenbones.nvim", dependencies = "rktjmp/lush.nvim" },"#
                    }
                    "dracula.toml" => r#"    { "Mofiqul/dracula.nvim" },"#,
                    _ => {
                        // Default case or handle other themes as needed
                        ""
                    }
                };

                let new_content = replace_line(&content, theme_line_below_index, &theme_module);

                save_file(&init_lua_path, new_content);
            } else {
                println!("Error: Anchor comment not found in the init.lua file.");
            }
        }
        Err(_) => {
            println!("Warning: File {} not found", &init_lua_path);
        }
    }
}

fn save_file(file_path: &str, content: String) {
    let mut file = File::create(file_path).expect("Could not open file");

    file.write_all(content.as_bytes())
        .expect("Error saving alacritty_path")
}

fn load_available_themes(home_path: &str) -> Vec<String> {
    let themes_path = format!("{}/.alacrittythemes", home_path);
    let paths = fs::read_dir(themes_path).expect("Failed to read directory");
    let mut themes = vec![];

    for path in paths {
        let path = path.expect("Invalid path");
        let filename = path.file_name().into_string().expect("invalid filename");
        if filename != "font.toml" {
            themes.push(path.file_name().into_string().expect("invalid filename"))
        }
    }

    return themes;
}

fn get_line_number(text: &str, word: &str) -> Option<usize> {
    text.lines()
        .enumerate()
        .find(|(_, line)| line.contains(word))
        .map(|(index, _)| index)
}

fn replace_line(text: &str, line_number: usize, new_line: &str) -> String {
    let lines: Vec<&str> = text.split("\n").collect();
    let mut new_lines = lines.clone();
    new_lines[line_number] = new_line;
    new_lines.join("\n")
}

fn get_theme_colors(selection: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut file = File::open(selection).expect("Could not open file");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Could not read file");

    let mut bg_color = get_key_value(&content, "background");
    let mut fg_color = get_key_value(&content, "foreground");
    let mut black = get_key_value(&content, "black");
    let mut red = get_key_value(&content, "red");
    let mut green = get_key_value(&content, "green");
    let mut yellow = get_key_value(&content, "yellow");
    let mut blue = get_key_value(&content, "blue");
    let mut magenta = get_key_value(&content, "magenta");
    let mut cyan = get_key_value(&content, "cyan");
    let mut white = get_key_value(&content, "white");

    if bg_color.len() > 7 {
        bg_color = bg_color.replace("0x", "#");
        fg_color = fg_color.replace("0x", "#");
        black = black.replace("0x", "#");
        red = red.replace("0x", "#");
        green = green.replace("0x", "#");
        yellow = yellow.replace("0x", "#");
        blue = blue.replace("0x", "#");
        magenta = magenta.replace("0x", "#");
        cyan = cyan.replace("0x", "#");
        white = white.replace("0x", "#");
    }

    result.insert("bg".to_owned(), bg_color.to_owned());
    result.insert("fg".to_owned(), fg_color.to_owned());
    result.insert("black".to_owned(), black.to_owned());
    result.insert("red".to_owned(), red.to_owned());
    result.insert("green".to_owned(), green.to_owned());
    result.insert("yellow".to_owned(), yellow.to_owned());
    result.insert("blue".to_owned(), blue.to_owned());
    result.insert("magenta".to_owned(), magenta.to_owned());
    result.insert("cyan".to_owned(), cyan.to_owned());
    result.insert("white".to_owned(), white.to_owned());

    return result;
}

fn find_and_extract_word_in_quotes(mut text: String) -> Option<String> {
    text = text.replace("'", "\"");
    let re = Regex::new(r#""([^"]*)"#).unwrap();
    re.captures(&text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

fn get_line_text(text: &str, line_number: usize) -> Option<String> {
    text.lines().nth(line_number).map(|line| line.to_string())
}

fn change_config_value(file_path: &str, string_to_find: &str, new_line_content: &str) {
    let contents = load_file(file_path);

    match contents {
        Ok(content) => {
            let line_number = get_line_number(&content, &string_to_find);

            let content = match line_number {
                Some(line) => replace_line(&content, line, &new_line_content),
                None => "".to_string(),
            };

            save_file(&file_path, content);
        }
        Err(_) => {
            println!("Warning: file {} not found", file_path)
        }
    }
}

fn get_key_value(content: &str, key: &str) -> String {
    let line_number = get_line_number(content, key).unwrap();
    let line_text = get_line_text(content, line_number).unwrap();
    let line_value = find_and_extract_word_in_quotes(line_text).unwrap();

    return line_value;
}

fn load_file(file_path: &str) -> Result<String, io::Error> {
    let file = File::open(file_path);

    let mut contents = String::new();
    match file {
        Ok(mut f) => {
            f.read_to_string(&mut contents)?;
            Ok(contents)
        }
        Err(error) => {
            println!("File not found!");
            Err(error)
        }
    }
}

fn print_welcome() {
    println!("  __            __     _       _     ");
    println!(" / _|          / _|   | |     | |    ");
    println!("| |_ __ _ _ __| |_ ___| |_ ___| |__  ");
    println!("|  _/ _` | '__|  _/ _ \\ __/ __| '_ \\ ");
    println!("| || (_| | |  | ||  __/ || (__| | | |");
    println!("|_| \\__,_|_|  |_| \\___|\\__\\___|_| |_|");
    println!();
}
