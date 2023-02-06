use serde::Deserialize;
use std::fs::write;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
}

pub fn create_config_file_in_user_home_dir(token: Option<String>) {
    let home_dir = dirs::home_dir().unwrap();
    let config_dir = home_dir.join(".config");
    let config_file = config_dir.join("command_recall.toml");

    if config_file.exists() {
        eprintln!("Config file already exists: {:?}", config_file);
        eprintln!("command_recall.toml already exists, please remove it to create a new one.");
        return;
    }
    // create the config file
    if token.is_none() {
        write(&config_file, "token = ").expect("Failed to create config file");
        println!("Please add your OpenAI API key to the config file: {:?}", config_file);
        println!("You can find your API key at https://platform.openai.com/account/api-keys");
    } else {
        write(&config_file, format!("token = \"{}\"", token.unwrap()))
            .expect("Failed to create config file");
    }
    println!("Config file: {:?}", config_file);
}

// read the config file and return a Config struct
pub fn read_config_file() -> Config {
    let home_dir = dirs::home_dir().unwrap();
    let config_dir = home_dir.join(".config");
    let config_file = config_dir.join("command_recall.toml");
    //parse the toml with the toml crate
    let config_file_content = match std::fs::read_to_string(&config_file) {
        Ok(content) => content,
        Err(_) => {
            println!("Config file not found: {:?}", config_file);
            println!("Please run command_recall configure to create the config file");
            std::process::exit(1);
        }
    };
    let config: Config = match toml::from_str(&config_file_content) {
        Ok(config) => config,
        Err(_) => {
            println!("Config file is not valid: {:?}", config_file);
            println!("Please check the config file");
            std::process::exit(1);
        }
    };
    config
}
