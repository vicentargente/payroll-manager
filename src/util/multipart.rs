use actix_multipart::{Field, Multipart};
use futures_util::{StreamExt, TryStreamExt};
use serde::de::DeserializeOwned;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{config, error::error::{AppError, AppErrorType}};

/// Extracts a field named "body" from an Actix multipart payload and deserializes it into the specified type.
///
/// This function expects the multipart payload to contain a field named "body" as the first field.
/// The order of the fields in the multipart payload must match the order in which this function
/// tries to fetch the data. If the "body" field is not the first field, the function will return an error.
///
/// # Type Parameters
///
/// * `B`: The type into which the "body" field will be deserialized. This type must implement `DeserializeOwned`.
///
/// # Arguments
///
/// * `payload`: A mutable reference to the `Multipart` payload from which the "body" field will be extracted.
///
/// # Returns
///
/// * `Result<B, AppError>`: On success, returns the deserialized "body" field. On failure, returns an `AppError`.
///
/// # Errors
///
/// This function will return an `AppError` if:
///
/// * The "body" field is missing.
/// * There is an error reading the "body" field data.
/// * The "body" field data cannot be converted to a string.
/// * The "body" field data cannot be deserialized into the specified type.
///
/// # Example
///
/// ```rust
/// use actix_multipart::Multipart;
/// use serde::Deserialize;
/// use crate::util::multipart::extract_body;
///
/// #[derive(Deserialize)]
/// struct MyBody {
///     field1: String,
///     field2: i32,
/// }
///
/// async fn handle_multipart(mut payload: Multipart) -> Result<MyBody, AppError> {
///     let body: MyBody = extract_body(&mut payload).await?;
///     Ok(body)
/// }
/// ```
pub async fn extract_body<B>(payload: &mut Multipart) -> Result<B, AppError>
where
    B: DeserializeOwned
{
    if let Some(Ok(mut field)) = payload.next().await {
        let content_type = field.content_disposition().unwrap();

        if let Some(name) = content_type.get_name() {
            if name == "body" {
                let body_bytes = match field.next().await {
                    Some(body) => 
                        body.map_err(|err| AppError::new(
                            format!("Failed reading body data: {}", err),
                            AppErrorType::InternalServerError,
                            None
                        )),
                    None => {
                        return Err(AppError::new(
                            String::from("Missing body data"),
                            AppErrorType::BadRequest,
                            None
                        ))
                    },
                }?;
                
                let value = String::from_utf8(
                    body_bytes.to_vec()
                ).map_err(|err| AppError::new(
                    format!("Failed converting body data to string: {}", err),
                    AppErrorType::InternalServerError,
                    None
                ))?;

                let parsed_body = serde_json::from_str::<B>(&value)
                    .map_err(|err| AppError::new(
                        err.to_string(),
                        AppErrorType::BadRequest,
                        None
                    ))?;

                return Ok(parsed_body);
            }
        }
    }

    Err(AppError::new(
        String::from("Expected body"),
        AppErrorType::BadRequest,
        None
    ))
}

