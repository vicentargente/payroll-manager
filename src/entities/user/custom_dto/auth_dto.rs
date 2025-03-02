use serde::Serialize;

use crate::user::user::RetrieveUserDto;

#[derive(Serialize)]
pub struct AuthDto {
    pub token: String,
    pub user: RetrieveUserDto
}
