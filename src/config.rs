use std::sync::OnceLock;

pub struct Config {
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub file: FileConfig,
    pub bucket: BucketConfig
}

impl Config {
    fn new(database: DatabaseConfig, auth: AuthConfig, file: FileConfig, bucket: BucketConfig) -> Config {
        Config {
            database,
            auth,
            file,
            bucket
        }
    }
}

static INSTANCE: OnceLock<Config> = OnceLock::new();

pub fn initialize(database: DatabaseConfig, auth: AuthConfig, file: FileConfig, bucket: BucketConfig) {
    match INSTANCE.set(Config::new(database, auth, file, bucket)) {
        Ok(_) => (),
        Err(_) => panic!("Config already initialized"),
    };
}

pub fn get() -> &'static Config {
    INSTANCE.get().expect("Config not initialized")
}

pub struct DatabaseConfig {
    pub url: String
}

pub struct AuthConfig {
    pub secret: Vec<u8>
}

impl AuthConfig {
    pub fn secret_from_hex_string(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 || hex.len() < 2 {
            panic!("Invalid hex string");
        }

        let mut result = Vec::new();
        let mut iter = hex.chars();
    
        while let (Some(high), Some(low)) = (iter.next(), iter.next()) {
            let high_digit = high.to_digit(16).ok_or_else(|| format!("Invalid hex character: {}", high))?;
            let low_digit = low.to_digit(16).ok_or_else(|| format!("Invalid hex character: {}", low))?;
    
            let byte = (high_digit << 4) | low_digit;
            result.push(byte as u8);
        }

        Ok(result)
    }
}

pub struct FileConfig {
    pub temp_upload_dir: String,
    pub max_size: u64,
    pub max_uncompressed_size: u64
}

pub struct BucketConfig {
    pub host: String,
    pub access_key: String,
    pub secret_key: String,
    pub payroll_base_bucket_name: String,
}
