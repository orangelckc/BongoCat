use super::{is_main_window, shared_hide_window, shared_show_window};
use crate::MAIN_WINDOW_LABEL;
use tauri::{command, AppHandle, Runtime, WebviewWindow};
use tauri_nspanel::ManagerExt;

pub enum MacOSPanelStatus {
    Show,
    Hide,
}

#[command]
pub async fn show_window<R: Runtime>(app_handle: AppHandle<R>, window: WebviewWindow<R>) {
    if is_main_window(&window) {
        set_macos_panel(&app_handle, &window, MacOSPanelStatus::Show);
    } else {
        shared_show_window(&app_handle, &window);
    }
}

#[command]
pub async fn hide_window<R: Runtime>(app_handle: AppHandle<R>, window: WebviewWindow<R>) {
    if is_main_window(&window) {
        set_macos_panel(&app_handle, &window, MacOSPanelStatus::Hide);
    } else {
        shared_hide_window(&app_handle, &window);
    }
}

pub fn set_macos_panel<R: Runtime>(
    app_handle: &AppHandle<R>,
    window: &WebviewWindow<R>,
    status: MacOSPanelStatus,
) {
    if is_main_window(window) {
        let app_handle_clone = app_handle.clone();

        let _ = app_handle.run_on_main_thread(move || {
            if let Ok(panel) = app_handle_clone.get_webview_panel(MAIN_WINDOW_LABEL) {
                match status {
                    MacOSPanelStatus::Show => {
                        panel.show();
                    }
                    MacOSPanelStatus::Hide => {
                        panel.order_out(None);
                    }
                }
            }
        });
    }
}
