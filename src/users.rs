use crate::helpers::payload_analyzer;
use crate::primitives::{Preferences, User};
use crate::requests_interfaces::SignUp;
use crate::{primitives::GenericResponse, server::AppState};

use axum::{extract::rejection::JsonRejection, http::StatusCode, Json};
use mongodb::{bson::doc, Collection};
use regex::Regex;
use serde_json::json;
use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};

pub async fn create_account(
    payload_result: Result<Json<SignUp>, JsonRejection>,
    state: Arc<AppState>,
) -> (StatusCode, Json<GenericResponse>) {
    let payload = match payload_analyzer(payload_result) {
        Ok(payload) => payload,
        Err((status_code, json)) => return (status_code, json),
    };

    if payload.username.len() < 2 || payload.username.len() > 15 {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from(
                    "invalid username, must be at least 2 characters and at most 15",
                ),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    if payload.email.len() < 5 || payload.email.len() > 100 {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from(
                    "invalid email, must be at least 5 characters and at most 100",
                ),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    if payload.password.len() < 8 || payload.password.len() > 100 {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from("invalid password, must be at least 8 characters"),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    let username_re = Regex::new(r"^[a-zA-Z0-9_]{2,15}$").unwrap();
    let email_re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    let password_re = Regex::new(r"^[a-zA-Z0-9_]{8,20}$").unwrap();

    if !username_re.is_match(&payload.username) {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from("invalid username"),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    if !email_re.is_match(&payload.email) {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from("invalid email"),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    if !password_re.is_match(&payload.password) {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from("invalid password"),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    if payload.password != payload.password_confirmation {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from("password and password confirmation must match"),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    if payload.username.to_lowercase() == payload.password.to_lowercase() {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse {
                message: String::from("username and password must be different"),
                data: json!({}),
                exited_code: 1,
            }),
        );
    }

    let collection: Collection<User> = state.mongo_db.collection("users");
    let filter = doc! {"$or": [
        {"username": &payload.username.to_lowercase()},
        {"email": &payload.email.to_lowercase()},
    ]};

    match collection.find_one(filter, None).await {
        Ok(user) => match user {
            Some(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(GenericResponse {
                        message: String::from("username or email already taken"),
                        data: json!({}),
                        exited_code: 1,
                    }),
                )
            }
            None => (),
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenericResponse {
                    message: String::from("error checking username/email availability"),
                    data: json!({}),
                    exited_code: 1,
                }),
            )
        }
    }

    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(hashed_password) => hashed_password,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenericResponse {
                    message: String::from("error hashing password"),
                    data: json!({}),
                    exited_code: 1,
                }),
            )
        }
    };


    let user = User {
        id: state.last_user_id + 1,
        username: payload.username.to_lowercase(),
        email: payload.email.to_lowercase(),
        password: hashed_password,
        backup_security_codes: vec![],
        currency: payload.currency,
        gender: payload.gender,
        preferences: Preferences {
            dark_mode: false,
            language: String::from("en"),
            notifications: true,
        },
    };

    match collection.insert_one(user.clone(), None).await {
        Ok(_) => (),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenericResponse {
                    message: String::from("error inserting user into database"),
                    data: json!({}),
                    exited_code: 1,
                }),
            )
        }
    }

    (
        StatusCode::CREATED,
        Json(GenericResponse {
            message: String::from("user registered successfully"),
            data: json!(user),
            exited_code: 0,
        }),
    )
}
