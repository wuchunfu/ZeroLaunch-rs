use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialUiConfig {
    pub selected_item_color: Option<String>,
    pub item_font_color: Option<String>,
    pub search_bar_font_color: Option<String>,
    pub search_bar_background_color: Option<String>,
    pub item_font_size: Option<f64>,
    pub search_bar_font_size: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct UiConfigInner {
    /// 显示器的大小与窗口的大小的比例
    /// 选中项的颜色
    #[serde(default = "UiConfigInner::default_selected_item_color")]
    pub selected_item_color: String,

    /// 选项中的字体的颜色
    #[serde(default = "UiConfigInner::default_item_font_color")]
    pub item_font_color: String,

    /// 搜索栏的字体颜色
    #[serde(default = "UiConfigInner::default_search_bar_font_color")]
    pub search_bar_font_color: String,

    /// 搜索栏与状态栏的背景颜色
    #[serde(default = "UiConfigInner::default_search_bar_background_color")]
    pub search_bar_background_color: String,

    /// 结果栏的字体大小
    #[serde(default = "UiConfigInner::default_item_font_size")]
    pub item_font_size: f64,

    /// 搜索栏的字体大小
    #[serde(default = "UiConfigInner::default_search_bar_font_size")]
    pub search_bar_font_size: f64,

    /// 测试
    #[serde(default = "UiConfigInner::default_search_bar_font_size")]
    pub test: f64,
}

impl Default for UiConfigInner {
    fn default() -> Self {
        Self {
            selected_item_color: Self::default_selected_item_color(),
            item_font_color: Self::default_item_font_color(),
            search_bar_font_color: Self::default_search_bar_font_color(),
            search_bar_background_color: Self::default_search_bar_background_color(),
            item_font_size: Self::default_item_font_size(),
            search_bar_font_size: Self::default_search_bar_font_size(),
            test: Self::default_search_bar_font_size(),
        }
    }
}

impl UiConfigInner {
    pub(crate) fn default_selected_item_color() -> String {
        "#e3e3e3cc".to_string()
    }

    pub(crate) fn default_item_font_color() -> String {
        "#000000".to_string()
    }

    pub(crate) fn default_search_bar_font_color() -> String {
        "#333333".to_string()
    }

    pub(crate) fn default_search_bar_background_color() -> String {
        "#FFFFFF00".to_string()
    }

    pub(crate) fn default_item_font_size() -> f64 {
        1.3
    }

    pub(crate) fn default_search_bar_font_size() -> f64 {
        2.0
    }
}

impl UiConfigInner {
    pub fn update(&mut self, partial_ui_config: PartialUiConfig) {
        if let Some(selected_item_color) = partial_ui_config.selected_item_color {
            self.selected_item_color = selected_item_color;
        }
        if let Some(item_font_color) = partial_ui_config.item_font_color {
            self.item_font_color = item_font_color;
        }
        if let Some(search_bar_font_color) = partial_ui_config.search_bar_font_color {
            self.search_bar_font_color = search_bar_font_color;
        }
        if let Some(search_bar_background_color) = partial_ui_config.search_bar_background_color {
            self.search_bar_background_color = search_bar_background_color;
        }
        if let Some(item_font_size) = partial_ui_config.item_font_size {
            self.item_font_size = item_font_size;
        }
        if let Some(search_bar_font_size) = partial_ui_config.search_bar_font_size {
            self.search_bar_font_size = search_bar_font_size;
        }
    }

    pub fn get_selected_item_color(&self) -> String {
        self.selected_item_color.clone()
    }
    pub fn get_item_font_color(&self) -> String {
        self.item_font_color.clone()
    }

    pub fn get_search_bar_font_color(&self) -> String {
        self.search_bar_font_color.clone()
    }

    pub fn get_search_bar_background_color(&self) -> String {
        self.search_bar_background_color.clone()
    }

    pub fn get_item_font_size(&self) -> f64 {
        self.item_font_size
    }

    pub fn get_search_bar_font_size(&self) -> f64 {
        self.search_bar_font_size
    }

    pub fn to_partial(&self) -> PartialUiConfig {
        PartialUiConfig {
            selected_item_color: Some(self.selected_item_color.clone()),
            item_font_color: Some(self.item_font_color.clone()),
            search_bar_font_color: Some(self.search_bar_font_color.clone()),
            search_bar_background_color: Some(self.search_bar_background_color.clone()),
            item_font_size: Some(self.item_font_size),
            search_bar_font_size: Some(self.search_bar_font_size),
        }
    }
}
#[derive(Debug)]
pub struct UiConfig {
    inner: RwLock<UiConfigInner>,
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            inner: RwLock::new(UiConfigInner::default()),
        }
    }
}

impl UiConfig {
    pub fn update(&self, partial_ui_config: PartialUiConfig) {
        let mut inner = self.inner.write();
        inner.update(partial_ui_config);
    }

    pub fn get_selected_item_color(&self) -> String {
        let inner = self.inner.read();
        inner.selected_item_color.clone()
    }
    pub fn get_item_font_color(&self) -> String {
        let inner = self.inner.read();
        inner.item_font_color.clone()
    }

    pub fn to_partial(&self) -> PartialUiConfig {
        let inner = self.inner.read();
        inner.to_partial()
    }

    pub fn get_search_bar_font_color(&self) -> String {
        let inner = self.inner.read();
        inner.search_bar_font_color.clone()
    }

    pub fn get_search_bar_background_color(&self) -> String {
        let inner = self.inner.read();
        inner.search_bar_background_color.clone()
    }

    pub fn get_item_font_size(&self) -> f64 {
        let inner = self.inner.read();
        inner.item_font_size
    }

    pub fn get_search_bar_font_size(&self) -> f64 {
        let inner = self.inner.read();
        inner.search_bar_font_size
    }
}

// // 手动实现序列化
// impl Serialize for UiConfig {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // 获取读锁后序列化内部数据
//         let inner = self.inner.read();
//         inner.serialize(serializer)
//     }
// }

// // 手动实现反序列化
// impl<'de> Deserialize<'de> for UiConfig {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         // 先反序列化出内部数据
//         let inner = UiConfigInner::deserialize(deserializer)?;
//         // 用 RwLock 包装后返回
//         Ok(UiConfig {
//             inner: RwLock::new(inner),
//         })
//     }
// }

// // 手动实现 Clone
// impl Clone for UiConfig {
//     fn clone(&self) -> Self {
//         // 获取读锁后克隆内部数据
//         let inner_data = self.inner.read().clone();
//         UiConfig {
//             inner: RwLock::new(inner_data),
//         }
//     }
// }
