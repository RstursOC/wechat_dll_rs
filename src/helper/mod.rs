pub mod read;
pub mod addr;
pub mod agree_friend;
pub mod time;

use log::info;
use winapi::{um::winuser::{
    MessageBoxW, MB_OK
}, shared::minwindef::DWORD};

use std::{ffi::{OsStr}, ptr::null_mut, os::windows::prelude::OsStrExt, arch::asm};
use widestring::U16CString;
/**
 * 微信通用结构体
 */
#[repr(C)]
pub struct WxStr{
    // 字符串指针
    pub content: DWORD,
    // 长度
    pub length: usize,
    // 最长长度
    pub max_length: usize,
    pub buff: DWORD,
    pub buff2: DWORD
}

impl WxStr {
    pub unsafe fn new_without_chinese(content: String) -> (WxStr, Vec<u16>){
        let (wx_id_ptr, len, wstr) = wide_string_c(&content);

        (WxStr {
            content: wx_id_ptr,
            length: len,
            max_length: len * 2,
            buff: 0,
            buff2: 0
        }, wstr)
    }

    pub unsafe fn new_with_chinese(content: String) -> (WxStr, Vec<u16>){
        let (wx_msg_ptr, len, wstr) = wide_string_c(&content);

        (WxStr {
            content: wx_msg_ptr,
            length: len,
            max_length: len  * 2,
            buff: 0,
            buff2: 0
        }, wstr)
    }

    pub unsafe fn new_null() -> WxStr {
        // let null_ptr = wide_string_c("");

        WxStr {
            content: 0,
            length: 0x0,
            max_length: 0x0,
            buff: 0,
            buff2: 0
        }
    }

    pub fn get_ptr(&self) -> u32{
        self as *const _ as u32
    }

    pub fn get_raw_ptr(&self) -> *const u16 {
        self as *const _ as *const u16
    } 
}

pub fn show_message_box(caption: &str, text: &str){
    unsafe {
        MessageBoxW(
            null_mut() as _,
            wide_string(text).as_ptr() as _,
            wide_string(caption).as_ptr() as _,
            MB_OK
        );
    }
}

pub fn wide_string(s: &str) -> Vec<u16>{
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
    // OsStr::new(s).encode_wide().collect()
}

pub unsafe fn wide_string_c(s: &str) -> (DWORD, usize, Vec<u16>){
    let wstr = U16CString::from_str_unchecked(s);
    let len = wstr.len();

    let wstr_vec = wstr.as_slice_with_nul().to_vec();
    
    let addr = wstr_vec.as_ptr() as DWORD;

    (addr, len, wstr_vec)
}