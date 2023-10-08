use winapi::{shared::minwindef::DWORD, um::libloaderapi::GetModuleHandleW};

use crate::{helper::wide_string, MODULE_NAME};


pub mod check_login;
pub mod multiple_wx;
pub mod msg_receive_asm;
pub mod hook_anywhere;
pub mod msg_sender;
pub mod edit_remaker;
pub mod read_info_by_wxid;
pub mod image_sender;

/**
 * 获取基础地址
 */
pub unsafe fn get_wechatwin_addr() -> DWORD{
    GetModuleHandleW(wide_string(MODULE_NAME).as_ptr()) as DWORD
}