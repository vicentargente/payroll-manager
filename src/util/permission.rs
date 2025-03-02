#[macro_export]
macro_rules! check_permission {
    ($permission_check:expr) => {
        match $permission_check {
            Ok(has_permission) => {
                if !has_permission {
                    return crate::util::json_response::json_response(&Err(crate::error::error::AppError::new(
                        String::from("You do not have permission to access the requested resource"),
                        crate::error::error::AppErrorType::Forbidden,
                        None
                    )));
                }
            }
            Err(err) => return crate::util::json_response::json_response(&Err(err.into())),
        }
    };
}
