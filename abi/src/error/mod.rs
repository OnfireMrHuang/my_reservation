#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("数据库错误: {0}")]
    DbError(sqlx::Error),
}
