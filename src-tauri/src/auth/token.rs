//! 令牌管理模块
//!
//! 提供 JWT 令牌的生成、验证和管理功能

use super::{AuthError, Role};
use anyhow::Result;
use base64::prelude::*;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JWT 声明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // 用户ID
    pub username: String, // 用户名
    pub role: Role,       // 用户角色
    pub exp: i64,         // 过期时间
    pub iat: i64,         // 签发时间
    pub jti: String,      // JWT ID
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
#[derive(Debug)]
pub struct TokenManager {
    secret_key: String,
    access_token_duration: Duration,
    refresh_token_duration: Duration,
    revoked_tokens: std::sync::RwLock<HashMap<String, i64>>, // JTI -> 撤销时间
}

impl TokenManager {
    /// 创建新的令牌管理器
    pub fn new(secret_key: String, access_token_hours: i64, refresh_token_days: i64) -> Self {
        Self {
            secret_key,
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
            exp,
            iat: now.timestamp(),
            jti: jti.clone(),
        };

        // 这里应该使用真正的 JWT 库，比如 jsonwebtoken
        // 为了简化，我们使用简单的 base64 编码
        let token = self.encode_claims(&claims)?;

        Ok(TokenInfo {
            token,
            token_type,
            expires_at: exp,
            user_id,
        })
    }

    /// 验证令牌
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let claims = self.decode_claims(token)?;

        // 检查是否过期
        let now = Utc::now().timestamp();
        if now > claims.exp {
            return Err(AuthError::TokenExpired.into());
        }

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
    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenInfo> {
        let claims = self.verify_token(refresh_token)?;

        // 确保这是一个刷新令牌（通过某种方式标识，这里简化处理）
        let user_id: i32 = claims.sub.parse().map_err(|_| AuthError::InvalidToken)?;

        self.generate_token(user_id, claims.username, claims.role, TokenType::Access)
    }

    /// 编码声明（简化实现，实际应使用 JWT 库）
    fn encode_claims(&self, claims: &Claims) -> Result<String> {
        let json = serde_json::to_string(claims)
            .map_err(|e| AuthError::Other(format!("序列化声明失败: {}", e)))?;

        // 简单的 base64 编码 + 签名
        let encoded = base64::prelude::BASE64_STANDARD.encode(json);
        let signature = self.sign(&encoded)?;

        Ok(format!("{}.{}", encoded, signature))
    }

    /// 解码声明（简化实现，实际应使用 JWT 库）
    fn decode_claims(&self, token: &str) -> Result<Claims> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err(AuthError::InvalidToken.into());
        }

        let encoded_claims = parts[0];
        let signature = parts[1];

        // 验证签名
        let expected_signature = self.sign(encoded_claims)?;
        if signature != expected_signature {
            return Err(AuthError::InvalidToken.into());
        }

        // 解码声明
        let json = base64::prelude::BASE64_STANDARD.decode(encoded_claims).map_err(|_| AuthError::InvalidToken)?;

        let json_str = String::from_utf8(json).map_err(|_| AuthError::InvalidToken)?;

        let claims: Claims =
            serde_json::from_str(&json_str).map_err(|_| AuthError::InvalidToken)?;

        Ok(claims)
    }

    /// 签名（简化实现）
    fn sign(&self, data: &str) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        self.secret_key.hash(&mut hasher);

        Ok(format!("{:x}", hasher.finish()))
    }
}

/// 全局令牌管理器实例
static TOKEN_MANAGER: std::sync::OnceLock<TokenManager> = std::sync::OnceLock::new();

/// 获取全局令牌管理器
pub fn get_token_manager() -> &'static TokenManager {
    TOKEN_MANAGER.get_or_init(|| {
        let secret_key = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_secret_key_change_in_production".to_string());

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
}
