/*开始实现服务入口 */

use async_trait::async_trait;
use sqlx::PgPool;
use tokio::sync::mpsc;

pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    // 创建一个预定
    async fn reserve(&self, req: abi::ReserveRequest) -> Result<abi::Error>;
}
