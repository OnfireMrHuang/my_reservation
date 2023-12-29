use crate::{Error, ReservationQuery, ReservationStatus};

impl ReservationQuery {
    pub fn get_status(&self) -> ReservationStatus {
        ReservationStatus::from(self.status)
    }
}
