#[cfg(target_arch = "wasm32")]
pub fn now_nanos() -> u128 {
    // Date::now() is milliseconds since Unix epoch.
    (js_sys::Date::now() * 1_000_000.0) as u128
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now_nanos() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
