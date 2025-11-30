use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::time::Duration;

/// 配置文件映射参数
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppConfig {
    pub wow_title_name: String,              // 游戏窗口标题
    pub app_exit_time: u64,                  // 防暂离程序退出的时间
    pub in_game_idle_min_time: u64,          // 在游戏中的最小等待时间
    pub in_game_idle_max_time: u64,          // 在游戏中的最大等待时间
    pub character_interface_dwell_time: u64, // 人物列表等待时间
    pub small_refund_waiting_time: u64,      // 小退等待时间
    pub entering_game_waiting_time: u64,     // 进入游戏等待时间
}

/// 读取配置文件
pub fn read_config() -> AppConfig {
    let yaml_path = Path::new("config.yaml");
    let yaml_content = fs::read_to_string(yaml_path).expect("没有找到配置文件");
    let config: AppConfig = serde_yaml::from_str(&yaml_content).expect("解析文件错误");

    println!("当前配置文件内容为：{:?}", config);

    config
}

pub fn check_config(app_config: &AppConfig) {
    while app_config.in_game_idle_min_time > app_config.in_game_idle_max_time {
        println!("in_game_idle_max_time 参数必须大于等于 in_game_idle_min_time 参数，手动更改");
        std::thread::sleep(Duration::from_secs(3));
    }

    while app_config.in_game_idle_min_time < 300 {
        println!("in_game_idle_min_time 参数不能小于300！");
        std::thread::sleep(Duration::from_secs(3));
    }
}
