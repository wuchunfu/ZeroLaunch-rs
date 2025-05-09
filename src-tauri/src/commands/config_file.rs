//use crate::core::storage::onedrive::get_onedrive_refresh_token;
use crate::core::storage::storage_manager::check_validation;
use crate::modules::config::config_manager::PartialRuntimeConfig;
use crate::modules::config::default::REMOTE_CONFIG_DEFAULT;
use crate::modules::config::load_local_config;
use crate::save_config_to_file;
use crate::storage::config::PartialLocalConfig;
use crate::update_app_setting;
use crate::AppState;
use crate::REMOTE_CONFIG_NAME;
use std::sync::Arc;
use tauri::Emitter;
use tauri::Manager;
use tauri::Runtime;
use tracing::error;

/// 更新程序管理器的路径配置
#[tauri::command]
pub async fn command_save_remote_config<R: Runtime>(
    _app: tauri::AppHandle<R>,
    state: tauri::State<'_, Arc<AppState>>,
    partial_config: PartialRuntimeConfig,
) -> Result<(), String> {
    let runtime_config = state.get_runtime_config().unwrap();
    runtime_config.update(partial_config);
    save_config_to_file(true).await;
    Ok(())
}

#[tauri::command]
pub async fn command_load_local_config<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<PartialLocalConfig, String> {
    let storage_manager = state.get_storage_manager().unwrap();
    Ok(storage_manager.to_partial().await)
}

#[tauri::command]
pub async fn command_save_local_config<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    state: tauri::State<'_, Arc<AppState>>,
    partial_config: PartialLocalConfig,
) -> Result<(), String> {
    let storage_manager = state.get_storage_manager().unwrap();
    storage_manager.upload_all_file_force().await;
    storage_manager.update(partial_config).await;

    let runtime_config = state.get_runtime_config().unwrap();

    let remote_config_data = {
        if let Some(data) = storage_manager
            .download_file_str(REMOTE_CONFIG_NAME.to_string())
            .await
        {
            data
        } else {
            storage_manager
                .upload_file_str(
                    REMOTE_CONFIG_NAME.to_string(),
                    REMOTE_CONFIG_DEFAULT.clone(),
                )
                .await;
            REMOTE_CONFIG_DEFAULT.clone()
        }
    };

    let partial_config = load_local_config(&remote_config_data);
    runtime_config.update(partial_config);
    update_app_setting().await;
    let setting_window = app.get_webview_window("setting_window").unwrap();
    if let Err(e) = setting_window.emit("emit_update_setting_window_config", "") {
        error!("向 setting_window 发送信号失败: {:?}", e);
    }
    Ok(())
}

#[tauri::command]
pub async fn command_check_validation<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    partial_config: PartialLocalConfig,
) -> Result<Option<PartialLocalConfig>, String> {
    Ok(check_validation(partial_config).await)
}

// #[tauri::command]
// pub async fn command_get_onedrive_refresh_token<R: Runtime>(
//     app: tauri::AppHandle<R>,
//     window: tauri::Window<R>,
// ) -> Result<String, String> {
//     get_onedrive_refresh_token(window).await
// }
