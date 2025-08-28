use log::info;
use std::default::Default;

pub fn init_logger() {
    // 根据环境变量初始化日志
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("日志初始化完成");
}
