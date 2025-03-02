use std::path::Path;

use minio::s3::{args::{BucketExistsArgs, MakeBucketArgs}, builders::ObjectContent, client::{Client, ClientBuilder}, creds::StaticProvider, http::BaseUrl, types::S3Api};

use crate::error::error::{AppError, AppErrorType};

pub struct MinioService {
    client: Client
}

impl MinioService {
    pub fn new(host: &str, access_key: &str, secret_key: &str) -> MinioService {
        let base_url = host.parse::<BaseUrl>().expect("Invalid base url");
        let static_provider = StaticProvider::new(
            access_key,
            secret_key,
            None
        );
    
        let client = ClientBuilder::new(base_url.clone())
            .provider(Some(Box::new(static_provider)))
            .build()
            .expect("Failed to create client");
    
        MinioService {
            client
        }
    }

    pub async fn create_bucket_if_not_exists(&self, bucket_name: &str) -> Result<(), AppError>  {
        let exists: bool = self.client
            .bucket_exists(&BucketExistsArgs::new(bucket_name).unwrap())
            .await
            .map_err(|err| AppError::new(
                format!("Failed to check if bucket exists: {}", err),
                AppErrorType::InternalServerError,
                None
            ))?;
    
        if !exists {
            self.client
                .make_bucket(&MakeBucketArgs::new(bucket_name).unwrap())
                .await
                .unwrap();
        };
    
        Ok(())
    }

    pub async fn upload_file(&self, bucket_name: &str, file_path: &str, object_name: &str) -> Result<(), AppError> {
        let file_path = Path::new(file_path);
        if !file_path.exists() {
            return Err(AppError::new(
                format!("File not found: {}", file_path.display()),
                AppErrorType::NotFound,
                None
            ));
        }

        let content = ObjectContent::from(file_path);
        self.client
            .put_object_content(bucket_name, object_name, content)
            .send()
            .await
            .map_err(
                |err| AppError::new(
                    format!("Failed to upload file: {}", err),
                    AppErrorType::InternalServerError,
                    None
                )
            )?;

        Ok(())
    }

    pub async fn remove_file(&self, bucket_name: &str, object_name: &str) -> Result<(), AppError> {
        self.client
            .remove_object(bucket_name, object_name)
            .send()
            .await
            .map_err(
                |err| AppError::new(
                    format!("Failed to remove file: {}", err),
                    AppErrorType::InternalServerError,
                    None
                )
            )?;

        Ok(())
    }
}
