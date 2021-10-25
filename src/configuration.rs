use std::rc::Rc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SettingsInner {
    pub database: DatabaseSettings,
    pub server: ServerSettingsInner,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize)]
pub struct ServerSettingsInner {
    pub host: String,
    pub application_port: u16,
    pub secure: bool,
}

pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(Clone)]
pub struct ServerSettings(Rc<ServerSettingsInner>);

impl ServerSettings {
    pub fn application_port(&self) -> u16 {
        self.0.application_port
    }

    pub fn host(&self) -> &str {
        &self.0.host
    }

    pub fn public_addr(&self) -> String {
        format!("{}:{}", &self.0.host, self.0.application_port)
    }

    pub fn secure(&self) -> bool {
        self.0.secure
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize the configuration reader
    let mut declared_settings = config::Config::default();

    // Add configuration values from a top-level file named
    // `configuration` that `config` knows how to parse:
    // yaml, json, etc.
    declared_settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values it reads into
    // our Settings type
    let declared_settings: SettingsInner = declared_settings.try_into()?;

    Ok(Settings {
        database: declared_settings.database,
        server: ServerSettings {
            0: Rc::new(declared_settings.server),
        },
    })
}
