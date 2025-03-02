pub mod entities;
pub mod error;
pub mod util;
pub mod auth;
pub mod config;
pub mod service;

use std::env;

pub use entities::user;


pub fn initialize_config() {
    //DATABASE
    const DATABASE_URL: &str = "DATABASE_URL";
    //AUTH
    const AUTH_SECRET: &str = "AUTH_SECRET";
    //FILE
    const FILE_TEMP_DIR: &str = "FILE_TEMP_DIR";
    const FILE_MAX_SIZE: &str = "FILE_MAX_SIZE";
    const FILE_MAX_UNCOMPRESSED_SIZE: &str = "FILE_MAX_UNCOMPRESSED_SIZE";
    //BUCKET
    const BUCKET_HOST: &str = "BUCKET_HOST";
    const BUCKET_ACCESS_KEY: &str = "BUCKET_ACCESS_KEY";
    const BUCKET_SECRET_KEY: &str = "BUCKET_SECRET_KEY";
    const BUCKET_PAYROLL_BASE_BUCKET_NAME: &str = "BUCKET_PAYROLL_BASE_BUCKET_NAME";


    let database_config = config::DatabaseConfig {
        url: env::var(DATABASE_URL).expect(format!("{} must be a valid sqlite url", DATABASE_URL).as_str())
    };

    let auth_config = {
        let auth_secret = config::AuthConfig::secret_from_hex_string(
            &env::var(AUTH_SECRET)
            .expect(format!("{} must be a valid hex string", AUTH_SECRET).as_str())
        )
        .unwrap_or_else(|e| panic!("Invalid {}: {}", AUTH_SECRET, e));

        config::AuthConfig {
            secret: auth_secret
        }
    };

    let file_config = config::FileConfig {
        temp_upload_dir: env::var(FILE_TEMP_DIR)
            .expect(format!("{} must be a valid directory", FILE_TEMP_DIR).as_str()),
        max_size: env::var(FILE_MAX_SIZE)
            .expect(format!("{} must be a valid number", FILE_MAX_SIZE).as_str())
            .parse()
            .expect(format!("{} must be a valid number", FILE_MAX_SIZE).as_str()),
        max_uncompressed_size: env::var(FILE_MAX_UNCOMPRESSED_SIZE)
            .expect(format!("{} must be a valid number", FILE_MAX_UNCOMPRESSED_SIZE).as_str())
            .parse()
            .expect(format!("{} must be a valid number", FILE_MAX_UNCOMPRESSED_SIZE).as_str())
    };

    let bucket_config = config::BucketConfig {
        host: env::var(BUCKET_HOST)
            .expect(format!("{} must be a valid host", BUCKET_HOST).as_str()),
        access_key: env::var(BUCKET_ACCESS_KEY).
            expect(format!("{} must be a valid access key", BUCKET_ACCESS_KEY).as_str()),
        secret_key: env::var(BUCKET_SECRET_KEY).
            expect(format!("{} must be a valid secret key", BUCKET_SECRET_KEY).as_str()),
        payroll_base_bucket_name: env::var(BUCKET_PAYROLL_BASE_BUCKET_NAME).
            expect(format!("{} must be a valid bucket name", BUCKET_PAYROLL_BASE_BUCKET_NAME).as_str())
    };

    config::initialize(database_config, auth_config, file_config, bucket_config);
}
