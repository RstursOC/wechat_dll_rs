use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Detail{
    pub pid: u32, // 进程PID
    pub account: String, // 账号
    pub nickname: String, // 昵称
    pub status: i32, // 状态：0 离线， 1 在线
    pub add_count: u32, // 申请数
    pub success_count: u32, // 同意数
    pub created_at: String, // 创建时间
    pub updated_at: String // 更新时间
}