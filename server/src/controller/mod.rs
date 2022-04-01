use poem::{web::Data, Error, Result, Request };
use poem_openapi::{param::Path, payload::Json, OpenApi,auth::ApiKey,SecurityScheme};

use crate::{
    domain::{
        dto::{DeviceSoft},
        vo::{Token, VoLogin, VoUpdateUser, VoUser, VoFirm, VoAddHard, VoUpdateHard, VoAddSoft, VoUpdateSoft, VoAddFirm, VoUpdateFirm, VoDeviceHard, BaseInfo, ReturnData},
    },
    service::{SYS_FIRM_SERVICE, SYS_HARD_SERVICE, SYS_SOFT_SERVICE, SYS_USER_SERVICE},
    utils::jwt::{gen_user_token, validate_token},
    DbPool,
};

#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "token",
    in = "header",
    checker = "api_checker"
)]
struct TokenAuthorization(pub VoUser);

async fn api_checker(_: &Request, api_key: ApiKey) -> Option<VoUser> {
    validate_token(api_key.key.as_str())
}

pub struct Api;

#[OpenApi]
impl Api {

    /// 登陆
    #[oai(path = "/login", method = "post")]
    async fn login(&self, pool: Data<&DbPool>, data: Json<VoLogin>) -> Result<Json<Token>> {
        let user = SYS_USER_SERVICE.login(&pool, data.0).await?;
        Ok(Json(gen_user_token(user)))
    }

    /// 更新用户密码
    #[oai(path = "/user/updatePass", method = "post")]
    async fn update_password(
        &self,
        pool: Data<&DbPool>,
        user: TokenAuthorization,
        data: Json<VoUpdateUser>,
    ) -> Result<()> {
        SYS_USER_SERVICE
            .change_pass(&pool, &user.0, data.0)
            .await
            .map_err(Error::from)
    }

    /// 获取所有硬件类型
    #[oai(path = "/devices", method = "get")]
    async fn devices(&self, pool: Data<&DbPool>, ) -> Result<Json<Vec<VoDeviceHard>>> {
        let devices = SYS_HARD_SERVICE.devices(&pool).await?;
        Ok(Json(devices))
    }

    /// 添加硬件类型
    #[oai(path = "/devices", method = "post")]
    async fn add_devices(&self, pool: Data<&DbPool>, data: Json<VoAddHard>) -> Result<Json<ReturnData>> {
        SYS_HARD_SERVICE.add_device(&pool, data.0).await?;
        Ok(Json(ReturnData::default()))
    }

    /// 更新硬件类型
    #[oai(path = "/devices", method = "put")]
    async fn update_devices(&self, pool: Data<&DbPool>, data: Json<VoUpdateHard>) -> Result<Json<ReturnData>> {
        SYS_HARD_SERVICE.update_device(&pool, data.0).await?;
        Ok(Json(ReturnData::default()))
    }

    /// 获取所有软件类型
    #[oai(path = "/softTypes", method = "get")]
    async fn soft_types(&self, pool: Data<&DbPool>) -> Result<Json<Vec<DeviceSoft>>> {
        let types = SYS_SOFT_SERVICE.soft_versions(&pool).await?;
        Ok(Json(types))
    }

    /// 添加软件类型
    #[oai(path = "/softTypes", method = "post")]
    async fn add_soft_types(&self, pool: Data<&DbPool>, data: Json<VoAddSoft>) -> Result<Json<ReturnData>> {
        SYS_SOFT_SERVICE.add_soft_version(&pool, data.0).await?;
        Ok(Json(ReturnData::default()))
    }

    /// 更新软件类型
    #[oai(path = "/softTypes", method = "put")]
    async fn update_soft_types(&self, pool: Data<&DbPool>, data: Json<VoUpdateSoft>) -> Result<Json<ReturnData>> {
        SYS_SOFT_SERVICE.update_soft_version(&pool, data.0).await?;
        Ok(Json(ReturnData::default()))
    }

    /// 获取所有固件
    #[oai(path = "/firms", method = "get")]
    async fn firms(&self, pool: Data<&DbPool>) -> Result<Json<Vec<VoFirm>>> {
        let firms = SYS_FIRM_SERVICE.firms(&pool).await?;
        Ok(Json(firms))
    }

    /// 添加固件
    #[oai(path = "/firms", method = "post")]
    async fn add_firms(&self, pool: Data<&DbPool>, data: Json<VoAddFirm>) -> Result<Json<ReturnData>> {
        SYS_FIRM_SERVICE.add_firms(&pool, data.0).await?;
        Ok(Json(ReturnData::default()))
    }

    /// 更新固件
    #[oai(path = "/firms", method = "put")]
    async fn update_firms(&self, pool: Data<&DbPool>, data: Json<VoUpdateFirm>) -> Result<Json<ReturnData>> {
        SYS_FIRM_SERVICE.update_firms(&pool, data.0).await?;
        Ok(Json(ReturnData::default()))
    }

    /// 删除固件
    #[oai(path = "/firms/:device", method = "delete")]
    async fn delete_device(
        &self,
        pool: Data<&DbPool>,
        device: Path<i32>,
    ) -> Result<Json<ReturnData>> {
        SYS_FIRM_SERVICE.delete_firm(&pool, device.0).await?;
        Ok(Json(ReturnData::default()))
    }

    /// 根据硬件id查询固件
    #[oai(path = "/firms/:device", method = "get")]
    async fn firms_by_device(
        &self,
        pool: Data<&DbPool>,
        device: Path<i32>,
    ) -> Result<Json<Vec<VoFirm>>> {
        let firms = SYS_FIRM_SERVICE.firms_by_device(&pool, device.0).await?;
        Ok(Json(firms))
    }

    /// 获取基础数据
    #[oai(path = "/baseInfo", method = "get")]
    async fn base_info(&self, pool: Data<&DbPool>) -> Result<Json<BaseInfo>>{
        let hard = SYS_HARD_SERVICE.devices(&pool).await?;
        let soft = SYS_SOFT_SERVICE.soft_versions(&pool).await?;
        Ok(Json(BaseInfo{hard, soft}))
    }
}
