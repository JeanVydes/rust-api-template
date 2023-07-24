use std::{str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericResponse {
    pub message: String,
    pub data: Value,
    pub exited_code: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Genders {
    MALE,
    FEMALE,
    OTHER,
}

impl FromStr for Genders {
    type Err = ();

    fn from_str(s: &str) -> Result<Genders, Self::Err> {
        match s {
            "MALE" => Ok(Genders::MALE),
            "FEMALE" => Ok(Genders::FEMALE),
            "OTHER" => Ok(Genders::OTHER),
            _ => Ok(Genders::OTHER),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Currencies {
    COP,
    USD,
    EUR,
}

impl FromStr for Currencies {
    type Err = ();

    fn from_str(s: &str) -> Result<Currencies, Self::Err> {
        match s {
            "COP" => Ok(Currencies::COP),
            "USD" => Ok(Currencies::USD),
            "EUR" => Ok(Currencies::EUR),
            _ => Ok(Currencies::USD),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    // identificators
    pub id: u64,
    pub username: String,
    pub email: String,

    // security
    pub password: String,                   // store hash of password
    pub backup_security_codes: Vec<String>, // store hashes of backup securities

    // miscelaneous
    pub currency: Currencies,
    pub gender: Genders,
    pub preferences: Preferences,

    pub created_at: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub dark_mode: bool,
    pub language: String,
    pub notifications: bool,
}
