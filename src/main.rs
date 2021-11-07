use home::home_dir;
use mt::config::Config;
use mt::opt::Opt;
use mt::time::Time;
use std::env::var;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use toml;

fn get_config(configpath: &PathBuf) -> Result<Result<Config, toml::de::Error>, std::io::Error> {
    match fs::read_to_string(configpath) {
        Ok(s) => Ok(Config::from(&s)),
        Err(e) => Err(e),
    }
}

fn config_exists(configpath: &PathBuf) -> bool {
    std::path::Path::new(&configpath).exists()
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

fn edit_config(editor: String, configpath: &PathBuf) {
    if !config_exists(&configpath) {
        create_config(&configpath);
    }
    Command::new("sh")
        .arg("-c")
        .arg(editor + " " + configpath.to_str().unwrap())
        .status()
        .expect("Could not edit configuration");
    check(&configpath);
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
fn check(configpath: &PathBuf) {
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
    config.check_semantics();
}

/// creates a new connfiguration file
/// warns the user if a configuration file is already present.
fn create_config(configpath: &PathBuf) {
    let overwrite = || {
        Command::new("sh")
            .arg("-c")
            .arg(String::from("mkdir -p ") + (*configpath.parent().unwrap()).to_str().unwrap())
            .output()
            .expect("Could not create directory");
        std::fs::write(
            configpath,
            &toml::to_string::<Config>(&Config::default()).unwrap(),
        )
        .expect("failed to write");
    };
    if config_exists(&configpath) {
        let mut input = String::new();
        println!(
        "Are you sure you want to overwrite any existing configurations at ~/.config/mt/config_v2.toml? [Y/n]"
    );
        std::io::stdin().read_line(&mut input).expect("fail");
        if input.len() >= 1 && input.chars().next().unwrap() == 'Y' {
            overwrite();
        } else {
            println!("creation of default config aborted");
            return;
        }
    } else {
        overwrite();
    }
}

fn main() {
    let configpath: PathBuf = {
        let mut x = home_dir().unwrap();
        x.push(".config");
        x.push("mt");
        x.push("config_v2.toml");
        x
    };
    let args = Opt::from_args();

    if args.configure {
        create_config(&configpath);
        return;
    } else if args.edit {
        let editor = match var("MT_EDITOR") {
            Ok(x) => x,
            Err(_) => "vi".to_owned(),
        };
        edit_config(editor, &configpath);
        return;
    }

    if args.check {
        check(&configpath);
    } else {
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
        if let Some(alias) = args.alias {
            determine_from_alias(&config, alias);
        } else {
            auto_determine(&config);
        }
    }
}
