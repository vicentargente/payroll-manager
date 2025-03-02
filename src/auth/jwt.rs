use std::future::{ready, Ready};
use actix_web::{http, Error as ActixWebError, FromRequest};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

pub fn generate_token(user_id: i64) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    let config = crate::config::get();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&config.auth.secret),
    )
    .expect("Failed to encode token")
}

// async fn validate_token(req: &ServiceRequest) -> Result<TokenData<Claims>, Error> {
//     // Extract the Authorization header
//     let auth_header = req.headers().get("Authorization");
//     let token = match auth_header.and_then(|h| h.to_str().ok()) {
//         Some(header) => header.trim_start_matches("Bearer ").trim(),
//         None => return Err(actix_web::error::ErrorUnauthorized("Missing Authorization header")),
//     };

//     // Decode and validate the token
//     decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
//         &Validation::default(),
//     )
//     .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))
// }

impl FromRequest for Claims {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get(http::header::AUTHORIZATION);
        let token = match auth_header.and_then(|h| h.to_str().ok()) {
            Some(header) => header.trim_start_matches("Bearer ").trim(),
            None => return ready(Err(actix_web::error::ErrorUnauthorized("Missing Authorization header"))),
        };

        let config = crate::config::get();

        ready(decode::<Claims>(
            token,
            &DecodingKey::from_secret(&config.auth.secret),
            &Validation::default()
        )
        .map(|data| data.claims)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token")))
    }
}
