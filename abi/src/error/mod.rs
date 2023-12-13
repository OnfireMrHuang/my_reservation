use sqlx::{error, postgres::PgDatabaseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("数据库错误: {0}")]
    DbError(sqlx::Error),

    #[error("配置文件读取错误")]
    ConfigReadError,

    #[error("配置文件解析错误")]
    ConfigParseError,

    #[error("非法的开始或结束预定时间")]
    InvalidTime,

    #[error("未找到预定记录")]
    NotFound,

    #[error("非法的用户ID: {0}")]
    InvalidUserId(String),

    #[error("非法的资源ID: {0}")]
    InvalidResourceId(String),

    #[error("未知错误")]
    Unknown,
}

// 为abi::Error实现部分对比接口
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DbError(_), Self::DbError(_)) => true,
            (Self::Unknown, Self::Unknown) => true,
            _ => false,
        }
    }
}

// 为abi::Error实现从sqlx::Error转换接口
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let err: &PgDatabaseError = e.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    _ => Error::DbError(sqlx::Error::Database(e)),
                }
            }
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(e),
        }
    }
}
