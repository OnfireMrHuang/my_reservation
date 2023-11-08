use abi::Validate;
use async_trait::async_trait;

use crate::{ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    // 实现预留预定资源接口
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        // 第一步，校验预定
        rsvp.validate()?;

        let status = abi::ReservationStatus::try_from(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let timespan = rsvp.get_timespan();

        // 生成insert into语句并将预定信息插入到数据库中
        // let id = sqlx::query(
        //     "INSERT INTO rsvp."
        // )

        Ok(rsvp)
    }
}
