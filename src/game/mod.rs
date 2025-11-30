use crate::{config, game, keyboard, window};
use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 返回人物列表
pub fn return_character_list(app_config: &config::AppConfig) {
    unsafe {
        // 按下空格，解除插件屏保状态
        keyboard::press_white_space();
        // 小退
        keyboard::press_alt_0();
    }
    println!("正在小退... 等待{}秒", app_config.small_refund_waiting_time);
    sleep(Duration::from_secs(app_config.small_refund_waiting_time));
}

/// 合成分解强效不灭精华
pub fn synthesis_and_decomposition() {
    unsafe {
        keyboard::press_alt_9();
    }
}

/// 进入游戏
pub fn enter_wow_game(app_config: &config::AppConfig) {
    println!(
        "{} 秒后进入游戏...",
        app_config.character_interface_dwell_time
    );
    sleep(Duration::from_secs(
        app_config.character_interface_dwell_time,
    ));
    unsafe {
        keyboard::press_enter();
    }
    println!(
        "正在进入游戏... 等待{}秒",
        app_config.entering_game_waiting_time
    );
    sleep(Duration::from_secs(app_config.entering_game_waiting_time));
}

/// 要执行的操作
pub fn do_action(in_game_time: &u64, hwnd0: isize) {
    let mut current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("获取系统时间失败")
        .as_secs();
    // 完成时间
    let done_time = current_time + in_game_time;
    let mut jump_count = 0;

    let mut rng = rand::rng();

    while current_time < done_time {
        let _ = window::activate_window(hwnd0);

        jump_count += 1;

        let action = jump_count % 3;

        if action == 0 {
            // 向前跳
            unsafe {
                keyboard::jump_forward();
            }
        } else if action == 1 {
            // 向后跳
            unsafe {
                keyboard::jump_backward();
            }
        } else if action == 2 {
            // 合成/分解
            unsafe {
                keyboard::press_alt_9();
            }
        }

        // 随机睡3-10
        let sleep_sec = rng.random_range(3..=10);
        sleep(Duration::from_secs(sleep_sec));
        current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("获取系统时间失败")
            .as_secs();
    }
}
