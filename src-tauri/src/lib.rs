use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
fn desktop_notify(app: tauri::AppHandle, title: String, body: String) {
    let _ = app.notification().builder().title(title).body(body).show();
}

fn inject_watchers_script(window: &tauri::WebviewWindow) {
    let js = r#"
(() => {
  if ((window).__dorunDesktopWatcherInstalled) return;
  (window).__dorunDesktopWatcherInstalled = true;

  const invokeNotify = async (title, body) => {
    try {
      if (window.__TAURI__?.core?.invoke) {
        await window.__TAURI__.core.invoke('desktop_notify', { title, body });
      }
    } catch (_) {}
  };

  const state = { msg: null, wall: null };

  const getUnreadMessageCount = async () => {
    const paths = ['/api/messages/unread-count', '/api/messenger/unread-count'];
    for (const p of paths) {
      try {
        const r = await fetch(p, { credentials: 'include' });
        if (!r.ok) continue;
        const d = await r.json();
        const n = Number(d?.unreadCount ?? d?.count ?? d?.unread ?? 0);
        if (!Number.isNaN(n)) return n;
      } catch (_) {}
    }
    return null;
  };

  const getWallCount = async () => {
    try {
      const r = await fetch('/api/wall/posts?page=1&pageSize=1', { credentials: 'include' });
      if (!r.ok) return null;
      const d = await r.json();
      const n = Number(d?.total ?? d?.count ?? (Array.isArray(d?.items) ? d.items.length : 0));
      if (Number.isNaN(n)) return null;
      return n;
    } catch (_) {
      return null;
    }
  };

  const poll = async () => {
    const [msg, wall] = await Promise.all([getUnreadMessageCount(), getWallCount()]);

    if (msg !== null) {
      if (state.msg !== null && msg > state.msg) {
        invokeNotify('두런허브', `새 메시지 ${msg - state.msg}개 도착`);
      }
      state.msg = msg;
    }

    if (wall !== null) {
      if (state.wall !== null && wall > state.wall) {
        invokeNotify('두런허브', `담벼락 새 글 ${wall - state.wall}개`);
      }
      state.wall = wall;
    }
  };

  poll();
  setInterval(poll, 30000);
})();
"#;

    let _ = window.eval(js);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![desktop_notify])
        .setup(|app| {
            // 모바일처럼 컴팩트한 메신저 중심 창
            let win = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://dorunhub.com".parse().unwrap()),
            )
            .title("두런허브")
            .inner_size(430.0, 900.0)
            .min_inner_size(360.0, 720.0)
            .resizable(true)
            .center()
            .decorations(true)
            .build()?;

            inject_watchers_script(&win);

            // 트레이 아이콘 메뉴
            let quit = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "두런허브 열기", true, None::<&str>)?;
            let compact = MenuItem::with_id(app, "compact", "컴팩트 크기(430x900)", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &compact, &quit])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("두런허브")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "compact" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.set_size(tauri::Size::Logical(tauri::LogicalSize::new(430.0, 900.0)));
                            let _ = win.center();
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        // 창 닫기 → 트레이로 최소화
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("두런허브 앱 실행 중 오류 발생");
}
