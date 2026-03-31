//! 令牌管理模块
//!
//! 提供 JWT 令牌的生成、验证和管理功能
//! 使用 jsonwebtoken 库实现标准 HMAC-SHA256 签名

use super::{AuthError, Role};
use anyhow::Result;
use base64::Engine;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JWT 声明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,             // 用户ID
    pub username: String,        // 用户名
    pub role: Role,              // 用户角色
    pub token_type: String,      // 令牌类型: "access" 或 "refresh"
    pub exp: i64,                // 过期时间
    pub iat: i64,                // 签发时间
    pub jti: String,             // JWT ID
}

/// 令牌类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Access,  // 访问令牌
    Refresh, // 刷新令牌
}

/// 令牌信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token: String,
    pub token_type: TokenType,
    pub expires_at: i64,
    pub user_id: i32,
}

/// 令牌对
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: TokenInfo,
    pub refresh_token: TokenInfo,
}

/// 令牌管理器
pub struct TokenManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_duration: Duration,
    refresh_token_duration: Duration,
    revoked_tokens: std::sync::RwLock<HashMap<String, i64>>, // JTI -> 撤销时间
}

impl std::fmt::Debug for TokenManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenManager")
            .field("access_token_duration", &self.access_token_duration)
            .field("refresh_token_duration", &self.refresh_token_duration)
            .finish()
    }
}

impl TokenManager {
    /// 创建新的令牌管理器
    pub fn new(secret_key: String, access_token_hours: i64, refresh_token_days: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret_key.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret_key.as_bytes()),
            access_token_duration: Duration::hours(access_token_hours),
            refresh_token_duration: Duration::days(refresh_token_days),
            revoked_tokens: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// 生成令牌对
    pub fn generate_token_pair(
        &self,
        user_id: i32,
        username: String,
        role: Role,
    ) -> Result<TokenPair> {
        let access_token =
            self.generate_token(user_id, username.clone(), role.clone(), TokenType::Access)?;

        let refresh_token = self.generate_token(user_id, username, role, TokenType::Refresh)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    /// 生成单个令牌
    fn generate_token(
        &self,
        user_id: i32,
        username: String,
        role: Role,
        token_type: TokenType,
    ) -> Result<TokenInfo> {
        let now = Utc::now();
        let duration = match token_type {
            TokenType::Access => self.access_token_duration,
            TokenType::Refresh => self.refresh_token_duration,
        };

        let exp = (now + duration).timestamp();
        let jti = uuid::Uuid::new_v4().to_string();

        let claims = Claims {
            sub: user_id.to_string(),
            username,
            role,
            token_type: match token_type {
                TokenType::Access => "access".to_string(),
                TokenType::Refresh => "refresh".to_string(),
            },
            exp,
            iat: now.timestamp(),
            jti: jti.clone(),
        };

        // 使用 jsonwebtoken 库进行标准 HMAC-SHA256 签名
        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::Other(format!("生成令牌失败: {}", e)))?;

        Ok(TokenInfo {
            token,
            token_type,
            expires_at: exp,
            user_id,
        })
    }

    /// 验证令牌
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        // 使用 jsonwebtoken 验证（自动校验 exp）
        let mut validation = Validation::default();
        // 不强制要求特定的 required_spec_claims（我们自己检查）
        validation.validate_exp = true;

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    anyhow::Error::from(AuthError::TokenExpired)
                }
                _ => anyhow::Error::from(AuthError::InvalidToken),
            })?;

        let claims = token_data.claims;

        // 检查是否被撤销
        if self.is_token_revoked(&claims.jti)? {
            return Err(AuthError::InvalidToken.into());
        }

        Ok(claims)
    }

    /// 撤销令牌
    pub fn revoke_token(&self, jti: &str) -> Result<()> {
        let mut revoked_tokens = self
            .revoked_tokens
            .write()
            .map_err(|e| AuthError::Other(format!("获取撤销令牌锁失败: {}", e)))?;

        let now = Utc::now().timestamp();
        revoked_tokens.insert(jti.to_string(), now);

        Ok(())
    }

    /// 检查令牌是否被撤销
    fn is_token_revoked(&self, jti: &str) -> Result<bool> {
        let revoked_tokens = self
            .revoked_tokens
            .read()
            .map_err(|e| AuthError::Other(format!("获取撤销令牌锁失败: {}", e)))?;

        Ok(revoked_tokens.contains_key(jti))
    }

    /// 清理过期的撤销令牌
    pub fn cleanup_revoked_tokens(&self) -> Result<usize> {
        let mut revoked_tokens = self
            .revoked_tokens
            .write()
            .map_err(|e| AuthError::Other(format!("获取撤销令牌锁失败: {}", e)))?;

        let now = Utc::now().timestamp();
        let initial_count = revoked_tokens.len();

        // 移除超过刷新令牌有效期的撤销记录
        let cleanup_threshold = now - self.refresh_token_duration.num_seconds();
        revoked_tokens.retain(|_, &mut revoked_at| revoked_at > cleanup_threshold);

        let final_count = revoked_tokens.len();
        Ok(initial_count - final_count)
    }

    /// 刷新访问令牌
    ///
    /// 校验传入的令牌必须是 refresh 类型，防止使用 access token 刷新
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenInfo> {
        let claims = self.verify_token(refresh_token)?;

        // 校验令牌类型必须是 refresh，防止 access token 混用
        if claims.token_type != "refresh" {
            return Err(AuthError::InvalidToken.into());
        }

        let user_id: i32 = claims.sub.parse().map_err(|_| AuthError::InvalidToken)?;

        self.generate_token(user_id, claims.username, claims.role, TokenType::Access)
    }
}

