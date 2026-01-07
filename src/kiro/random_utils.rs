//! 随机标识生成工具
//!
//! 同步自 kiro2api 的实现，用于生成随机化的 User-Agent 组件
//! 降低被识别为同一客户端的风险

/// 生成指定范围内的随机整数 [min, max]
#[inline]
fn random_int(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    fastrand::u32(min..=max)
}

/// 生成随机 Git 提交哈希（40 字符十六进制）
pub fn generate_random_git_hash() -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let mut hash = String::with_capacity(40);
    for _ in 0..40 {
        let idx = fastrand::usize(..16);
        hash.push(HEX_CHARS[idx] as char);
    }
    hash
}

/// 生成随机 OS 版本（模拟不同的 Electron 环境）
///
/// 范围: 13.7.x.x-electron.0 ~ 13.9.x.x-electron.0
pub fn generate_random_os_version() -> String {
    let major = 13;
    let minor = random_int(7, 9);       // 7-9
    let patch = random_int(0, 99);      // 0-99
    let build = random_int(0, 299);     // 0-299
    format!("{}.{}.{}.{}-electron.0", major, minor, patch, build)
}

/// 生成随机 Node/Chromium 版本
///
/// 范围: 138.0.7200.x ~ 138.0.7210.x
pub fn generate_random_node_version() -> String {
    let major = 138;
    let minor = 0;
    let patch = random_int(7200, 7210); // 7200-7210
    let build = random_int(0, 999);     // 0-999
    format!("{}.{}.{}.{}", major, minor, patch, build)
}

/// User-Agent 头部信息
pub struct UserAgentHeaders {
    pub x_amzn_kiro_agent_mode: &'static str,
    pub x_amz_user_agent: String,
    pub user_agent: String,
}

/// 构建随机化的 User-Agent 请求头
///
/// 保守随机化策略：
/// - 固定版本：SDK 版本、Kiro IDE 版本
/// - 随机版本：OS 版本、Node 版本、Git Hash
pub fn build_user_agent_headers(kiro_version: &str) -> UserAgentHeaders {
    // 固定版本（保持稳定）
    const SDK_VERSION: &str = "1.0.18";

    // 随机版本（模拟不同用户环境）
    let os_version = generate_random_os_version();
    let node_version = generate_random_node_version();
    let hash = generate_random_git_hash();

    UserAgentHeaders {
        x_amzn_kiro_agent_mode: "spec",
        x_amz_user_agent: format!(
            "aws-sdk-js/{} KiroIDE-{}-{}",
            SDK_VERSION, kiro_version, hash
        ),
        user_agent: format!(
            "aws-sdk-js/{} ua/2.1 os/{} lang/js md/nodejs#{} api/codewhispererstreaming#{} m/E KiroIDE-{}-{}",
            SDK_VERSION, os_version, node_version, SDK_VERSION, kiro_version, hash
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_int() {
        for _ in 0..100 {
            let val = random_int(5, 10);
            assert!(val >= 5 && val <= 10);
        }
        // 边界情况
        assert_eq!(random_int(5, 5), 5);
        assert_eq!(random_int(10, 5), 10);
    }

    #[test]
    fn test_generate_random_git_hash() {
        let hash = generate_random_git_hash();
        assert_eq!(hash.len(), 40);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_random_os_version() {
        let version = generate_random_os_version();
        assert!(version.starts_with("13."));
        assert!(version.ends_with("-electron.0"));
    }

    #[test]
    fn test_generate_random_node_version() {
        let version = generate_random_node_version();
        assert!(version.starts_with("138.0."));
    }

    #[test]
    fn test_build_user_agent_headers() {
        let headers = build_user_agent_headers("0.8.0");

        assert_eq!(headers.x_amzn_kiro_agent_mode, "spec");
        assert!(headers.x_amz_user_agent.contains("aws-sdk-js/1.0.18"));
        assert!(headers.x_amz_user_agent.contains("KiroIDE-0.8.0-"));
        assert!(headers.user_agent.contains("aws-sdk-js/1.0.18"));
        assert!(headers.user_agent.contains("-electron.0"));
        assert!(headers.user_agent.contains("138.0."));
    }

    #[test]
    fn test_randomness() {
        // 验证每次生成的值不同（概率性测试）
        let hash1 = generate_random_git_hash();
        let hash2 = generate_random_git_hash();
        // 两个随机哈希相同的概率极低
        assert_ne!(hash1, hash2);
    }
}
