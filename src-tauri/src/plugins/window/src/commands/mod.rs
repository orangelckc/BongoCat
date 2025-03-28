use tauri::{async_runtime::spawn, AppHandle, Manager, Runtime, WebviewWindow};

pub static MAIN_WINDOW_LABEL: &str = "main";
pub static PREFERENCE_WINDOW_LABEL: &str = "preference";

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub use linux::*;

pub fn is_main_window<R: Runtime>(window: &WebviewWindow<R>) -> bool {
    window.label() == MAIN_WINDOW_LABEL
}

fn shared_show_window<R: Runtime>(app_handle: &AppHandle<R>, window: &WebviewWindow<R>) {
    let _ = window.show();
    let _ = window.unminimize();
    let _ = window.set_focus();

    let _ = app_handle;
}

fn shared_hide_window<R: Runtime>(app_handle: &AppHandle<R>, window: &WebviewWindow<R>) {
    let _ = window.hide();

    let _ = app_handle;
}

pub fn show_main_window(app_handle: &AppHandle) {
    show_window_by_label(app_handle, MAIN_WINDOW_LABEL);
}

pub fn show_preference_window(app_handle: &AppHandle) {
    show_window_by_label(app_handle, PREFERENCE_WINDOW_LABEL);
}

fn show_window_by_label(app_handle: &AppHandle, label: &str) {
    if let Some(window) = app_handle.get_webview_window(label) {
        let app_handle_clone = app_handle.clone();

        spawn(async move {
            show_window(app_handle_clone, window).await;
        });
    }
}
