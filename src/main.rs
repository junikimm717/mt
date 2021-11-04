use home::home_dir;
use mt::config::Config;
use mt::time::Time;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use toml;

fn get_config(configpath: &PathBuf) -> Result<Result<Config, toml::de::Error>, std::io::Error> {
    match fs::read_to_string(configpath) {
        Ok(s) => Ok(Config::from(&s)),
        Err(e) => Err(e),
    }
}

fn open_url(config: &Config, url: &String) {
    let mut browsercmd = config.browser();
    if cfg!(target_os = "macos") {
        browsercmd = format!("open -a '{}'", browsercmd);
    }
    println!("Running '{} {}'", browsercmd, url);
    Command::new("sh")
        .arg("-c")
        .arg(browsercmd + " " + url)
        .spawn()
        .expect("Process failed");
}

fn auto_determine(config: &Config) {
    let time = Time::now();
    let mut vt = config.meetings_today();
    let hashmap = config.aliases_to_hashmap();
    vt.sort_unstable();
    for x in vt.iter().rev() {
        if x.0 <= time + config.time_threshold() {
            let url = hashmap.get(&x.1).unwrap();
            open_url(config, url);
            return;
        }
    }
}

fn determine_from_alias(config: &Config, alias: String) {
    match config.aliases_to_hashmap().get(&alias) {
        Some(x) => open_url(config, x),
        None => {
            eprintln!("No existing meeting for alias {}", &alias);
        }
    }
}

/// check the configuration.
fn check(config: &Config) {
    config.check_syntax();
}

fn create_config(configpath: &PathBuf) {
    let mut input = String::new();
    println!(
        "Are you sure you want to overwrite any existing configurations at ~/.config/mt/config_v2.toml? [Y/n]"
    );
    std::io::stdin().read_line(&mut input).expect("fail");
    if input.len() >= 1 && input.chars().next().unwrap() == 'Y' {
        Command::new("sh")
            .arg("-c")
            .arg(String::from("mkdir -p ") + (*configpath.parent().unwrap()).to_str().unwrap())
            .output()
            .expect("Could not create directory");
        std::fs::write(
            configpath,
            &toml::to_string::<Config>(&Config::default()).unwrap()[..],
        )
        .expect("failed to write");
    } else {
        println!("creation of default config aborted");
        return;
    }
}

fn main() {
    let mut configpath: PathBuf = home_dir().unwrap();
    configpath.push(".config");
    configpath.push("mt");
    configpath.push("config_v2.toml");

    if std::env::args().nth(1) == Some("--config".to_string()) {
        create_config(&configpath);
        return;
    }
    let config = match get_config(&configpath) {
        Ok(fileread) => match fileread {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Unable to parse the config file. {:?}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("Unable to read the config file. {:?}", e);
            return;
        }
    };
    match std::env::args().nth(1) {
        Some(x) => {
            if x == "--check" {
                check(&config);
            } else {
                determine_from_alias(&config, x)
            }
        }
        None => {
            auto_determine(&config)
        }
    }
}
