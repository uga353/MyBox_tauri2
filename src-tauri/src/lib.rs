use std::sync::Mutex;
use tauri::{
    async_runtime::spawn,
    AppHandle, Manager, State, WindowEvent
};
use tauri_plugin_prevent_default::Flags;

// 引入托盘模块
#[cfg(desktop)]
mod tray;

#[derive(Default)]
pub struct LoadState {
    backend: bool,
    frontend: bool,
}

async fn backend_setup(app: AppHandle) -> Result<(), ()> {
    //模拟1.6秒程序处理的时间
    tokio::time::sleep(tokio::time::Duration::from_millis(1600)).await;

    if let Some(state) = app.try_state::<Mutex<LoadState>>() {
        if let Ok(mut guard) = state.lock() {
            guard.backend = true;
            guard.frontend = true;
        }
        check_ready(&app, state);
    }

    Ok(())
}

pub fn check_ready(app: &AppHandle, state: State<'_, Mutex<LoadState>>) {
    let (backend_ok, frontend_ok) = {
        let Ok(guard) = state.lock() else { return };
        (guard.backend, guard.frontend)
    };

    if !backend_ok || !frontend_ok {
        return;
    }

    // 关闭启动屏
    if let Some(splash) = app.get_webview_window("splashscreen") {
        let _ = splash.close();
    }

    // 显示主窗口
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.show();
        let _ = main.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let prevent = tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::CONTEXT_MENU | Flags::DEV_TOOLS) // 仅禁用右键和开发者工具
        .build();

    tauri::Builder::default()
        .manage(Mutex::new(LoadState::default()))
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent)
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let app_handle = app.handle();
            spawn(backend_setup(app_handle.clone()));
            
            // 仅桌面端启用托盘
            #[cfg(desktop)]
                tray::create_tray(&app_handle)?;
                       
            Ok(())
        })

        // 窗口事件
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = window.hide();
                }
                WindowEvent::Resized(_) => {
                    if window.is_minimized().unwrap_or(false) {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })

        .run(tauri::generate_context!())
        .expect("程序启动失败！");
        
    Ok(())
}