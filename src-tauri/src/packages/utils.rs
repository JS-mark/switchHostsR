use std::time::{SystemTime, UNIX_EPOCH};
pub fn now_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_millis() as i64; // 获取毫秒
}
