use crate::{ReservationStatus, RsvpStatus};
use std::fmt;

impl From<RsvpStatus> for ReservationStatus {
    fn from(status: RsvpStatus) -> Self {
        match status {
            RsvpStatus::Confirmed => ReservationStatus::Confirmed,
            RsvpStatus::Pending => ReservationStatus::Pending,
            RsvpStatus::Blocked => ReservationStatus::Blocked,
            RsvpStatus::Unknown => ReservationStatus::Unknown,
        }
    }
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReservationStatus::Pending => write!(f, "Pending"),
            ReservationStatus::Confirmed => write!(f, "Confirmed"),
            ReservationStatus::Blocked => write!(f, "Blocked"),
            ReservationStatus::Unknown => write!(f, "Unknown"),
        }
    }
}
