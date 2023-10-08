#![feature(naked_functions)]
#![feature(asm_sym)]
#![feature(asm_const)]

extern crate core;
extern crate kernel32;

mod data;
mod hook;

use base64::Engine;
use helper::time::now;
use hook::check_login;

mod helper;

use log::error;
use log::info;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::Rng;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};

use winapi;
use winapi::um::libloaderapi::DisableThreadLibraryCalls;

use std::error::Error;
use std::sync::Mutex;
use std::{thread, time};

use std::str;
use std::string::ToString;
use std::thread::sleep;

use serde::{Deserialize, Serialize};

use crate::data::detail::Detail;
use crate::data::Setting;
use crate::hook::read_info_by_wxid::read_by_wxid;
use winapi::um::processthreadsapi::GetCurrentProcessId;
use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

use crate::helper::read::read_wx_account2;
use crate::helper::read::read_wx_id;
use crate::hook::{get_wechatwin_addr, read_info_by_wxid};
use crate::hook::msg_receive_asm;

use std::io::Write;
use serde_json::json;
use crate::hook::edit_remaker::edit_remaker;
use crate::hook::image_sender::send_image;
use crate::hook::msg_sender::send_text;

#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub mode: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AllData {
    pub setting: Setting,
    pub detail: Detail,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    wxid: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestWxIdData {
    wxid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestDataArr {
    wxid: String,
    content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EncryptData {
    password: String,
    base64: String,
    time: String,
    verify: String,
    length: String,
}

pub enum Signal {
    Ping,
    Login,
}

// 数据存储
pub static GLOBAL_DATA: Lazy<Mutex<AllData>> = Lazy::new(|| {
    let data = AllData {
        setting: Setting::default(),
        detail: Detail {
            pid: 0,
            account: "".to_string(),
            nickname: "".to_string(),
            status: 1,
            add_count: 0,
            success_count: 0,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        },
    };

    Mutex::new(data)
});

const MODULE_NAME: &str = "WeChatWin.dll";

#[no_mangle]
unsafe extern "system" fn DllMain(hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    // kernel32::AllocConsole();

    match reason {
        DLL_PROCESS_DETACH => {
            println!("Remove from main process");
        }
        DLL_PROCESS_ATTACH => unsafe {
            DisableThreadLibraryCalls(hinst);
            // 打开一个控制台窗口，输出调试信息
            #[cfg(debug_assertions)]
            kernel32::AllocConsole();

            thread::spawn(|| run());
        },
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => {}
    }

    TRUE
}

unsafe fn run() {
    #[cfg(debug_assertions)]
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Initializing...");
    // 多开HOOK
    hook::multiple_wx::run();

    // 获取process id
    let pid = GetCurrentProcessId();

    // 获取DLL地址

    let mut wechat_win: DWORD = 0;

    loop {
        if wechat_win != 0 {
            break;
        }
        wechat_win = get_wechatwin_addr();
        sleep(time::Duration::from_millis(500))
    }

    info!("开始处理消息");

    msg_receive_asm::start_hook_msg_fn();

    // 大循环，防止微信退出登录
    loop {
        // 检测是否登录，登录成功后，开启socket
        loop {
            if check_login::is_login(wechat_win) {
                // 已登录
                info!("登录成功");
                // show_message_box("Hello World", "登录成功");

                // 更新信息
                let mut data = match GLOBAL_DATA.lock() {
                    Ok(data) => data,
                    Err(err) => {
                        error!("{}", err.to_string());
                        continue;
                    }
                };
                // 新方法，获取wxid
                data.detail.account = read_wx_account2(wechat_win);

                let user_info = read_by_wxid(data.detail.account.clone());
                info!("{:?}", user_info);

                if user_info.wxid.is_empty() {
                    sleep(time::Duration::from_millis(1000));
                    continue;
                }

                data.detail.nickname = user_info.nickname;
                // pid 更新
                data.detail.pid = pid;
                // 更新创建时间
                data.detail.created_at = now();

                info!(
                    "昵称：{}, 账号：{}, ID：{}",
                    &data.detail.nickname,
                    &data.detail.account,
                    read_wx_id(wechat_win)
                );

                break;
            }
            sleep(time::Duration::from_millis(500))
        }

        info!("开始连接本地服务器");
        loop {
            // TODO: 这里实现自己的消息发送和指令接收功能，例如使用TCP，Websocket与控制端交互

            // 这里假装从控制端收到一条指令
            let command = Msg {
                data: "指令内容".to_string(),
                mode: "从控制端接收的指令".to_string(),
            };

            match command.mode.as_str() {
                "agree" => {
                    // 同步设置
                    match GLOBAL_DATA.lock() {
                        Ok(mut g_data) => {
                            g_data.setting = match serde_json::from_str(&command.data) {
                                Ok(setting) => setting,
                                Err(err) => {
                                    error!("{}", err.to_string());
                                    return;
                                }
                            };
                        }
                        Err(err) => {
                            error!("{}", err.to_string());
                        }
                    };
                }
                "test_text" => {
                    // 测试发送文本消息
                    let data: TestData = serde_json::from_str(&command.data).unwrap();
                    send_text(data.wxid, data.content);
                }
                "test_image" => {
                    // 测试发送图片消息
                    let data: TestDataArr =
                        serde_json::from_str(&command.data).unwrap();
                    let wxid = data.wxid;
                    for img in data.content {
                        send_image(wxid.clone(), img.clone());
                    }
                }
                "test_edit_remark" => {
                    // 测试修改备注
                    let data: TestData = serde_json::from_str(&command.data).unwrap();
                    edit_remaker(data.content, data.wxid);
                }
                "test_get_info" => {
                    // 测试读取信息
                    let data: TestWxIdData =
                        serde_json::from_str(&command.data).unwrap();
                    let info = read_info_by_wxid::read_by_wxid(data.wxid);
                    info!("{:?}", info);
                }
                "reset" => {
                    // 重置计数
                    let mut data = GLOBAL_DATA.lock().unwrap();
                    if command.data == data.detail.account {
                        data.detail.add_count = 0;
                        data.detail.success_count = 0;
                    }
                }
                "get_info" => {
                    // 获取某人信息
                    let wx_id = read_wx_account2(wechat_win);
                    if command.data == wx_id {
                        match GLOBAL_DATA.lock() {
                            Ok(mut data) => {
                                let user_info = read_by_wxid(wx_id.clone());
                                data.detail.account = wx_id.clone();
                                data.detail.nickname = user_info.nickname;
                            }
                            Err(_) => (),
                        };
                    }
                }
                _ => {
                    error!("无法识别的指令")
                }
            }

            if check_login::is_login(wechat_win) {
                let ten_millis = time::Duration::from_millis(3000);
                thread::sleep(ten_millis);
                info!("尝试重新连接");
            } else {
                info!("检测到微信退出登录，重新检测登录状态");
                break;
            }
        }
    }
}

// 生成随机字符串
fn rand_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
