use crate::primitives::{Currencies, Genders};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignIn {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUp {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub currency: Currencies,
    pub gender: Genders,
}
