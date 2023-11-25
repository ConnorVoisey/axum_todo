pub mod log;

use dotenvy::dotenv;
use log::{get_log_level, LogOutput};
use tracing::level_filters::LevelFilter;

pub struct PrivateConf {
    pub db_url: String,
}
pub struct PublicConf {
    pub port: u16,
    pub db_connections: u32,
    pub log_dir: String,
    pub log_console_level: LevelFilter,
    pub log_file_level: LevelFilter,
    pub log_file_name: String,
}
pub struct Config {
    pub private: PrivateConf,
    pub public: PublicConf,
}

impl Config {
    pub fn init() -> Config {
        // load env file as enviroment variables
        dotenv().expect("failed to read .env file");

        let private = PrivateConf {
            db_url: get_env("DATABASE_URL", None),
        };

        let public = PublicConf {
            port: get_env("PORT", None),
            db_connections: get_env("DB_CONNECTIONS", None),
            log_dir: get_env("LOG_DIR", Some(String::from("./logs"))),
            log_console_level: get_log_level(LogOutput::Console),
            log_file_level: get_log_level(LogOutput::File),
            log_file_name: get_env("LOG_FILE_NAME", Some(String::from("log"))),
        };

        Config { private, public }
    }
}

pub fn get_env<T: std::str::FromStr>(key: &str, default: Option<T>) -> T
where
    T::Err: std::fmt::Debug,
{
    match std::env::var(key) {
        Ok(val) => val
            .trim()
            .parse::<T>()
            .unwrap_or_else(|_| panic!("incorrect type for env '{key}'")),
        Err(err) => match default {
            Some(default_val) => default_val,
            None => panic!(
                "Missing required env variable {key}
Error: {err}"
            ),
        },
    }
}
