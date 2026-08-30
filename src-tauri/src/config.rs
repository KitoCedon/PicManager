/*
 * 生成相应配置文件
 * 没有配置文件时从config.toml.template生成默认配置文件到app_data_dir()
 * 从配置文件加载
 */

use serde::{Serialize,Deserialize};
use std::fs;
use tauri::Manager;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub beds: Vec<Bed>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bed {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub bed_type: String,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub remote_dir: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub local_dir: Option<String>,
}

impl Config {
    pub fn load(app: &tauri::AppHandle) -> Config {
        let dir = app
            .path()
            .app_config_dir()
            .expect("无法获取 app_config 目录");
        let file = dir.join("config.toml");
        if !file.exists() {
            fs::create_dir_all(&dir).expect("无法创建配置目录");
            fs::write(&file, include_str!("../config.toml.template")).expect("无法生成默认配置");
        }
        let content = fs::read_to_string(&file).expect("无法读取配置文件");
        toml::from_str(&content).expect("配置文件无法解析")
    }
}
