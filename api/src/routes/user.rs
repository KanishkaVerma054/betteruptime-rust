use std::sync::{Arc, Mutex};
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::http::{StatusCode};
use poem::Error;
//poem for routing
use poem::{
    handler, web::{Data, Json}
};
use serde::{Deserialize, Serialize};
use crate::{request_inputs::{CreateUserInput}, request_outputs::{CreateUserOutput, SigninOutput}};
use store::{store::Store};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize
}

#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<CreateUserOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password).map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    let response = CreateUserOutput {
        id
    };

    Ok(Json(response))
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<SigninOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let my_claims = Claims {
                sub: user_id,
                exp: 11111111111111111
            };

            //moved JWT_SECRET to an env file (hint: dotenv)
            let jwt_secret = std::env::var("JWT_SECRET")
                .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_secret.as_ref())).map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SigninOutput {
                jwt: token
            };
            Ok(Json(response))
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED))
    }

    

    
}