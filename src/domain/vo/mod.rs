use super::dto::{DeviceHard, DeviceSoft, Firm, User};
use poem::{http::StatusCode, Error as PError};
use poem_openapi::{Object, Enum};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct Token {
    pub access_token: String,
    // pub refresh_token: String,
    pub user: VoUser,
}

impl Token {
    pub fn new(access_token: String, user: VoUser) -> Token {
        Token {
            access_token,
            // refresh_token,
            user,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Object)]
pub struct VoUser {
    pub id: i32,
    pub name: String,
    pub mail: String,
    pub ticker: i64,
}

impl From<User> for VoUser {
    fn from(u: User) -> Self {
        VoUser {
            id: u.id,
            name: u.name,
            mail: u.mail,
            ticker: u.update_time.timestamp(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Object)]
pub struct VoUpdateUser {
    pub old_pass: String,
    pub new_pass: String,
}

#[derive(Serialize, Deserialize, Object)]
pub struct BaseInfo {
    pub hard: Vec<VoDeviceHard>,
    pub soft: Vec<DeviceSoft>,
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoLogin {
    pub email: String,
    pub password: String,
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoFirm {
    pub id: i32,
    pub hard_version: i32,
    pub version_name: String,
    pub version_format: String,
    pub version_type: i32,
    pub finger_level: i32,
    pub url: String,
    pub desc: String,
    pub update_time: i64,
    pub rely_version_type: Option<i32>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub des_en: String,
    pub des_ko: String,
    pub des_sp: String,
}

impl From<Firm> for VoFirm {
    fn from(f: Firm) -> Self {
        VoFirm {
            id: f.id,
            hard_version: f.hard_version,
            version_name: f.version_name,
            version_format: f.version_format,
            version_type: f.version_type,
            finger_level: f.finger_level,
            url: f.url,
            desc: f.desc,
            update_time: f.update_time.timestamp(),
            rely_version_type: f.rely_version_type,
            min: f.min,
            max: f.max,
            des_en: f.des_en,
            des_ko: f.des_ko,
            des_sp: f.des_sp,
        }
    }
}

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("token expire")]
    TokenError,
    #[error("server error: `{0}`")]
    Internal(String),
    #[error("mail or passwprd error")]
    MailOrPasswordFail,
    #[error("data not found")]
    DataNotFound,
    #[error("password not incorrept")]
    PasswordError,
}

impl From<SqlxError> for CustomError {
    fn from(e: SqlxError) -> Self {
        match e {
            SqlxError::RowNotFound => CustomError::DataNotFound,
            _ => CustomError::Internal(e.to_string()),
        }
    }
}

impl From<CustomError> for PError {
    fn from(e: CustomError) -> Self {
        println!("{:?}", e);
        match e {
            CustomError::TokenError
            | CustomError::MailOrPasswordFail
            | CustomError::PasswordError => {
                PError::from_string(format!("{:?}", e), StatusCode::UNAUTHORIZED)
            }
            CustomError::DataNotFound => {
                PError::from_string(format!("{:?}", e), StatusCode::NOT_FOUND)
            }
            _ => PError::from_string(format!("{:?}", e), StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[derive(Serialize, Deserialize, Enum)]
pub enum VoHardCategory {
    Lock,
    Box,
}

impl From<i32> for VoHardCategory {
    fn from(d: i32) -> Self {
        match d {
            1 => VoHardCategory::Lock,
            2 => VoHardCategory::Box,
            _ => VoHardCategory::Lock,
        }
    }
}

impl From<VoHardCategory> for i32 {
    fn from(d: VoHardCategory) -> Self {
        match d {
            VoHardCategory::Box => 2,
            VoHardCategory::Lock => 1,
        }
    }
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoDeviceHard {
    pub id: i32,
    pub hard_version: String,
    pub name: String,
    pub category: VoHardCategory, // 1锁, 2盒子
    pub has_ble: bool,
    pub has_finger: bool,
    pub has_stm32: bool,
    pub desc: String,
}

impl From<DeviceHard> for VoDeviceHard {
    fn from(d: DeviceHard) -> Self {
        VoDeviceHard {
            id: d.id,
            hard_version: d.hard_version,
            name: d.name,
            category: d.category.into(),
            has_ble: d.has_ble,
            has_finger: d.has_finger,
            has_stm32: d.has_stm32,
            desc: d.desc,
        }
    }
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoAddHard {
    pub hard_version: String,
    pub name: String,
    pub category: VoHardCategory,
    pub has_ble: bool,
    pub has_finger: bool,
    pub has_stm32: bool,
    pub desc: String,
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoUpdateHard {
    pub id: i32,
    pub hard_version: String,
    pub name: String,
    pub category: VoHardCategory,
    pub has_ble: bool,
    pub has_finger: bool,
    pub has_stm32: bool,
    pub desc: String,
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoAddSoft {
    pub name: String,
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoUpdateSoft {
    pub id: i32,
    pub name: String,
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoAddFirm {
    pub hard_version: i32,
    pub version_name: String,
    pub version_format: String,
    pub version_type: i32,
    pub finger_level: i32,
    pub url: String,
    pub desc: String,
    pub update_time: i64,
    pub rely_version_type: Option<i32>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub des_en: String,
    pub des_ko: String,
    pub des_sp: String,
}
impl VoAddFirm {
    pub fn check_data(self) -> VoAddFirm {
        if self.rely_version_type.is_some() || self.min.is_some() {
            self
        } else {
            VoAddFirm{
                hard_version: self.hard_version,
                version_name: self.version_name,
                version_format: self.version_format,
                version_type: self.version_type,
                finger_level: self.finger_level,
                url: self.url,
                desc: self.desc,
                update_time: self.update_time,
                rely_version_type: None,
                min: None,
                max: None,
                des_en: self.des_en,
                des_ko: self.des_ko,
                des_sp: self.des_sp, 
            }
        }
    }
}

#[derive(Object, Serialize, Deserialize)]
pub struct VoUpdateFirm {
    pub id: i32,
    pub hard_version: i32,
    pub version_name: String,
    pub version_format: String,
    pub version_type: i32,
    pub finger_level: i32,
    pub url: String,
    pub desc: String,
    pub update_time: i64,
    pub rely_version_type: Option<i32>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub des_en: String,
    pub des_ko: String,
    pub des_sp: String,
}

impl VoUpdateFirm {
    pub fn check_data(self) -> VoUpdateFirm {
        if self.rely_version_type.is_some() || self.min.is_some() {
            self
        } else {
            VoUpdateFirm{
                id: self.id,
                hard_version: self.hard_version,
                version_name: self.version_name,
                version_format: self.version_format,
                version_type: self.version_type,
                finger_level: self.finger_level,
                url: self.url,
                desc: self.desc,
                update_time: self.update_time,
                rely_version_type: None,
                min: None,
                max: None,
                des_en: self.des_en,
                des_ko: self.des_ko,
                des_sp: self.des_sp, 
            }
        }
    }
}