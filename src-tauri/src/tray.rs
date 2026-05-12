use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Runtime,
    Manager,
};

// 初始化系统托盘
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let tray_menu = Menu::with_items(app, &[
        &MenuItem::with_id(app, "open", "🏠 主 页", true, None::<&str>)?,
        //&MenuItem::with_id(app, "sep",  "───────────────", false, None::<&str>)?,
        &MenuItem::with_id(app, "quit", "❎ 退 出", true, None::<&str>)?,
    ])?;

    let _tray = TrayIconBuilder::new()
        .menu(&tray_menu)
        .icon(app.default_window_icon().cloned().unwrap())
        .tooltip("MyBox")
        .show_menu_on_left_click(false)
        .on_tray_icon_event({
            let app = app.clone();
            move |_, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event {
                    if let Some(main_win) = app.get_webview_window("main") {
                        let _ = main_win.unminimize();
                        let _ = main_win.show();
                        let _ = main_win.set_focus();
                    }
                }
            }
        })
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "open" => {
                    if let Some(main_win) = app.get_webview_window("main") {
                        let _ = main_win.unminimize();
                        let _ = main_win.show();
                        let _ = main_win.set_focus();
                    }
                }
                "quit" => app.exit(0),

                _ => {}
            }
        })
        .build(app)?;
        
    Ok(())
}