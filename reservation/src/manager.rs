use crate::{ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    // 实现预留预定资源接口
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        // 第一步，校验预定
        rsvp.validate()?;
    }
}
