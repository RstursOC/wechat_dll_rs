use std::arch::asm;

use log::info;
use serde::{Serialize, Deserialize};
use winapi::shared::minwindef::DWORD;
use std::ptr::read_unaligned;

use crate::{helper::{WxStr, addr, read::read_addr_with_len_in_unicode}, hook::get_wechatwin_addr};

/**
 * 通过wxid读取个人信息
*/

static mut EAX_PTR: DWORD = 0;
static mut EDI_PTR: DWORD = 0;
static mut ECX_PTR: DWORD = 0;

static mut CALL1_PTR: DWORD = 0;
static mut CALL2_PTR: DWORD = 0;
static mut CALL3_PTR: DWORD = 0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Info {
    pub wxid: String,
    pub account: String,
    // pub v3: String,
    // pub remaker: String,
    pub nickname: String
}

pub unsafe fn read_by_wxid(wxid: String) -> Info{

    let wechat_win = get_wechatwin_addr();

    let edi: [DWORD; 0x3E8] = [0; 0x3E8];
    EDI_PTR = edi.as_ptr() as u32;

    let eax: [DWORD; 0x130] = [0; 0x130];
    EAX_PTR = eax.as_ptr() as u32;

    let (ecx, wstr) = WxStr::new_without_chinese(wxid);
    ECX_PTR = ecx.get_ptr();

    CALL1_PTR = wechat_win.clone() + addr::GET_INFO_BY_WXID_CALL1;
    CALL2_PTR = wechat_win.clone() + addr::GET_INFO_BY_WXID_CALL2;
    CALL3_PTR = wechat_win.clone() + addr::GET_INFO_BY_WXID_CALL3;


    asm!{
        "pushad",
        "pushfd",
        "nop",
        "nop",
        "nop",
        "mov edi, dword ptr ds:[{}]",
        "mov eax, dword ptr ds:[{}]",
        "push eax",
        "mov ecx, edi",
        "call dword ptr ds:[{}]",
        "call dword ptr ds:[{}]",
        "push edi",
        "mov ecx, dword ptr ds:[{}]",
        "push ecx",
        "mov ecx, eax",
        "call dword ptr ds:[{}]",
        "popfd",
        "popad",
        sym EDI_PTR,
        sym EAX_PTR,
        sym CALL1_PTR,
        sym CALL2_PTR,
        sym ECX_PTR,
        sym CALL3_PTR
    };
    // 读取信息
    let wxid_addr = EDI_PTR + addr::INFO_WXID_OFFSET;
    let wxid_len = EDI_PTR + addr::INFO_WXID_LENGTH_OFFSET;

    let account_addr = EDI_PTR + addr::INFO_ACCOUNT_OFFSET;
    let account_len = EDI_PTR + addr::INFO_ACCOUNT_LENGTH_OFFSET;

    let nickname_addr = EDI_PTR + addr::INFO_NICKNAME_OFFSET;
    let nickname_len = EDI_PTR + addr::INFO_NICKNAME_LENGTH_OFFSET;

    
    let info = Info {
        wxid: read_item(wxid_addr, wxid_len),
        account: read_item(account_addr, account_len),
        // v3: read_item(v3_addr, v3_len),
        // remaker: read_item(remaker_addr, remaker_len),
        nickname: read_item(nickname_addr, nickname_len),
        // avatar: read_item(avatar_addr, avatar_len)
    };

    info!("读取的个人信息：{:?}", info);

    // println!("数组列表：{:?}", edi);

    info
}

/**
 * 根据地址读取内容
 */
pub unsafe fn read_item(content_addr: u32, len_addr: u32) -> String{
    // 偏移地址写错了，这里要减去0x4
    let content_addr = content_addr - 0x4;
    let len_addr = len_addr - 0x4;

    let content_addr_ptr = read_unaligned(content_addr as *const u32);
    let len = read_unaligned(len_addr as *const u32);

    // info!("读取的地址，内容：{}, 长度：{}", content_addr_ptr, len);

    read_addr_with_len_in_unicode(content_addr_ptr as *mut u16, len as usize)
}