use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY, VK_0, VK_MENU, VK_RETURN, VK_SPACE, VK_A, VK_S, VK_W, VK_9};

/// 发送回车键
pub unsafe fn press_enter() {
    key_down(VK_RETURN);
    key_up(VK_RETURN);
}

/// 发送组合按键 按下
unsafe fn key_down(vk: VIRTUAL_KEY) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk.0),
                ..Default::default()
            },
        },
    };
    SendInput(
        std::slice::from_ref(&input),
        std::mem::size_of::<INPUT>() as i32,
    );
}

/// 发送组合按键 抬起
unsafe fn key_up(vk: VIRTUAL_KEY) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk.0),
                dwFlags: KEYEVENTF_KEYUP,
                ..Default::default()
            },
        },
    };
    SendInput(std::slice::from_ref(&input), size_of::<INPUT>() as i32);
}

// 按空格
pub unsafe fn press_white_space() {
    key_down(VK_SPACE);
    key_up(VK_SPACE);
}

// 按组合键 alt+0
pub unsafe fn press_alt_0() {
    key_down(VK_MENU); // Alt down
    key_down(VK_0); // 0 down
    key_up(VK_0); // 0 up
    key_up(VK_MENU); // Alt up
}

// 按组合键 alt+9
pub unsafe fn press_alt_9() {
    key_down(VK_MENU); // Alt down
    key_down(VK_9); // 9 down
    key_up(VK_9); // 9 up
    key_up(VK_MENU); // Alt up
}


/// 向前跳
pub unsafe fn jump_forward() {
    key_down(VK_W); // 按下w
    key_down(VK_SPACE); // 按下空格
    key_up(VK_SPACE); // 抬起空额
    key_up(VK_W); // 抬起w
}

/// 向后跳
pub unsafe fn jump_backward() {
    key_down(VK_S); // Alt down
    key_down(VK_SPACE); // 0 down
    key_up(VK_SPACE); // 0 up
    key_up(VK_S); // Alt up
}