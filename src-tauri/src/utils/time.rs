//! 时间工具模块
//!
//! 提供时间相关的工具函数。

use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳（秒）
pub fn now() -> i32 {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_secs() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let timestamp = now();
        assert!(timestamp > 0);

        // 确保时间戳是当前时间
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;

        // 允许有1秒的误差
        assert!((current_time - timestamp).abs() < 1000);
    }
}