/// 全局令牌管理器实例
static TOKEN_MANAGER: std::sync::OnceLock<TokenManager> = std::sync::OnceLock::new();

/// 生成随机密钥字符串（32 字节，base64 编码）
///
/// 桌面应用场景下，每次启动生成新的随机密钥即可。
/// 重启后旧 token 自动失效，用户需重新登录，这是可接受的行为。
fn generate_random_secret() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen::<u8>()).collect();
    base64::engine::general_purpose::STANDARD.encode(&random_bytes)
}

/// 获取全局令牌管理器
///
/// 优先从环境变量 `JWT_SECRET` 读取密钥；
/// 若未设置，自动生成随机密钥（每次重启后 token 失效，桌面应用可接受）。
pub fn get_token_manager() -> &'static TokenManager {
    TOKEN_MANAGER.get_or_init(|| {
        let secret_key = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                let random_key = generate_random_secret();
                log::info!("未设置 JWT_SECRET 环境变量，已自动生成随机密钥");
                random_key
            });

        TokenManager::new(secret_key, 2, 7) // 访问令牌2小时，刷新令牌7天
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let token_pair = manager
            .generate_token_pair(1, "test_user".to_string(), Role::User)
            .unwrap();

        assert_eq!(token_pair.access_token.user_id, 1);
        assert_eq!(token_pair.refresh_token.user_id, 1);
        assert!(matches!(
            token_pair.access_token.token_type,
            TokenType::Access
        ));
        assert!(matches!(
            token_pair.refresh_token.token_type,
            TokenType::Refresh
        ));
    }

    #[test]
    fn test_token_verification() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let token_info = manager
            .generate_token(1, "test_user".to_string(), Role::User, TokenType::Access)
            .unwrap();

        let claims = manager.verify_token(&token_info.token).unwrap();
        assert_eq!(claims.sub, "1");
        assert_eq!(claims.username, "test_user");
        assert_eq!(claims.role, Role::User);
    }

    #[test]
    fn test_token_revocation() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let token_info = manager
            .generate_token(1, "test_user".to_string(), Role::User, TokenType::Access)
            .unwrap();

        // 验证令牌有效
        assert!(manager.verify_token(&token_info.token).is_ok());

        // 撤销令牌
        let claims = manager.verify_token(&token_info.token).unwrap();
        manager.revoke_token(&claims.jti).unwrap();

        // 验证令牌已被撤销
        assert!(manager.verify_token(&token_info.token).is_err());
    }

    #[test]
    fn test_invalid_token_rejected() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        // 用不同密钥生成的令牌应该被拒绝
        let other_manager = TokenManager::new("other_secret".to_string(), 1, 7);
        let token_info = other_manager
            .generate_token(1, "test_user".to_string(), Role::User, TokenType::Access)
            .unwrap();

        assert!(manager.verify_token(&token_info.token).is_err());
    }

    #[test]
    fn test_refresh_token_type_validation() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        // 生成 access token
        let access_token = manager
            .generate_token(1, "test_user".to_string(), Role::User, TokenType::Access)
            .unwrap();

        // 尝试用 access token 刷新应该失败
        assert!(manager.refresh_access_token(&access_token.token).is_err());

        // 生成 refresh token
        let refresh_token = manager
            .generate_token(1, "test_user".to_string(), Role::User, TokenType::Refresh)
            .unwrap();

        // 用 refresh token 刷新应该成功
        assert!(manager.refresh_access_token(&refresh_token.token).is_ok());
    }

    #[test]
    fn test_token_type_in_claims() {
        let manager = TokenManager::new("test_secret".to_string(), 1, 7);

        let access_token = manager
            .generate_token(1, "test_user".to_string(), Role::User, TokenType::Access)
            .unwrap();
        let claims = manager.verify_token(&access_token.token).unwrap();
        assert_eq!(claims.token_type, "access");

        let refresh_token = manager
            .generate_token(1, "test_user".to_string(), Role::User, TokenType::Refresh)
            .unwrap();
        let claims = manager.verify_token(&refresh_token.token).unwrap();
        assert_eq!(claims.token_type, "refresh");
    }
}
