// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{
    generate_handler, async_runtime::spawn,
    Manager, menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, State, WindowEvent,
};

// ============== 共享状态 ==============
#[derive(Default)]
struct LoadState {
    backend: bool,
    frontend: bool,
}

// ============== 前端调用的命令 ==============
#[tauri::command]
async fn set_loaded(
    app: AppHandle,
    state: State<'_, Mutex<LoadState>>
) -> Result<(), ()> {
    state.lock().unwrap().frontend = true;
    check_ready(&app, state);
    Ok(())
}

// ============== 后端初始化任务 ==============
async fn backend_setup(app: AppHandle) -> Result<(), ()> {
    // 模拟耗时3秒
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    let state = app.state::<Mutex<LoadState>>();
    state.lock().unwrap().backend = true;
    
    check_ready(&app, state);
    Ok(())
}

// ============== 检查加载完成 ==============
fn check_ready(app: &AppHandle, state: State<'_, Mutex<LoadState>>) {
    let state = state.lock().unwrap();
    if state.backend && state.frontend {
        // 关闭启动屏
        let splash = app.get_webview_window("splashscreen").unwrap();
        let _ = splash.close();
        
        // 显示主窗口
        let main = app.get_webview_window("main").unwrap();
        let _ = main.show();
        let _ = main.set_focus();
    }
}

// ============== 主应用入口 ==============
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        // 注册共享状态
        .manage(Mutex::new(LoadState::default()))
        // 注册前端命令
        .invoke_handler(generate_handler![set_loaded])
        // 托盘 + 初始化配置
        .setup(|app| {
            // 托盘菜单
            let quit_item = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_item])?;

            // 托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let win = tray.app_handle().get_webview_window("main").unwrap();
                        let _ = win.show();
                        let _ = win.unminimize();
                        let _ = win.set_focus();
                    }
                    _ => {}
                })
                .on_menu_event(|app, event| {
                    if event.id() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

            // 启动后端异步任务
            spawn(backend_setup(app.handle().clone()));
            
            Ok(())
        })
        // 窗口事件
        .on_window_event(|window, event| {
            match event {
                // 关闭按钮：隐藏窗口
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = window.hide();
                }
                // 最小化：隐藏窗口
                WindowEvent::Resized(_) => {
                    if window.is_minimized().unwrap_or(false) {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })
        // 启动应用
        .run(tauri::generate_context!())?;

    Ok(())
}