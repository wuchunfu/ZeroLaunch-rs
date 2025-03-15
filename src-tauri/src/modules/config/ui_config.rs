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
    pub vertical_position_ratio: Option<f64>,
    pub search_bar_height: Option<u32>,
    pub result_item_height: Option<u32>,
    pub footer_height: Option<u32>,
    pub window_width: Option<u32>,
    pub background_size: Option<String>,
    pub background_position: Option<String>,
    pub background_repeat: Option<String>,
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

    /// 垂直方向偏移比例因子
    #[serde(default = "UiConfigInner::default_vertical_position_ratio")]
    pub vertical_position_ratio: f64,

    /// 搜索栏的高度
    #[serde(default = "UiConfigInner::default_search_bar_height")]
    pub search_bar_height: u32,

    /// 结果栏中一项的高度
    #[serde(default = "UiConfigInner::default_result_item_height")]
    pub result_item_height: u32,

    /// 底栏的高度（为0时则隐藏）
    #[serde(default = "UiConfigInner::default_footer_height")]
    pub footer_height: u32,

    /// 程序的宽度
    #[serde(default = "UiConfigInner::default_window_width")]
    pub window_width: u32,

    /// 背景图片的大小
    pub background_size: String,
    /// 背景图片的位置
    pub background_position: String,
    /// 背景图片的重复
    pub background_repeat: String,
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
            vertical_position_ratio: Self::default_vertical_position_ratio(),
            search_bar_height: Self::default_search_bar_height(),
            result_item_height: Self::default_result_item_height(),
            footer_height: Self::default_footer_height(),
            window_width: Self::default_window_width(),
            background_size: Self::default_background_size(),
            background_position: Self::default_background_position(),
            background_repeat: Self::default_background_repeat(),
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

    pub(crate) fn default_vertical_position_ratio() -> f64 {
        0.4
    }

    pub(crate) fn default_search_bar_height() -> u32 {
        65
    }

    pub(crate) fn default_result_item_height() -> u32 {
        62
    }

    pub(crate) fn default_footer_height() -> u32 {
        42
    }

    pub(crate) fn default_window_width() -> u32 {
        1000
    }

    pub(crate) fn default_background_size() -> String {
        "cover".to_string()
    }

    pub(crate) fn default_background_position() -> String {
        "center".to_string()
    }

    pub(crate) fn default_background_repeat() -> String {
        "no-repeat".to_string()
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
        if let Some(vertical_position_ratio) = partial_ui_config.vertical_position_ratio {
            self.vertical_position_ratio = vertical_position_ratio;
        }
        if let Some(search_bar_height) = partial_ui_config.search_bar_height {
            self.search_bar_height = search_bar_height;
        }
        if let Some(result_item_height) = partial_ui_config.result_item_height {
            self.result_item_height = result_item_height;
        }
        if let Some(footer_height) = partial_ui_config.footer_height {
            self.footer_height = footer_height;
        }
        if let Some(window_width) = partial_ui_config.window_width {
            self.window_width = window_width;
        }
        if let Some(background_size) = partial_ui_config.background_size {
            self.background_size = background_size;
        }
        if let Some(background_position) = partial_ui_config.background_position {
            self.background_position = background_position;
        }
        if let Some(background_repeat) = partial_ui_config.background_repeat {
            self.background_repeat = background_repeat;
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

    pub fn get_search_bar_height(&self) -> u32 {
        self.search_bar_height
    }

    pub fn get_result_item_height(&self) -> u32 {
        self.result_item_height
    }

    pub fn get_footer_height(&self) -> u32 {
        self.footer_height
    }

    pub fn to_partial(&self) -> PartialUiConfig {
        PartialUiConfig {
            selected_item_color: Some(self.selected_item_color.clone()),
            item_font_color: Some(self.item_font_color.clone()),
            search_bar_font_color: Some(self.search_bar_font_color.clone()),
            search_bar_background_color: Some(self.search_bar_background_color.clone()),
            item_font_size: Some(self.item_font_size),
            search_bar_font_size: Some(self.search_bar_font_size),
            vertical_position_ratio: Some(self.vertical_position_ratio),
            search_bar_height: Some(self.search_bar_height),
            result_item_height: Some(self.result_item_height),
            footer_height: Some(self.footer_height),
            window_width: Some(self.window_width),
            background_size: Some(self.background_size.clone()),
            background_position: Some(self.background_position.clone()),
            background_repeat: Some(self.background_repeat.clone()),
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

    pub fn get_vertical_position_ratio(&self) -> f64 {
        let inner = self.inner.read();
        inner.vertical_position_ratio
    }

    pub fn get_search_bar_height(&self) -> u32 {
        let inner = self.inner.read();
        inner.search_bar_height
    }

    pub fn get_result_item_height(&self) -> u32 {
        let inner = self.inner.read();
        inner.result_item_height
    }

    pub fn get_footer_height(&self) -> u32 {
        let inner = self.inner.read();
        inner.footer_height
    }
    pub fn get_window_width(&self) -> u32 {
        let inner = self.inner.read();
        inner.window_width
    }

    pub fn get_background_size(&self) -> String {
        let inner = self.inner.read();
        inner.background_size.clone()
    }

    pub fn get_background_position(&self) -> String {
        let inner = self.inner.read();
        inner.background_position.clone()
    }

    pub fn get_background_repeat(&self) -> String {
        let inner = self.inner.read();
        inner.background_repeat.clone()
    }
}
