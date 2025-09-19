use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::{http::StatusCode, Error, FromRequest, Request, RequestBody, Result};

use crate::routes::user::Claims;


pub struct UserId(pub String);

impl<'a> FromRequest<'a> for UserId {
    async fn from_request(
            req: &'a Request,
            _body: &mut RequestBody,
        ) -> Result<Self> {
            let token = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("missing token", StatusCode::UNAUTHORIZED))?;

            // Get JWT_SECRET from environment variable
            let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| Error::from_string("JWT_SECRET not configured", StatusCode::INTERNAL_SERVER_ERROR))?;

        let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::default()).map_err(|_| Error::from_string("token malformed", StatusCode::UNAUTHORIZED))?;

        Ok(UserId(token_data.claims.sub))

    }
}