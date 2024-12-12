#[cfg(target_os = "windows")]
use tauri::Window;

// #[cfg(target_os = "windows")]
// use windows::Win32::

const WM_SPAWN_WORKER: u32 = 0x052C;

#[cfg(target_os = "windows")]
pub fn set_window_behind(hwnd: HWND) {
    use std::os::raw::c_void;

    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
    };
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_BOTTOM,
            0,
            0,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE,
        )
        .unwrap();
    }
}

use windows::{
    core::s,
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging,
    },
};

extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    let shell_dll_def_view = unsafe {
        WindowsAndMessaging::FindWindowExA(window, HWND::default(), s!("SHELLDLL_DefView"), None)
            .unwrap_or(HWND::default())
    };

    if HWND::is_invalid(&shell_dll_def_view) {
        return BOOL(1);
    }

    let worker_w = unsafe {
        WindowsAndMessaging::FindWindowExA(HWND::default(), window, s!("WorkerW"), None)
            .unwrap_or(HWND::default())
    };

    if HWND::is_invalid(&worker_w) {
        return BOOL(1);
    }

    unsafe {
        *(ref_worker_w.0 as *mut HWND) = worker_w;
    }

    BOOL(1)
}

pub fn attach(webview_window: HWND) -> Result<(), ()> {
    let hwnd = webview_window;

    unsafe {
        let progman_hwnd = WindowsAndMessaging::FindWindowA(s!("Progman"), None).unwrap();

        WindowsAndMessaging::SendMessageA(progman_hwnd, 0x052C, WPARAM(0xD), LPARAM(0x1));

        let mut worker_w: HWND = HWND::default();

        WindowsAndMessaging::EnumWindows(
            Some(enum_window),
            LPARAM(&mut worker_w as *mut HWND as isize),
        )
        .unwrap();

        // if HWND::is_invalid(&worker_w) {
        //     return Err(());
        // }

        WindowsAndMessaging::SetParent(hwnd, worker_w).unwrap();
    }

    Ok(())
}
