//! 安全工具模块
//!
//! 提供密码加密、验证等安全相关功能。

use bcrypt::{hash, verify, DEFAULT_COST};
use thiserror::Error;

/// 安全操作错误
#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("密码加密错误: {0}")]
    HashError(String),

    #[error("密码验证错误: {0}")]
    VerifyError(String),
}

/// 使用 bcrypt 加密密码
///
/// # 参数
///
/// * `password` - 要加密的明文密码
///
/// # 返回
///
/// 成功时返回加密后的密码哈希，失败时返回错误
pub fn hash_password(password: &str) -> Result<String, SecurityError> {
    hash(password, DEFAULT_COST).map_err(|e| SecurityError::HashError(e.to_string()))
}

/// 验证密码是否匹配
///
/// # 参数
///
/// * `password` - 明文密码
/// * `hash` - 存储的密码哈希
///
/// # 返回
///
/// 密码匹配时返回 true，不匹配或出错时返回 false
pub fn verify_password(password: &str, hash: &str) -> Result<bool, SecurityError> {
    verify(password, hash).map_err(|e| SecurityError::VerifyError(e.to_string()))
}

/// 生成随机令牌（可用于会话令牌、重置密码令牌等）
pub fn generate_token() -> String {
    use rand::distributions::Alphanumeric;
    // 需要在 Cargo.toml 中添加 rand = "0.8.5" 依赖
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let token: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();

    token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "test_password123";

        // 测试密码加密
        let hashed = hash_password(password).expect("密码加密失败");
        assert_ne!(password, hashed, "加密后的密码不应与原密码相同");

        // 测试密码验证 - 正确密码
        let result = verify_password(password, &hashed).expect("密码验证失败");
        assert!(result, "正确密码应验证通过");

        // 测试密码验证 - 错误密码
        let result = verify_password("wrong_password", &hashed).expect("密码验证失败");
        assert!(!result, "错误密码不应验证通过");
    }

    #[test]
    fn test_generate_token() {
        let token1 = generate_token();
        let token2 = generate_token();

        // 令牌应该是32字符长
        assert_eq!(token1.len(), 32);

        // 两个令牌应该不同
        assert_ne!(token1, token2);
    }
}
