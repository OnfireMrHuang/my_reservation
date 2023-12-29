/*开始实现服务入口 */
mod manager;
mod sqlx_tester;

use abi::FilterPager;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx_tester::*;
use tokio::sync::mpsc;

// 定义业务接口
#[async_trait]
pub trait Rsvp {
    // 创建一个预定
    async fn reserve(&self, req: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
    // 修改预定状态(如果当前预定是pending状态，可以修改为确认或者取消状态)
    async fn change_status(&self, id: abi::ReservationId) -> Result<abi::Reservation, abi::Error>;
    // 修改预定备注
    async fn update_note(
        &self,
        id: abi::ReservationId,
        note: String,
    ) -> Result<abi::Reservation, abi::Error>;
    // 删除预定
    async fn delete(&self, id: abi::ReservationId) -> Result<abi::Reservation, abi::Error>;
    // 获取指定预定
    async fn get(&self, id: abi::ReservationId) -> Result<abi::Reservation, abi::Error>;
    // 条件查询预定
    async fn query(
        &self,
        req: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, abi::Error>>;
    async fn filter(
        &self,
        query: abi::ReservationFilter,
    ) -> Result<(FilterPager, Vec<abi::Reservation>), abi::Error>;
}

// 定义一个预定管理实现类
#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}
