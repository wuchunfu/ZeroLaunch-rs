use crate::core::image_processor::ImageIdentity;
use crate::core::image_processor::ImageProcessor;
use crate::modules::config::app_config::PartialAppConfig;
use crate::modules::config::default::APP_PIC_PATH;
use crate::modules::config::ui_config::PartialUiConfig;
use crate::modules::shortcut_manager::shortcut_config::PartialShortcutConfig;
use crate::state::app_state::AppState;
use crate::utils::service_locator::ServiceLocator;
use crate::utils::ui_controller::handle_focus_lost;
use std::sync::Arc;
use tauri::image::Image;
use tauri::Emitter;
use tauri::Manager;
use tauri::Runtime;

#[tauri::command]
pub async fn update_search_bar_window<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<(PartialAppConfig, PartialUiConfig, PartialShortcutConfig), String> {
    let runtime_config = state.get_runtime_config().unwrap();
    let app_config = runtime_config.get_app_config();
    let ui_config = runtime_config.get_ui_config();
    let shortcut_config = runtime_config.get_shortcut_config();
    Ok((
        app_config.to_partial(),
        ui_config.to_partial(),
        shortcut_config.to_partial(),
    ))
}

#[tauri::command]
pub async fn get_background_picture<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<u8>, String> {
    let storage_manager = state.get_storage_manager().unwrap();
    if let Some(data) = storage_manager
        .download_file_bytes("background.png".to_string())
        .await
    {
        return Ok(data);
    } else {
        storage_manager
            .upload_file_bytes("background.png".to_string(), Vec::new())
            .await;
    }
    Ok(Vec::new())
}

#[tauri::command]
pub async fn get_remote_config_dir<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let storage_manager = state.get_storage_manager().unwrap();
    let path = storage_manager.get_target_dir_path().await;
    Ok(path)
}

#[tauri::command]
pub async fn select_background_picture<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, Arc<AppState>>,
    path: String,
) -> Result<(), String> {
    let path = ImageIdentity::File(path);
    let content: Vec<u8> = ImageProcessor::load_image(&path).await;
    let storage_manager = state.get_storage_manager().unwrap();
    storage_manager
        .upload_file_bytes("background.png".to_string(), content)
        .await;
    app.emit("update_search_bar_window", "").unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_dominant_color<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    path: String,
) -> Result<String, String> {
    let path = ImageIdentity::File(path);
    let content = ImageProcessor::load_image(&path).await;
    let ret = ImageProcessor::get_dominant_color(content).await.unwrap();
    Ok(format!("rgba({}, {}, {}, 0.8)", ret.0, ret.1, ret.2))
}

/// 隐藏窗口
#[tauri::command]
pub fn hide_window() -> Result<(), String> {
    let state = ServiceLocator::get_state();
    let main_window = state
        .get_main_handle()
        .unwrap()
        .get_webview_window("main")
        .unwrap();
    handle_focus_lost(Arc::new(main_window));
    Ok(())
}

/// 展示设置窗口
#[tauri::command]
pub fn show_setting_window() -> Result<(), String> {
    let state = ServiceLocator::get_state();
    let setting_window = state
        .get_main_handle()
        .unwrap()
        .get_webview_window("setting_window")
        .unwrap();
    let _ = setting_window.unminimize();
    setting_window.show().unwrap();
    setting_window.set_focus().unwrap();
    hide_window().unwrap();
    Ok(())
}

/// 用于更改系统托盘图标的颜色

#[tauri::command]
pub async fn command_change_tray_icon<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, Arc<AppState>>,
    is_dark: bool,
) -> Result<(), String> {
    let key = {
        if is_dark {
            "tray_icon_white"
        } else {
            "tray_icon"
        }
    };

    let icon_path = APP_PIC_PATH.get(key).unwrap();
    let tray_icon = state.get_tray_icon().unwrap();
    if let Err(e) = tray_icon.set_icon(Some(Image::from_path(icon_path.value()).unwrap())) {
        return Err(format!("error: {:?}", e));
    }
    Ok(())
}
