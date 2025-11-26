use std::thread::sleep;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, SendInput, VIRTUAL_KEY, VK_1,
    VK_MENU, VK_RETURN,
};
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;
use windows::core::PCSTR;

fn main() {
    loop {
        find_window("");

        unsafe {
            press_alt_1();
        }

        sleep(Duration::from_secs(60));

        unsafe {
            press_enter();
        }

        sleep(Duration::from_secs(80));
    }
}

/// 查找窗口
fn find_window(title: &str) -> isize {
    unsafe { FindWindowA(None, PCSTR(title.as_ptr())).0 }
}

/// 发送按键
unsafe fn press_key(vk: u16) {
    let mut input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),
                ..Default::default()
            },
        },
    };
    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);

    // key up
    input.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS(2);
    SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
}

// 按组合键 alt+1
unsafe fn press_alt_1() {
    press_key(VK_MENU.0);
    press_key(VK_1.0);
}

// 按回车键
unsafe fn press_enter() {
    press_key(VK_RETURN.0);
}
