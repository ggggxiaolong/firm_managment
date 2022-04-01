use chrono::{DateTime, NaiveDateTime, Utc};
use poem::web::Data;

use crate::{
    domain::{
        dto::{DeviceHard, DeviceSoft, Firm, User},
        vo::{
            CustomError, VoAddFirm, VoAddHard, VoAddSoft, VoDeviceHard, VoFirm, VoLogin,
            VoUpdateFirm, VoUpdateHard, VoUpdateSoft, VoUpdateUser, VoUser,
        },
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
    pub async fn login(&self, pool: &Data<&DbPool>, data: VoLogin) -> Result<VoUser, CustomError> {
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
        pool: &Data<&DbPool>,
        user: &VoUser,
        data: VoUpdateUser,
    ) -> Result<(), CustomError> {
        let mut helper = SqlHelper::query(TABLE_USER, USER_COLUMNS);
        let sql = helper.and_where_eq("id", "?").sql();
        let user: User = sqlx::query_as(&sql)
            .bind(user.id)
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
        pool: &Data<&DbPool>,
        data: VoUser,
    ) -> Result<User, CustomError> {
        let mut helper = SqlHelper::query(TABLE_USER, USER_COLUMNS);
        let sql = helper.and_where_eq("id", "?").sql();
        let user: User = sqlx::query_as(&sql)
            .bind(data.id)
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
const HARD_ADD_COLUMNS: &str =
    " hard_version, name, category, has_ble, has_finger, has_stm32, desc";
impl DeviceHardService {
    pub async fn devices(&self, pool: &Data<&DbPool>) -> Result<Vec<VoDeviceHard>, CustomError> {
        let sql = SqlHelper::query(TABLE_HARD, HARD_COLUMNS).sql();
        let devices: Vec<DeviceHard> = sqlx::query_as(&sql).fetch_all(pool.0).await?;
        let mut data = Vec::<VoDeviceHard>::with_capacity(devices.len());
        for d in devices {
            data.push(d.into());
        }
        Ok(data)
    }

    pub async fn add_device(
        &self,
        pool: &Data<&DbPool>,
        data: VoAddHard,
    ) -> Result<(), CustomError> {
        let sql = SqlHelper::insert(TABLE_HARD, HARD_ADD_COLUMNS).sql();
        let rows_affected = sqlx::query(&sql)
            .bind(data.hard_version)
            .bind(data.name)
            .bind(i32::from(data.category))
            .bind(data.has_ble)
            .bind(data.has_finger)
            .bind(data.has_stm32)
            .bind(data.desc)
            .execute(pool.0)
            .await?
            .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(CustomError::DataNotFound)
        }
    }

    pub async fn update_device(
        &self,
        pool: &Data<&DbPool>,
        data: VoUpdateHard,
    ) -> Result<(), CustomError> {
        let sql = SqlHelper::update(TABLE_HARD, HARD_ADD_COLUMNS)
            .and_where_eq(" id ", "?")
            .sql();
        let rows_affected = sqlx::query(&sql)
            .bind(data.hard_version)
            .bind(data.name)
            .bind(i32::from(data.category))
            .bind(data.has_ble)
            .bind(data.has_finger)
            .bind(data.has_stm32)
            .bind(data.desc)
            .bind(data.id)
            .execute(pool.0)
            .await?
            .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(CustomError::DataNotFound)
        }
    }
}

pub struct DeviceSoftService;

const TABLE_SOFT: &str = "version_type";
const SOFT_COLUMNS: &str = "id, name";
const SOFT_ADD_COLUMNS: &str = "name";
impl DeviceSoftService {
    pub async fn soft_versions(
        &self,
        pool: &Data<&DbPool>,
    ) -> Result<Vec<DeviceSoft>, CustomError> {
        let sql = SqlHelper::query(TABLE_SOFT, SOFT_COLUMNS).sql();
        sqlx::query_as(&sql)
            .fetch_all(pool.0)
            .await
            .map_err(CustomError::from)
    }

    pub async fn add_soft_version(
        &self,
        pool: &Data<&DbPool>,
        data: VoAddSoft,
    ) -> Result<(), CustomError> {
        let sql = SqlHelper::insert(TABLE_SOFT, SOFT_ADD_COLUMNS).sql();
        sqlx::query(&sql).bind(data.name).execute(pool.0).await?;
        Ok(())
    }

    pub async fn update_soft_version(
        &self,
        pool: &Data<&DbPool>,
        data: VoUpdateSoft,
    ) -> Result<(), CustomError> {
        let sql = SqlHelper::update(TABLE_SOFT, SOFT_ADD_COLUMNS)
            .and_where_eq(" id ", "?")
            .sql();
        let rows_affected = sqlx::query(&sql)
            .bind(data.name)
            .bind(data.id)
            .execute(pool.0)
            .await?
            .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(CustomError::DataNotFound)
        }
    }
}

