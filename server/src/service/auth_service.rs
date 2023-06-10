use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder, error::InternalError, Error,
};
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde_json::json;

use crate::{
    api::response_handler::{handle_conflict, handle_error, handle_success},
    entity::user,
    model::{
        auth_model::RegisterUserSchema,
        user_model::{self, Filter},
    },
    AppState,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{offset::Utc, FixedOffset};
use jsonwebtoken::{encode, EncodingKey, Header};

pub async fn register(body: Json<RegisterUserSchema>, data: Data<AppState>) -> HttpResponse {
    let query = Filter {
        user_name: Some(body.user_name.clone()),
        user_username: Some(body.user_username.clone()),
        user_mobile: Some(body.user_mobile.clone()),
        user_email: Some(body.user_email.clone()),
        ..Default::default()
    };
    let user = user_model::find_one_by_query(body, data).await;
    let user = match user {
        Ok(user) => user,
        Err(err) => return handle_error(err),
    };

    if user {
        if user.user_username == body.user_username {
            return handle_conflict("user", "username");
        }
        if user.user_mobile == body.user_mobile {
            return handle_conflict("user", "mobile");
        }
        if user.user_email == body.user_email {
            return handle_conflict("user", "email");
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password =
        match Argon2::default().hash_password(body.user_password.as_bytes(), &salt) {
            Ok(hashed_password) => hashed_password.to_string(),
            Err(_) => return handle_error(_),
        };

    let mut user = user::ActiveModel {
        user_name: ActiveValue::Set(body.user_name.to_string()),
        user_username: ActiveValue::Set(body.user_username.to_string()),
        user_mobile: ActiveValue::Set(body.user_mobile.to_string()),
        user_email: ActiveValue::Set(body.user_email.to_string()),
        user_password: ActiveValue::Set(hashed_password.to_string()),
        user_created_at: ActiveValue::Set(
            Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
        ),
        user_updated_at: ActiveValue::Set(
            Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()),
        ),
        ..Default::default()
    }
    .insert(&data.db)
    .await;

    match user {
        Ok(_) => handle_success("user", user),
        Err(err) => handle_error(err),
    }
}
