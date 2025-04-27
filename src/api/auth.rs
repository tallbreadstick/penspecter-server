use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

use super::log::{log, LogType};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String
}

#[derive(Serialize)]
pub struct RegisterResponse {
    msg: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    msg: String
}

pub enum AuthError {
    InvalidCredentials,
    PasswordHashFailed,
    UserAlreadyExists,
    EmailAlreadyUsed,
    DatabaseOperationFailed
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "InvalidCredentials").into_response()
            }
            AuthError::PasswordHashFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "PasswordHashFailed").into_response()
            },
            AuthError::UserAlreadyExists => {
                (StatusCode::CONFLICT, "UsernameAlreadyExists").into_response()
            },
            AuthError::EmailAlreadyUsed => {
                (StatusCode::CONFLICT, "EmailAlreadyUsed").into_response()
            },
            AuthError::DatabaseOperationFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseOperationFailed").into_response()
            }
        }
    }
}

#[axum::debug_handler]
pub async fn register(
    State(db): State<Pool<Postgres>>,
    Json(payload): Json<RegisterRequest>
) -> Result<Json<RegisterResponse>, AuthError> {
    log(LogType::HTTP, &format!("Anonymous user requested REGISTER as <{}> with <{}>", payload.username, payload.email));
    let user = sqlx::query!(
        "SELECT username, email FROM users WHERE username=$1 OR email=$2",
        payload.username,
        payload.email
    )
    .fetch_optional(&db)
    .await
    .map_err(|_| AuthError::DatabaseOperationFailed)?;
    if let Some(user) = user {
        if user.username == payload.username {
            return Err(AuthError::UserAlreadyExists);
        }
        if let Some(email) = user.email {
            if email == payload.email {
                return Err(AuthError::EmailAlreadyUsed);
            }
        }
    }
    let hash = hash_password(&payload.password)?;
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        payload.username,
        payload.email,
        hash
    )
    .fetch_one(&db)
    .await
    .map_err(|_| AuthError::DatabaseOperationFailed)?;
    Ok(Json(RegisterResponse { msg: String::new() }))
}

#[axum::debug_handler]
pub async fn login(
    State(db): State<Pool<Postgres>>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, AuthError> {
    log(LogType::HTTP, &format!("User <{}> requested LOGIN", payload.username));
    let user = sqlx::query!(
        "SELECT password FROM users WHERE username=$1",
        payload.username
    )
    .fetch_optional(&db)
    .await
    .map_err(|_| AuthError::DatabaseOperationFailed)?;
    let user = user.ok_or(AuthError::InvalidCredentials)?;
    if verify_password(&payload.password, &user.password)? {
        Ok(Json(LoginResponse { msg: String::new() }))
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::PasswordHashFailed)?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|_| AuthError::InvalidCredentials)?;

    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}