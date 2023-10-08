// 发送消息助手
use std::{arch::asm};

use log::info;
use winapi::shared::minwindef::{DWORD, FALSE, BOOL};

use crate::{helper::{addr, wide_string, WxStr}, hook::get_wechatwin_addr};

static mut send_call: DWORD = 0;
static mut clear_cache_call: DWORD = 0;

pub unsafe fn send_text(wx_id: String, content: String) -> bool {
    let wechat_win = get_wechatwin_addr();

    let (mut wx_msg, wstr) = WxStr::new_with_chinese(content);
    let wx_msg_c = wx_msg.get_raw_ptr();

    let (mut wx_id, wstr2) = WxStr::new_without_chinese(wx_id);
    let wx_id_c = wx_id.get_raw_ptr();

    let buff = WxStr::new_null();
    let buff_c = buff.get_raw_ptr();

    let buff_2: [DWORD; 0x3A8] = [0; 0x3A8];
    let buff_2_c = &buff_2 as *const _ as u32;

    send_call =  wechat_win.clone() + addr::SEND_MSG_CALL;
    clear_cache_call = wechat_win.clone() + addr::CLEAR_MSG_CACHE_CALL;

    let mut tmp: DWORD;

    asm!(
        "pushad",
        "push 0x0",
        // "mov eax, {buff}",
        "push 0x0",
        "push 0x1",
        "push eax",
        // "mov edi, {msg}",
        "push edi",
        // "edx",
        // "mov {tmp}, {buff_2_c:e}",
        "mov {tmp:e}, ecx",
        "call dword ptr ds:[{send_call}]",
        "add esp, 0x14",
        send_call = sym send_call,
        // buff_2_c = inout(reg) buff_2_c,
        tmp = out(reg) tmp,
        in("eax") buff_c,
        in("edi") wx_msg_c,
        in("edx") wx_id_c,
        in("ecx") buff_2_c,
    );

    asm!(
        "mov ecx, {tmp:e}",
        "call dword ptr ds:[{clear_cache_call}]",
        "popad",
        clear_cache_call = sym clear_cache_call,
        tmp = in(reg) tmp,
    );

    // info!("发送的内容:{:?}, {:?}", wstr, wstr2);

    true
}