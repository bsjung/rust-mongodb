use dotenv::dotenv;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub server: String,
    pub database: String,
    pub database_url: String,
    pub jwt_expiration: i64,
    pub jwt_key: String,
    pub auth_salt: String,
    pub session_key: String,
    pub session_name: String,
    pub session_secure: bool,
    pub session_timeout: i64,
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = get_config();
        assert_ne!(config.server, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.server, "".to_string());
    }
}
