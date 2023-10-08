pub mod detail;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setting {
    pub agree: bool,
    pub rename_date: bool,
    pub keep_nickname: bool,
    pub custom_name: bool,
    pub custom_name_content: String,
    pub enable_auto_send_images: bool,
    pub auto_send_images: Vec<String>,
    pub auto_send_text: String,
    pub enable_auto_send_text: bool,
    pub agree_delay_min: i32,
    pub agree_delay_max: i32,
    pub send_msg_delay_min: i32,
    pub send_msg_delay_max: i32
}


impl Default for Setting {
    fn default() -> Self {
        Setting{
            agree: true,
            rename_date: false,
            keep_nickname: false,
            custom_name: false,
            custom_name_content: "".to_string(),
            enable_auto_send_images: false,
            auto_send_images: vec![],
            auto_send_text: "".to_string(),
            enable_auto_send_text: false,
            agree_delay_min: 1,
            agree_delay_max: 10,
            send_msg_delay_min: 1,
            send_msg_delay_max: 10
        }
    }
}