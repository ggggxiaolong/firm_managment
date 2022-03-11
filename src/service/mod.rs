use chrono::Utc;
use poem::web::Data;

use crate::{
    domain::{
        dto::{DeviceHard, DeviceSoft, Firm, User},
        vo::{CustomError, VoFirm, VoLogin, VoUpdateUser, VoUser},
    },
    utils::sql_helper::SqlHelper,
    DbPool,
};

lazy_static! {
    pub static ref SYS_USER_SERVICE: UserService = UserService {};
    pub static ref SYS_HARD_SERVICE: DeviceHardService = DeviceHardService {};
    pub static ref SYS_SOFT_SERVICE: DeviceSoftService = DeviceSoftService {};
    pub static ref SYS_FIRM_SERVICE: FirmService = FirmService {};
}

pub struct UserService;

const TABLE_USER: &str = "user";
const USER_COLUMNS: &str = "id, name, mail, password";

impl UserService {
    pub async fn login(&self, pool: Data<&DbPool>, data: VoLogin) -> Result<VoUser, CustomError> {
        let mut helper = SqlHelper::query(TABLE_USER, USER_COLUMNS);
        let sql = helper.and_where_eq("mail", &data.email).sql();
        let user: User = sqlx::query_as(&sql)
            .fetch_one(pool.0)
            .await
            .map_err(|_| CustomError::MailOrPasswordFail)?;
        match bcrypt::verify(&data.password, &user.password) {
            Ok(true) => Ok(user.into()),
            _ => Err(CustomError::MailOrPasswordFail),
        }
    }
    pub async fn change_pass(
        &self,
        pool: Data<&DbPool>,
        user: &VoUser,
        data: VoUpdateUser,
    ) -> Result<(), CustomError> {
        let mut helper = SqlHelper::query(TABLE_USER, USER_COLUMNS);
        let sql = helper.and_where_eq("mail", &user.mail).sql();
        let user: User = sqlx::query_as(&sql)
            .fetch_one(pool.0)
            .await
            .map_err(|_| CustomError::MailOrPasswordFail)?;
        match bcrypt::verify(&data.old_pass, &user.password) {
            Ok(true) => {
                sqlx::query("UPDATE user SET password = ?, update_time = ? WHERE id = ?")
                    .bind(data.new_pass)
                    .bind(Utc::now().naive_utc())
                    .bind(user.id)
                    .execute(pool.0)
                    .await?;
                Ok(())
            }
            _ => Err(CustomError::PasswordError),
        }
    }

    pub async fn check_token(
        &self,
        pool: Data<&DbPool>,
        data: VoUser,
    ) -> Result<User, CustomError> {
        let mut helper = SqlHelper::query(TABLE_USER, USER_COLUMNS);
        let sql = helper.and_where_eq("mail", &data.mail).sql();
        let user: User = sqlx::query_as(&sql)
            .fetch_one(pool.0)
            .await
            .map_err(|_| CustomError::TokenError)?;
        if data.ticker == user.update_time.timestamp() {
            Ok(user)
        } else {
            Err(CustomError::TokenError)
        }
    }
}

pub struct DeviceHardService;

const TABLE_HARD: &str = "device_type";
const HARD_COLUMNS: &str = "id, hard_version, name, category, has_ble, has_finger, has_stm32, desc";
impl DeviceHardService {
    pub async fn devices(&self, pool: Data<&DbPool>) -> Result<Vec<DeviceHard>, CustomError> {
        let sql = SqlHelper::query(TABLE_HARD, HARD_COLUMNS).sql();
        sqlx::query_as(&sql)
            .fetch_all(pool.0)
            .await
            .map_err(CustomError::from)
    }
}

pub struct DeviceSoftService;

const TABLE_SOFT: &str = "version_type";
const SOFT_COLUMNS: &str = "id, name";
impl DeviceSoftService {
    pub async fn soft_versions(&self, pool: Data<&DbPool>) -> Result<Vec<DeviceSoft>, CustomError> {
        let sql = SqlHelper::query(TABLE_SOFT, SOFT_COLUMNS).sql();
        sqlx::query_as(&sql)
            .fetch_all(pool.0)
            .await
            .map_err(CustomError::from)
    }
}

pub struct FirmService;

const TABLE_FIRM: &str = "firm";
const FIRM_COLUMNS: &str = "id, hard_version, version_name, version_format, version_type, finger_level, url, desc, update_time, rely_version_type, min, max, des_en, des_ko, des_sp";
impl FirmService {
    pub async fn firms(&self, pool: Data<&DbPool>) -> Result<Vec<VoFirm>, CustomError> {
        let sql = SqlHelper::query(TABLE_FIRM, FIRM_COLUMNS)
            .order_desc("update_time")
            .sql();
        let data: Vec<Firm> = sqlx::query_as(&sql).fetch_all(pool.0).await?;
        let mut firms = Vec::with_capacity(data.len());
        for f in data {
            firms.push(f.into())
        }
        Ok(firms)
    }
    pub async fn firms_by_device(
        &self,
        pool: Data<&DbPool>,
        hard_version: i32,
    ) -> Result<Vec<VoFirm>, CustomError> {
        let sql = SqlHelper::query(TABLE_FIRM, FIRM_COLUMNS)
            .and_where_eq("hard_version", &hard_version.to_string())
            .order_desc("update_time")
            .sql();
        let data: Vec<Firm> = sqlx::query_as(&sql).fetch_all(pool.0).await?;
        let mut firms = Vec::with_capacity(data.len());
        for f in data {
            firms.push(f.into())
        }
        Ok(firms)
    }
}

#[cfg(test)]
mod tests {

    // use bcrypt::{DEFAULT_COST, hash, verify};

    #[test]
    fn test_bcrypt() {
        let ret = bcrypt::verify(
            "UEoC?5K3*xays2/B",
            "$2b$12$dQu7D/KHYLC0GiZWLn7zV.7ceiccLA74aCd.u676aaRCUENIcLvCq",
        )
        .unwrap();
        assert!(ret);
    }
}
