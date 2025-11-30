mod config;
mod game;
mod keyboard;
mod window;

use rand::Rng;
use std::time::Duration;
use windows::Win32::Foundation::HWND;

static mut FOUND_HWND: Option<HWND> = None;

fn main() {
    // 读取配置文件，从配置文件中得到进程名称
    let app_config = config::read_config();

    config::check_config(&app_config);

    // 找到游戏的进程窗口
    let result = window::find_window_by_keyword(&app_config.wow_title_name);

    match result {
        None => {
            println!(
                "没有找到名为 {} 的窗口, 程序{}秒后自动退出!",
                app_config.wow_title_name, app_config.app_exit_time
            );
            std::thread::sleep(Duration::from_secs(app_config.app_exit_time));
        }
        Some(hwnd) => {
            if hwnd.0 != 0 {
                // 将游戏窗口放置在前台
                if window::activate_window(hwnd.0) {
                    println!(
                        "已找到名为 {} 的窗口 {}，将窗口放置在前台！",
                        app_config.wow_title_name, hwnd.0
                    );

                    let mut count = 1;
                    let mut rng = rand::rng();
                    loop {
                        // 通过随机数，计算游戏在线时间
                        let in_game_idle_time = rng.random_range(
                            app_config.in_game_idle_min_time..=app_config.in_game_idle_max_time,
                        );

                        println!("{} 秒后小退...", in_game_idle_time);
                        // 在游戏里以随机做一个动作
                        game::do_action(&in_game_idle_time,hwnd.0);
                        // game::synthesis_and_decomposition();

                        // 返回人物列表
                        game::return_character_list(&app_config);
                        // 进入游戏
                        game::enter_wow_game(&app_config);

                        println!("======== 已循环 {} 次 ========", count);
                        count += 1;
                    }
                } else {
                    println!(
                        "窗口放置前台失败, 程序{}秒后自动退出!",
                        app_config.app_exit_time
                    );
                    std::thread::sleep(Duration::from_secs(app_config.app_exit_time));
                }
            } else {
                println!(
                    "没有找到名为 {} 的窗口, 程序{}秒后自动退出!",
                    app_config.wow_title_name, app_config.app_exit_time
                );
                std::thread::sleep(Duration::from_secs(app_config.app_exit_time));
            }
        }
    }
}
