use chrono::format;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub db: DbConfig,
    pub server: ServerConfig,
}

fn default_pool_size() -> u32 {
    5
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(default = "default_pool_size")]
    pub max_connections: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load(filename: impl AsRef<Path>) -> Result<Self, Error> {
        // first: 从文件路径中读取文件内容
        let content = fs::read_to_string(filename.as_ref()).map_err(|_| Error::ConfigReadError)?;
        // second: 将文件内容反序列化为Config结构体
        serde_yaml::from_str(&content).map_err(|_| Error::ConfigParseError)
    }
}

impl DbConfig {
    // 获取通用连接字符串
    pub fn server_url(&self) -> String {
        if self.password.is_empty() {
            format!("postgres://{}@{}:{}", self.user, self.host, self.port)
        } else {
            format!(
                "postgres://{}:{}@{}:{}",
                self.user, self.password, self.host, self.port
            )
        }
    }

    // 获取具体数据库连接字符串
    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url(), self.database)
    }
}

impl ServerConfig {
    pub fn url(&self, https: bool) -> String {
        if https {
            format!("https://{}:{}", self.host, self.port)
        } else {
            format!("http://{}:{}", self.host, self.port)
        }
    }
}

// 添加单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_should_be_loaded() {
        let config = Config::load("../service/fixtures/config.yml").unwrap();
        assert_eq!(
            config,
            Config {
                db: DbConfig {
                    host: "10.11.32.24".to_string(),
                    port: 5432,
                    user: "tester".to_string(),
                    password: "test123".to_string(),
                    database: "rsvp".to_string(),
                    max_connections: 5,
                },
                server: ServerConfig {
                    host: "0.0.0.0".to_string(),
                    port: 10020,
                }
            }
        );
    }
}
