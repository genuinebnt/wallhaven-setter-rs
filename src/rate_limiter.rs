#[derive(Debug)]
pub struct RateLimiter {
    pub requests_per_minute: u32,
    pub counter: u32,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> RateLimiter {
        RateLimiter {
            requests_per_minute,
            counter: requests_per_minute,
        }
    }
}
