use std::time::Duration;
use super::Fut;

pub trait Time {
    fn now_as_timestamp_nanos(&self) -> u128;
    fn wait(&self, dur: Duration) -> Fut;
}