/// Extracts a field named "file" from an Actix multipart payload, saves the received file in a configured directory,
/// and returns the important data to access it later.
///
/// This function expects the multipart payload to contain a field named "file" as the first field.
/// The order of the fields in the multipart payload must match the order in which this function
/// tries to fetch the data. If the "file" field is not the first field, the function will return an error.
///
/// # Arguments
///
/// * `payload`: A mutable reference to the `Multipart` payload from which the "file" field will be extracted.
///
/// # Returns
///
/// * `Result<FileInfo, AppError>`: On success, returns a `FileInfo` struct containing the file path, original file name, and unique file name. On failure, returns an `AppError`.
///
/// # Errors
///
/// This function will return an `AppError` if:
///
/// * The "file" field is missing.
/// * There is an error reading the "file" field data.
/// * There is an error creating or writing to the temporary file.
/// * The file size exceeds the maximum allowed size.
///
/// # Example
///
/// ```rust
/// use actix_multipart::Multipart;
/// use crate::util::multipart::extract_file;
///
/// async fn handle_multipart(mut payload: Multipart) -> Result<FileInfo, AppError> {
///     let file_info = extract_file(&mut payload).await?;
///     Ok(file_info)
/// }
/// ```
pub async fn extract_file(payload: &mut Multipart) -> Result<FileInfo, AppError> {
    if let Some(Ok(mut field)) = payload.next().await {
        let content_type = field.content_disposition().unwrap();

        if let Some(name) = content_type.get_name() {
            if name == "file" {
                let file_name = match content_type.get_filename().map(|name| name.to_string()) {
                    Some(name) => name,
                    None => {
                        return Err(AppError::new(
                            String::from("Missing file name"),
                            AppErrorType::BadRequest,
                            None
                        ))
                    }
                };

                let uuid = Uuid::now_v7().to_string();

                let file_dir = &config::get().file.temp_upload_dir;
                let unique_file_name = uuid;
                let file_path = format!("{}/{}", file_dir, unique_file_name);

                // let start = tokio::time::Instant::now();

                if let Err(e) = process_file_receiving(&mut field, &file_path).await {
                    tokio::fs::remove_file(&file_path).await.map_err(|err| AppError::new(
                        format!("Failed removing file [{}]. Trigger error: {}", file_path, err),
                        AppErrorType::InternalServerError,
                        None
                    ))?;

                    return Err(e);
                }

                return Ok(FileInfo {
                    file_path,
                    original_file_name: file_name,
                    unique_file_name
                });
            }
        }
    }

    Err(AppError::new(
        String::from("Expected file"),
        AppErrorType::BadRequest,
        None
    ))
}

async fn process_file_receiving(field: &mut Field, file_path: &str) -> Result<(), AppError> {
    let max_size = config::get().file.max_size;

    let mut total_size = 0u64;
    let mut temp_file = File::create(&file_path).await.map_err(|err| AppError::new(
        format!("Failed creating temporal file: {}", err),
        AppErrorType::InternalServerError,
        None
    ))?;

    while let Some(chunk) = field.try_next().await
        .map_err(|err| AppError::new(
            format!("Failed reading file data: {}", err),
            AppErrorType::InternalServerError,
            None
        ))?
    {
        total_size += chunk.len() as u64;
        if total_size > max_size {
            return Err(AppError::new(
                format!("File size cannot exceed {} bytes", max_size),
                AppErrorType::BadRequest,
                None
            ));
        }

        temp_file.write_all(&chunk).await.map_err(|err| AppError::new(
            format!(
                "Failed writing file data (Chunk [{}-{}], Size: {}). Error: {}",
                total_size - chunk.len() as u64,
                total_size,
                chunk.len(),
                err
            ),
            AppErrorType::InternalServerError,
            None
        ))?;
    }

    Ok(())
}

// pub async fn process_file_request_with_body<B, F>(payload: &mut Multipart) -> Result<BodyAndFile<B>, AppError>
// where
//     B: DeserializeOwned
// {
//     let mut res_body: Option<B> = None;
//     let mut res_file_path: Option<String> = None;
//     let mut res_original_file_name: Option<String> = None;
//     let mut res_unique_file_name: Option<String> = None;

//     let mut index = 0; // To force client to send body first

//     while let Some(Ok(mut field)) = payload.next().await {
//         let content_type = field.content_disposition().unwrap();

//         if let Some(name) = content_type.get_name() {
//             match name {
//                 "body" => {
//                     if index != 0 {
//                         return Err(AppError::new(
//                             String::from(r#""body" field must be the first"#),
//                             AppErrorType::BadRequest,
//                             None
//                         ))
//                     }

//                     let body_bytes = match field.next().await {
//                         Some(body) => 
//                             body.map_err(|err| AppError::new(
//                                 format!("Failed reading body data: {}", err),
//                                 AppErrorType::InternalServerError,
//                                 None
//                             )),
//                         None => {
//                             return Err(AppError::new(
//                                 String::from("Missing body data"),
//                                 AppErrorType::BadRequest,
//                                 None
//                             ))
//                         },
//                     }?;
                    
