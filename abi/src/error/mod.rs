use sqlx::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("数据库错误: {0}")]
    DbError(sqlx::Error),

    #[error("非法的开始或结束预定时间")]
    InvalidTime,

    #[error("非法的用户ID: {0}")]
    InvalidUserId(String),

    #[error("非法的资源ID: {0}")]
    InvalidResourceId(String),

    #[error("未知错误")]
    Unknown,
}
