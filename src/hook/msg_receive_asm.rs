use core::time;
use log::{error, info};
use rand::Rng;
use regex::Regex;
use std::{
    arch::asm,
    ptr::{read, read_unaligned},
    thread::{self, sleep},
};

use winapi::{
    ctypes::c_void,
    shared::minwindef::{DWORD, TRUE},
};

use crate::{
    helper::{
        addr::{
            self, MSG_TYPE_EMOJI, MSG_TYPE_FRIEND_REQUEST, MSG_TYPE_FRIEND_REQUEST_SUCCESS,
            MSG_TYPE_TEXT,
        },
        read::{read_addr_with_len, read_addr_with_len_in_unicode},
        time::{now, now_without_year},
    },
    hook::{
        edit_remaker::{self, edit_remaker},
        image_sender::send_image,
        msg_sender::send_text,
        read_info_by_wxid::read_by_wxid,
    },
    GLOBAL_DATA,
};

use super::{get_wechatwin_addr, hook_anywhere::hook_any_address};
use crate::helper::agree_friend::agree;

static mut RET_MSG: u32 = 0x0;
static mut COVER_MSG: u32 = 0x0;

static mut wechat_win_: DWORD = 0;

pub unsafe fn start_hook_msg_fn() {
    let wechat_win = get_wechatwin_addr();

    RET_MSG = wechat_win + addr::MSG_RECEIVE_USE_OFFSET + 5;
    COVER_MSG = wechat_win + addr::MSG_RECEIVE_OFFSET;

    wechat_win_ = wechat_win;

    // println!("被覆盖的call：{}, 跳转回的地址：{}", COVER_MSG, RET_MSG);

    hook_any_address(
        wechat_win + addr::MSG_RECEIVE_USE_OFFSET,
        asm_recieve_wx_msg as u32,
    );
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn asm_recieve_wx_msg() {
    asm!(
        "pushad",
        "pushfd",
        "push eax",
        "call {}",
        "add esp,0x4",
        "popfd",
        "popad",
        "call dword ptr ds:[{}]",
        "jmp dword ptr ds:[{}]",
        sym msg_hook,
        sym COVER_MSG,
        sym RET_MSG,
        options(noreturn)
    );
}

unsafe extern "C" fn msg_hook(eax: DWORD) {
    let eax_ptr = eax as *const u32;

    if eax_ptr.is_null() {
        return;
    }

    // println!("接收消息");
    let msg_ptr: u32 = read(eax_ptr);

    // 等于0就不处理
    if msg_ptr == 0 {
        return;
    }

    let msg_type = (msg_ptr + addr::MSG_TYPE_OFFSET) as *const c_void;
    let len_prt = (msg_ptr + addr::MSG_CONTENT_LEN_OFFSET) as *const c_void;
    let content_prt = (msg_ptr + addr::MSG_CONTENT_OFFSET) as *const c_void;
    let sender_prt = (msg_ptr + addr::MSG_SENDER_ID_OFFSET) as *const c_void;
    let sender_len_prt = (msg_ptr + addr::MSG_SENDER_ID_LEN_OFFSET) as *const c_void;


    if msg_type.is_null()
        || len_prt.is_null()
        || content_prt.is_null()
        || sender_len_prt.is_null()
        || sender_prt.is_null()
    {
        // 有无效指针，放弃读取
        return;
    }

    let sender_ptr_temp = read(sender_prt as *const u32) as *const c_void;
    let content_prt_temp = read(content_prt as *const u32) as *const c_void;

    if sender_ptr_temp.is_null() || content_prt_temp.is_null() {
        return;
    }

    let msg_type_val = read(msg_type as *const u32);
    info!("消息类型：{}", msg_type_val);

    match msg_type_val {
        MSG_TYPE_TEXT => {
            // println!("收到文本消息");
            let (sender, content) =
                read_content(len_prt, sender_len_prt, content_prt_temp, sender_ptr_temp);
        }
        MSG_TYPE_FRIEND_REQUEST => {
            // println!("收到好友请求");

            // 是否开启了自动同意
            let run;

            {
                let mut global_data = match GLOBAL_DATA.lock() {
                    Ok(global_data) => global_data,
                    Err(err) => {
                        error!("{}", err.to_string());
                        run = 0;
                        return;
                    }
                };

                global_data.detail.add_count += 1;
                // 更新时间
                global_data.detail.updated_at = now();

                if !global_data.setting.agree {
                    run = 0;
                    return;
                }

                run = 1;
            };

            if run == 0 {
                // 没有开启自动同步
                return;
            }

            // 正则匹配V3：(?<=encryptusername=")\w+@stranger
            // 正则匹配V4：(?<=ticket=")\w+@stranger
            let (sender, content) =
                read_content(len_prt, sender_len_prt, content_prt_temp, sender_ptr_temp);

            // 提取
            let reg_v3 = match Regex::new("v3_\\w+@stranger") {
                Ok(reg_v3) => reg_v3,
                Err(err) => {
                    error!("{}", err.to_string());
                    return;
                }
            };
            let v3: String = match reg_v3.captures(&content) {
                Some(v3) => match v3.get(0) {
                    Some(v33) => v33.as_str().to_string(),
                    None => {
                        error!("V3 token not found: 2");
                        return;
                    }
                },
                None => {
                    error!("V3 token not found");
                    return;
                }
            };
            // info!("V3: {}", v3);

            let reg_v4 = match Regex::new("v4_\\w+@stranger") {
                Ok(reg_v4) => reg_v4,
                Err(err) => {
                    error!("{}", err.to_string());
                    return;
                }
            };
            let v4: String = match reg_v4.captures(&content) {
                Some(v4) => match v4.get(0) {
                    Some(v44) => v44.as_str().to_string(),
                    None => {
                        error!("V4 token not found: 2");
                        return;
                    }
                },
                None => {
                    error!("V4 token not found");
                    return;
                }
            };
            // info!("V4: {}", v4);

            // 打开一个线程来执行
            thread::spawn(move || {
                let global_data = match GLOBAL_DATA.lock() {
                    Ok(global_data) => global_data,
                    Err(err) => {
                        error!("{}", err.to_string());
                        return;
                    }
                };

                // 休眠N秒，才开始执行
                let min = global_data.setting.agree_delay_min;
                let max = global_data.setting.agree_delay_max;

                drop(global_data);

                if min == 0 && max == 0 {
                    // 忽略
                } else if min == max {
                    sleep(time::Duration::from_millis(min as u64 * 1000));
                } else {
                    let num = rand::thread_rng().gen_range(min..=max) as u64;
                    sleep(time::Duration::from_millis(num * 1000));
                }

                // 循环同意
                let max_times = 10;
                let gap = 10;
                let mut cur_time = 0;
                loop {
                    // 执行同意请求
                    let res = agree(wechat_win_, v3.clone(), v4.clone());

                    // info!("加好友结果：{}", res);

                    if res {
                        let mut global_data = match GLOBAL_DATA.lock() {
                            Ok(global_data) => global_data,
                            Err(err) => {
                                error!("{}", err.to_string());
                                return;
                            }
                        };
                        // 更新计数器
                        global_data.detail.success_count += 1;
                        // 主动释放
                        drop(global_data);
                        // info!("更新后的数据：{:?}", &global_data);

                        break;
                    } else {
                        // send_error("自动同意好友请求验证失败".to_string());
                        // 自动隔断时间重试
                        cur_time += 1;
                        if cur_time > max_times {
                            break;       
                        }
                        sleep(time::Duration::from_secs(cur_time * gap));
                    }
                }

            });
        }
        MSG_TYPE_FRIEND_REQUEST_SUCCESS => {
            info!("收到成功加好友打招呼消息");

            let (sender, content) =
                read_content(len_prt, sender_len_prt, content_prt_temp, sender_ptr_temp);

            // 忽略不包含开始聊天的消息
            if !content.contains("开始聊天") {
                return;
            }

            // 获取个人信息
            let info = read_by_wxid(sender.clone());

            let info2 = info.clone();
            // 发送到服务器
            thread::spawn(move || {
                let msg = match serde_json::to_string(&info2) {
                    Ok(msg) => msg,
                    Err(err) => {
                        error!("Fail Log: {}", err.to_string());
                        return;
                    }
                };
            });

            // 自动发送消息
            let global_data = match GLOBAL_DATA.lock() {
                Ok(global_data) => global_data.clone(),
                Err(err) => {
                    error!("{}", err.to_string());
                    return;
                }
            };

            // 备注列表
            let mut remark_arr: Vec<String> = Vec::new();
            // 修改备注
            let mut remark_date = String::from("");
            if global_data.setting.rename_date {
                remark_date = now_without_year();
            }
            if !remark_date.is_empty() {
                remark_arr.push(remark_date);
            }

            let mut remark_custom = String::from("");
            if global_data.setting.custom_name {
                remark_custom = global_data.setting.custom_name_content.clone();
            }
            if !remark_custom.is_empty() {
                remark_arr.push(remark_custom);
            }

            let mut remark_keep = String::from("");
            if global_data.setting.keep_nickname {
                remark_keep = info.nickname;
            }
            if !remark_keep.is_empty() {
                remark_arr.push(remark_keep);
            }

            // 最后组装
            if !remark_arr.is_empty() {
                let new_remark = remark_arr.join("_");
                // 如果长度大于0，证明启用了修改昵称
                if !new_remark.is_empty() {
                    // 修改
                    edit_remaker(new_remark, sender.clone());
                }
            }

            // 自动发送文本消息
            if global_data.setting.enable_auto_send_text
                || global_data.setting.enable_auto_send_images
            {
                let sender = sender;
                thread::spawn(move || {
                    let global_data = match GLOBAL_DATA.lock() {
                        Ok(global_data) => global_data.clone(),
                        Err(err) => {
                            error!("{}", err.to_string());
                            return;
                        }
                    };

                    // 休眠N秒，才开始执行
                    let min = global_data.setting.send_msg_delay_min;
                    let max = global_data.setting.send_msg_delay_max;

                    drop(global_data);

                    if min == 0 && max == 0 {
                        // 忽略
                    } else if min == max {
                        sleep(time::Duration::from_millis(min as u64 * 1000));
                    } else {
                        let num = rand::thread_rng().gen_range(min..=max) as u64;
                        sleep(time::Duration::from_millis(num * 1000));
                    }

                    let global_data = match GLOBAL_DATA.lock() {
                        Ok(global_data) => global_data.clone(),
                        Err(err) => {
                            error!("{}", err.to_string());
                            return;
                        }
                    };

                    if global_data.setting.enable_auto_send_text {
                        send_text(sender.clone(), global_data.setting.auto_send_text.clone());
                    }

                    // 自动发图片
                    if global_data.setting.enable_auto_send_images
                        && !global_data.setting.auto_send_images.is_empty()
                    {
                        for path in &global_data.setting.auto_send_images {
                            send_image(sender.clone(), path.to_string());
                        }
                    }

                    drop(global_data);
                });
            }
        }
        _ => {
            // println!("未知消息类型")
        }
    }
}

// 无意义函数
unsafe extern "C" fn do_nothing() {}

/**
 * 读取内容
 */
unsafe fn read_content(
    len_prt: *const c_void,
    sender_len_prt: *const c_void,
    content_prt_temp: *const c_void,
    sender_ptr_temp: *const c_void,
) -> (String, String) {
    let len_content = read(len_prt as *const u32) as usize;
    let len_sender = read(sender_len_prt as *const u32) as usize;

    if len_content != 0 && len_sender != 0 {
        // 读取内容
        let content = read_addr_with_len_in_unicode(content_prt_temp as *mut u16, len_content);
        let sender = read_addr_with_len_in_unicode(sender_ptr_temp as *mut u16, len_sender);

        info!("消息内容：{}，发送者：{}", content, sender);

        (sender, content)
    } else {
        ("".to_string(), "".to_string())
    }
}
