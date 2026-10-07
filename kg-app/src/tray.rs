//! 系统托盘：关窗不退出，从托盘控制播放。
//!
//! 托盘依赖桌面环境的 StatusNotifier / AppIndicator 支持，缺失时只告警，
//! 不影响主窗口正常使用。

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};
use tracing::{info, warn};

use kg_audio::PlayerCommand;

use crate::AppState;

const TRAY_ID: &str = "kgplayer-tray";

/// 把主窗口显示到前台。
fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// 切换主窗口显示 / 隐藏。
fn toggle_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        _ => show_main_window(app),
    }
}

/// 把播放控制交给后台运行时（托盘回调不是 async 上下文）。
fn dispatch<F>(task: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    tauri::async_runtime::spawn(task);
}

pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "播放 / 暂停", true, None::<&str>)?;
    let prev = MenuItem::with_id(app, "prev", "上一首", true, None::<&str>)?;
    let next = MenuItem::with_id(app, "next", "下一首", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出 KGPlayer", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &show,
            &PredefinedMenuItem::separator(app)?,
            &toggle,
            &prev,
            &next,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;

    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("KGPlayer")
        .menu(&menu)
        // 注意：Linux 的 AppIndicator 只能靠左键弹菜单，
        // 这里若关掉左键菜单，托盘菜单（含全部文字）就再也打不开了。
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main_window(app),
            "quit" => {
                info!("从托盘退出");
                app.exit(0);
            }
            "prev" => {
                let app = app.clone();
                dispatch(async move {
                    let state = app.state::<AppState>();
                    let playback = state.playback.clone();
                    if let Err(e) = playback.previous().await {
                        warn!("托盘上一首未生效：{e}");
                    }
                });
            }
            "next" => {
                let app = app.clone();
                dispatch(async move {
                    let state = app.state::<AppState>();
                    let playback = state.playback.clone();
                    if let Err(e) = playback.next().await {
                        warn!("托盘下一首未生效：{e}");
                    }
                });
            }
            "toggle" => {
                let app = app.clone();
                dispatch(async move {
                    let state = app.state::<AppState>();
                    let playing = state
                        .playing
                        .load(std::sync::atomic::Ordering::Relaxed);
                    let cmd = if playing {
                        PlayerCommand::Pause
                    } else {
                        PlayerCommand::Resume
                    };
                    let _ = state.player_cmd_tx.send(cmd);
                });
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            // 左键留给菜单（Linux 必须如此），双击才切换窗口
            if let TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } = event
            {
                toggle_main_window(tray.app_handle());
            }
        });

    // 托盘显式用 256px 图标：窗口图标默认取 32px，在高分屏上会糊
    builder = builder.icon(tauri::include_image!("icons/256x256.png"));

    match builder.build(app) {
        Ok(_) => {
            info!("系统托盘已就绪");
            Ok(())
        }
        Err(e) => {
            // 缺少 AppIndicator 支持的桌面环境很常见，降级而不是崩
            warn!("系统托盘不可用，已跳过：{e}");
            Ok(())
        }
    }
}

/// 拦截关窗：默认收起窗口而不是退出，退出走托盘的「退出 KGPlayer」。
pub fn handle_close_requested(window: &tauri::Window) {
    let _ = window.hide();
}
