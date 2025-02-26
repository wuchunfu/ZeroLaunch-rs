pub mod access_policy;
pub mod defer;
pub mod service_locator;
pub mod ui_controller;
pub mod windows;
use std::path::Path;

use crate::modules::config::default::REMOTE_CONFIG_NAME;
use chrono::{Local, NaiveDate};
use dashmap::DashMap;
use service_locator::ServiceLocator;
use std::collections::HashMap;
use std::hash::Hash;

/// 生成当前日期的函数
pub fn generate_current_date() -> String {
    let current_date = Local::now().date_naive();
    current_date.format("%Y-%m-%d").to_string()
}

/// 比较日期字符串与当前日期的函数
pub fn is_date_current(date_str: &str) -> bool {
    // 解析输入的日期字符串
    let input_date = match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return false, // 如果解析失败,返回false
    };

    // 获取当前日期
    let current_date = Local::now().date_naive();

    // 比较两个日期
    input_date == current_date
}

// 将 DashMap 转换为 HashMap
pub fn dashmap_to_hashmap<K, V>(dash_map: &DashMap<K, V>) -> HashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    dash_map
        .iter()
        .map(|r| (r.key().clone(), r.value().clone()))
        .collect()
}

// 将 HashMap 转换为 DashMap
pub fn hashmap_to_dashmap<K, V>(hash_map: &HashMap<K, V>) -> DashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    let dash_map = DashMap::with_capacity(hash_map.len());
    for (key, value) in hash_map {
        dash_map.insert(key.clone(), value.clone());
    }
    dash_map
}

// 获得远程配置文件的保存地址
pub fn get_remote_config_path() -> String {
    let state = ServiceLocator::get_state();
    let dir_path = state.get_remote_config_dir_path();
    let config_path = Path::new(&dir_path).join(REMOTE_CONFIG_NAME);
    config_path.to_str().unwrap().to_string()
}
