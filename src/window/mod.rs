use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible, SetForegroundWindow, ShowWindow, SW_RESTORE};
use crate::FOUND_HWND;

/// 让窗口保持在前台
pub fn activate_window(hwnd: isize) -> bool {
    unsafe {
        let handle = HWND(hwnd);

        // 如果窗口被最小化，则恢复
        let _ = ShowWindow(handle, SW_RESTORE);

        // 尝试设为前台窗口
        SetForegroundWindow(handle).as_bool()
    }
}

/// 枚举所有的窗口，得到wow的窗口
pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // 从 lParam 恢复关键字指针
    let keyword: &String = &*(lparam.0 as *const String);

    if IsWindowVisible(hwnd).as_bool() {
        let mut buf = [0u16; 1024];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len > 0 {
            // 转换 UTF-16 → Rust String
            let title = String::from_utf16_lossy(&buf[..len as usize]);
            println!("句柄: {:?}, 标题: {}", hwnd, title);

            if title.eq(keyword) {
                println!("匹配成功: [{}] HWND={:?}", title, hwnd);
                FOUND_HWND = Some(hwnd);
                return BOOL::from(false); // 停止遍历
            }
        }
    }
    BOOL::from(true)
}

/// 根据要寻找的进程窗口标题，找到对应的窗口对象
pub fn find_window_by_keyword(keyword: &str) -> Option<HWND> {
    unsafe {
        FOUND_HWND = None;

        // 把 keyword 放入 Box，保证指针有效
        let boxed = Box::new(keyword.to_string());
        let ptr = Box::into_raw(boxed);

        EnumWindows(Some(enum_windows_proc), LPARAM(ptr as isize));

        // 恢复 Box 避免泄露
        let _ = Box::from_raw(ptr);

        FOUND_HWND
    }
}