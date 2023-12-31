mod config;
mod error;
mod pb;
mod types;
mod utils;

pub use config::*;
pub use error::*;
pub use pb::*;
pub use types::*;
pub use utils::*;

pub type ReservationId = i64;
pub type UserId = i64;
pub type ResourceId = String;

// 定义校验特征，用于校验非法时抛出错误
pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}

// 定义规范特征，用于规范化(绑定校验特征)
pub trait Normalizer: Validate {
    fn do_normalize(&mut self);
    fn normalize(&mut self) -> Result<(), Error> {
        self.validate()?;
        self.do_normalize();
        Ok(())
    }
}

// 定义Tosql特征
pub trait ToSql {
    fn to_sql(&self) -> String;
}

// 定义预定状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "reservation_status", rename_all = "lowercase")]
pub enum RsvpStatus {
    Unknown,
    Pending,
    Confirmed,
    Blocked,
}

// 给预定ID实现校验方法
impl Validate for ReservationId {
    fn validate(&self) -> Result<(), Error> {
        if *self <= 0 {
            return Err(Error::InvalidReservationId(*self));
        }
        Ok(())
    }
}