pub struct FirmService;

const TABLE_FIRM: &str = "firm";
const FIRM_COLUMNS: &str = "id, hard_version, version_name, version_format, version_type, finger_level, url, desc, update_time, rely_version_type, min, max, des_en, des_ko, des_sp";
const FIRM_ADD_COLUMNS: &str = " hard_version, version_name, version_format, version_type, finger_level, url, desc, update_time, rely_version_type, min, max, des_en, des_ko, des_sp";
impl FirmService {
    pub async fn firms(&self, pool: &Data<&DbPool>) -> Result<Vec<VoFirm>, CustomError> {
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
        pool: &Data<&DbPool>,
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

    pub async fn delete_firm(&self, pool: &Data<&DbPool>, id: i32) -> Result<(), CustomError> {
        let sql = SqlHelper::delete(TABLE_FIRM).and_where_eq("id", "?").sql();
        let rows_affected = sqlx::query(&sql)
            .bind(id)
            .execute(pool.0)
            .await?
            .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(CustomError::DataNotFound)
        }
    }

    pub async fn add_firms(
        &self,
        pool: &Data<&DbPool>,
        data: VoAddFirm,
    ) -> Result<(), CustomError> {
        let sql = SqlHelper::insert(TABLE_FIRM, FIRM_ADD_COLUMNS).sql();
        let data = data.check_data();
        sqlx::query(&sql)
            .bind(data.hard_version)
            .bind(data.version_name)
            .bind(data.version_format)
            .bind(data.version_type)
            .bind(data.finger_level)
            .bind(data.url)
            .bind(data.desc)
            .bind(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(data.update_time, 0),
                Utc,
            ))
            .bind(data.rely_version_type)
            .bind(data.min)
            .bind(data.max)
            .bind(data.des_en)
            .bind(data.des_ko)
            .bind(data.des_sp)
            .execute(pool.0)
            .await?;
        Ok(())
    }

    pub async fn update_firms(
        &self,
        pool: &Data<&DbPool>,
        data: VoUpdateFirm,
    ) -> Result<(), CustomError> {
        let sql = SqlHelper::update(TABLE_FIRM, FIRM_ADD_COLUMNS)
            .and_where_eq(" id ", "?")
            .sql();
        let data = data.check_data();
        let rows_affected = sqlx::query(&sql)
            .bind(data.hard_version)
            .bind(data.version_name)
            .bind(data.version_format)
            .bind(data.version_type)
            .bind(data.finger_level)
            .bind(data.url)
            .bind(data.desc)
            .bind(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(data.update_time, 0),
                Utc,
            ))
            .bind(data.rely_version_type)
            .bind(data.min)
            .bind(data.max)
            .bind(data.des_en)
            .bind(data.des_ko)
            .bind(data.des_sp)
            .bind(data.id)
            .execute(pool.0)
            .await?
            .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(CustomError::DataNotFound)
        }
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
