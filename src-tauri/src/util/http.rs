use std::time::Duration;

// Per-IP connect timeout so reqwest's multi-IP fallback (hyper) engages
// instead of stalling on one dead anycast address. See reqwest#1939, reqwest#1940.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

pub fn blocking_client() -> reqwest::blocking::Client {
  reqwest::blocking::Client::builder()
    .connect_timeout(CONNECT_TIMEOUT)
    .build()
    .expect("failed to build reqwest blocking client")
}

pub fn async_client() -> reqwest::Client {
  reqwest::Client::builder()
    .connect_timeout(CONNECT_TIMEOUT)
    .build()
    .expect("failed to build reqwest client")
}
