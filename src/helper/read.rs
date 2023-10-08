use winapi::shared::minwindef::DWORD;
use crate::helper::addr;
use std::ptr::{read, read_unaligned};

/**
 * 读取微信ID
 */
pub unsafe fn read_wx_id(wechat_win: DWORD) -> String {
    
    let wx_id = (wechat_win + addr::WXID_ADDR_OFFSET) as *mut u8;
    let len = read((wechat_win + addr::WXID_ADDR_OFFSET + addr::USER_INFO_LEN_OFFSET) as *mut u8);
    read_addr_with_len(wx_id, len as usize)
}

/**
 * 读取微信昵称
 */
pub unsafe fn read_wx_name(wechat_win: DWORD) -> String {
    let wx_name = (wechat_win + addr::WXNAME_ADDR_OFFSET) as *mut u8;
    let len = read((wechat_win + addr::WXNAME_ADDR_OFFSET + addr::USER_INFO_LEN_OFFSET) as *mut u8);
    read_addr_with_len(wx_name, len as usize).to_string()
}


/**
 * 读取微信账号
 */
pub unsafe fn read_wx_account(wechat_win: DWORD) -> String {
    let wx_account = (wechat_win + addr::WXACCOUNT_ADDR_OFFSET) as *mut u8;
    let len = read((wechat_win + addr::WXACCOUNT_ADDR_OFFSET + addr::USER_INFO_LEN_OFFSET) as *mut u8);
    read_addr_with_len(wx_account, len as usize).to_string()
}


pub unsafe fn read_wx_account2(wechat_win: DWORD) -> String {
    let wx_account_ptr = (wechat_win + addr::NEW_WXID) as *mut u32;
    // println!("指针：{:?}", wx_account_ptr);
    // println!("指针内容：{:?}", *wx_account_ptr);
    let len = 19;
    let wx_account = (*wx_account_ptr) as *mut u8;
    read_addr_with_len(wx_account, len as usize).to_string()
}

/**
 * 读取指定地址
 */
unsafe fn read_addr(address: *mut u8) -> Box<str> {
    std::str::from_boxed_utf8_unchecked(Box::from(std::slice::from_raw_parts(address, 50))).to_owned()
}

// 读取指定长度的值
pub unsafe fn read_addr_with_len(address: *mut u8, leng: usize) -> String {
    if leng == 0 {
        return "".to_string()
    }
    let slice = std::slice::from_raw_parts(address, leng);
    String::from_utf8_lossy(slice).to_string()
}

// 读取指定长度的值 unicode格式
pub unsafe fn read_addr_with_len_in_unicode(address: *mut u16, leng: usize) -> String {
    let slice = std::slice::from_raw_parts(address, leng);
    String::from_utf16_lossy(slice).to_string()
}