use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use poem_openapi::Object;

#[derive(sqlx::FromRow, Serialize, Deserialize, Object)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub mail: String,
    pub password: String,
    pub update_time: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Object)]
pub struct DeviceHard {
    pub id: i32,
    pub hard_version: String,
    pub name: String,
    pub category: i32, // 1锁, 2盒子
    pub has_ble: bool,
    pub has_finger: bool,
    pub has_stm32: bool,
    pub desc: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Object)]
pub struct DeviceSoft {
    pub id: i32,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Object)]
pub struct Firm {
    pub id: i32,
    pub hard_version: i32,
    pub version_name: String,
    pub version_format: String,
    pub version_type: i32,
    pub finger_level: i32,
    pub url: String,
    pub desc: String,
    pub update_time: DateTime<Utc>,
    pub rely_version_type: Option<i32>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub des_en: String,
    pub des_ko: String,
    pub des_sp: String,
}