//                     let value = String::from_utf8(
//                         body_bytes.to_vec()
//                     ).map_err(|err| AppError::new(
//                         format!("Failed converting body data to string: {}", err),
//                         AppErrorType::InternalServerError,
//                         None
//                     ))?;

//                     let parsed_body = serde_json::from_str::<B>(&value)
//                         .map_err(|err| AppError::new(
//                             err.to_string(),
//                             AppErrorType::BadRequest,
//                             None
//                         ))?;

//                     res_body = Some(parsed_body);
//                 },
//                 "file" => {
//                     if index != 1 {
//                         return Err(AppError::new(
//                             String::from(r#""file" field must be the second"#),
//                             AppErrorType::BadRequest,
//                             None
//                         ))
//                     }

//                     let file_name = match content_type.get_filename().map(|name| name.to_string()) {
//                         Some(name) => name,
//                         None => {
//                             return Err(AppError::new(
//                                 String::from("Missing file name"),
//                                 AppErrorType::BadRequest,
//                                 None
//                             ))
//                         }
//                     };

//                     let uuid = Uuid::now_v7().to_string();

//                     let file_dir = &config::get().file.temp_upload_dir;
//                     let unique_file_name = format!("{}_{}", uuid, file_name);
//                     let file_path = format!("{}/{}", file_dir, unique_file_name);

//                     let max_size = config::get().file.max_size;

//                     // let start = tokio::time::Instant::now();

//                     let mut total_size = 0u64;
//                     let mut temp_file = File::create(&file_path).await.map_err(|err| AppError::new(
//                         format!("Failed creating temporal file: {}", err),
//                         AppErrorType::InternalServerError,
//                         None
//                     ))?;

//                     while let Some(chunk) = field.try_next().await
//                         .map_err(|err| AppError::new(
//                             format!("Failed reading file data: {}", err),
//                             AppErrorType::InternalServerError,
//                             None
//                         ))?
//                     {
//                         total_size += chunk.len() as u64;
//                         if total_size > max_size {
//                             tokio::fs::remove_file(&file_path).await.map_err(|err| AppError::new(
//                                 format!("Failed removing file when exceeding max size: {}", err),
//                                 AppErrorType::InternalServerError,
//                                 None
//                             ))?;

//                             return Err(AppError::new(
//                                 format!("File size cannot exceed {} bytes", max_size),
//                                 AppErrorType::BadRequest,
//                                 None
//                             ));
//                         }

//                         temp_file.write_all(&chunk).await.map_err(|err| AppError::new(
//                             format!(
//                                 "Failed writing file data (Chunk [{}-{}], Size: {}). Error: {}",
//                                 total_size - chunk.len() as u64,
//                                 total_size,
//                                 chunk.len(),
//                                 err
//                             ),
//                             AppErrorType::InternalServerError,
//                             None
//                         ))?;
//                     }

//                     res_original_file_name = Some(file_name);
//                     res_unique_file_name = Some(unique_file_name);
//                     res_file_path = Some(file_path);

//                     // let elapsed = start.elapsed().as_millis();
//                     // println!("Time elapsed: {} ms", elapsed);
//                 },
//                 _ => {
//                     return Err(AppError::new(
//                         format!("Invalid field name: {}", name),
//                         AppErrorType::BadRequest,
//                         None
//                     ))
//                 }
//             }
//         }

//         index += 1;
//     }

//     if res_body.is_none() {
//         return Err(AppError::new(
//             String::from("Missing body data"),
//             AppErrorType::BadRequest,
//             None
//         ))
//     }

//     if res_file_path.is_none() || res_original_file_name.is_none() || res_unique_file_name.is_none() {
//         return Err(AppError::new(
//             String::from("Missing file data"),
//             AppErrorType::BadRequest,
//             None
//         ))
//     }

//     Ok(BodyAndFile {
//         body: res_body.unwrap(),
//         file_path: res_file_path.unwrap(),
//         original_file_name: res_original_file_name.unwrap(),
//         unique_file_name: res_unique_file_name.unwrap()
//     })
// }

pub struct FileInfo {
    pub file_path: String,
    pub original_file_name: String,
    pub unique_file_name: String
}
