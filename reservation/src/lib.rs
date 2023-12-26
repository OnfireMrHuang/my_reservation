/*开始实现服务入口 */
mod manager;
mod sqlx_tester;

use async_trait::async_trait;
use sqlx::PgPool;
use sqlx_tester::*;

// 定义业务接口
#[async_trait]
pub trait Rsvp {
    // 创建一个预定
    async fn reserve(&self, req: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
}

// 定义一个预定管理实现类
#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}
