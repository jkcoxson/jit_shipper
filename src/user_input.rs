// jkcoxson

use dialoguer::{theme::ColorfulTheme, Select};

pub fn yes_no_prompt(prompt: &str) -> bool {
    let options = &["Yuh", "Nah"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    if selection == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn multi_input(prompt: &str, options: &[&str]) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    return options[selection].to_string();
}
