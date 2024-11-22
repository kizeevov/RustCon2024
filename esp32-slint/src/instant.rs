use button_driver::InstantProvider;
use core::{ops::Sub, time::Duration};
use esp_hal::time::now;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Instant {
    counter: Duration,
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Self::Output {
        self.counter - rhs.counter
    }
}

impl InstantProvider<Duration> for Instant {
    fn now() -> Self {
        Instant {
            counter: Duration::from_micros(now().ticks()),
        }
    }
}
