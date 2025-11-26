use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput,
    VIRTUAL_KEY, VK_0, VK_1, VK_MENU, VK_RETURN,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, IsWindowVisible, SW_RESTORE, SetForegroundWindow, ShowWindow,
};

static mut FOUND_HWND: Option<HWND> = None;

fn main() {
    // 读取配置文件，从配置文件中得到进程名称
    let app_config = read_config();

    // 找到游戏的进程窗口
    let result = find_window_by_keyword(&app_config.wow_title_name);

    match result {
        None => {
            println!(
                "没有找到名为 {} 的窗口, 程序{}秒后自动退出!",
                app_config.wow_title_name, app_config.app_exit_time
            );
            sleep(Duration::from_secs(app_config.app_exit_time));
        }
        Some(hwnd) => {
            if hwnd.0 != 0 {
                // 将游戏窗口放置在前台
                if activate_window(hwnd.0) {
                    println!(
                        "已找到名为 {} 的窗口 {}，将窗口放置在前台！",
                        app_config.wow_title_name, hwnd.0
                    );

                    let mut count = 1;
                    loop {
                        // 返回人物列表
                        return_character_list(&app_config);
                        // 进入游戏
                        enter_wow_game(&app_config);

                        println!("======== 已循环 {} 次 ========", count);
                        count += 1;
                    }
                } else {
                    println!(
                        "窗口放置前台失败, 程序{}秒后自动退出!",
                        app_config.app_exit_time
                    );
                    sleep(Duration::from_secs(app_config.app_exit_time));
                }
            } else {
                println!(
                    "没有找到名为 {} 的窗口, 程序{}秒后自动退出!",
                    app_config.wow_title_name, app_config.app_exit_time
                );
                sleep(Duration::from_secs(app_config.app_exit_time));
            }
        }
    }
}

/// 返回人物列表
fn return_character_list(app_config: &AppConfig) {
    println!("{} 秒后小退...", app_config.in_game_idle_time);
    sleep(Duration::from_secs(app_config.in_game_idle_time));
    unsafe {
        press_alt_0();
    }
    println!("正在小退... 等待{}秒", app_config.small_refund_waiting_time);
    sleep(Duration::from_secs(app_config.small_refund_waiting_time));
}

/// 进入游戏
fn enter_wow_game(app_config: &AppConfig) {
    println!(
        "{} 秒后进入游戏...",
        app_config.character_interface_dwell_time
    );
    sleep(Duration::from_secs(
        app_config.character_interface_dwell_time,
    ));
    unsafe {
        press_enter();
    }
    println!(
        "正在进入游戏... 等待{}秒",
        app_config.entering_game_waiting_time
    );
    sleep(Duration::from_secs(app_config.entering_game_waiting_time));
}

/// 读取配置文件
fn read_config() -> AppConfig {
    let yaml_path = Path::new("config.yaml");
    let yaml_content = fs::read_to_string(yaml_path).expect("没有找到配置文件");
    let config: AppConfig = serde_yaml::from_str(&yaml_content).expect("解析文件错误");

    println!("当前配置文件内容为：{:?}", config);

    config
}

/// 让窗口保持在前台
fn activate_window(hwnd: isize) -> bool {
    unsafe {
        let handle = HWND(hwnd);

        // 如果窗口被最小化，则恢复
        let _ = ShowWindow(handle, SW_RESTORE);

        // 尝试设为前台窗口
        SetForegroundWindow(handle).as_bool()
    }
}

/// 发送回车键
unsafe fn press_enter() {
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

// 按组合键 alt+0
unsafe fn press_alt_0() {
    key_down(VK_MENU); // Alt down
    key_down(VK_0); // 0 down
    key_up(VK_0); // 0 up
    key_up(VK_MENU); // Alt up
}

/// 配置文件映射参数
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct AppConfig {
    wow_title_name: String,              // 游戏窗口标题
    app_exit_time: u64,                  // 防暂离程序退出的时间
    in_game_idle_time: u64,              // 在游戏中的等待时间
    character_interface_dwell_time: u64, // 人物列表等待时间
    small_refund_waiting_time: u64,      // 小退等待时间
    entering_game_waiting_time: u64,     // 进入游戏等待时间
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
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
fn find_window_by_keyword(keyword: &str) -> Option<HWND> {
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
