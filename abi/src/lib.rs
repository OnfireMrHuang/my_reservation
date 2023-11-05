mod error;
mod pb;
mod types;

use std::fmt::Result;

pub use error::*;
pub use pb::*;
pub use types::*;

pub type ReservationId = i64;
pub type UserId = i64;
pub type ResourceId = String;

// 定义校验特征，用于校验非法时抛出错误
pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
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
