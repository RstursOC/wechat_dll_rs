use chrono::{DateTime, Local};

// 获取当前格式化的时间
pub fn now() -> String{
    let local: DateTime<Local> = Local::now();
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn now_without_year() -> String{
    let local: DateTime<Local> = Local::now();
    local.format("%m-%d %H:%M:%S").to_string()
}
