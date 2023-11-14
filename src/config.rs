use dotenvy::dotenv;

pub struct PrivateConf {
    pub db_url: String,
}
pub struct PublicConf {
    pub port: u16,
    pub db_connections: u32,
}
pub struct Config {
    pub private: PrivateConf,
    pub public: PublicConf,
}

impl Config {
    pub fn init() -> Config {
        // load env file as enviroment variables
        dotenv().ok();

        let private = PrivateConf {
            db_url: get_env("DATABASE_URL"),
        };

        let public = PublicConf {
            port: get_env("PORT"),
            db_connections: get_env("DB_CONNECTIONS")
        };

        Config { private, public }
    }
}

fn get_env<T: std::str::FromStr>(key: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    std::env::var(key)
        .expect("DATABASE_URL must be set")
        .trim()
        .parse::<T>()
        .expect(&format!("incorrect type for env '{key}'"))
}
