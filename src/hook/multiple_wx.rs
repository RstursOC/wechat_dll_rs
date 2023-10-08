use detour::static_detour;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::ffi::CString;
use std::{mem, iter};
use std::ptr::null_mut;
use std::fmt::format;
use std::iter::{once};
use winapi::shared::ntdef::LPCWSTR;
use winapi::um::psapi::GetModuleInformation;
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::winnt::{HANDLE, WCHAR};

use winapi::shared::minwindef::{
    BOOL
};

use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleW};

use crate::helper::{wide_string, show_message_box};

// 多开HOOK
static_detour! {
    static CreateMutexWHook: unsafe extern "system" fn(LPSECURITY_ATTRIBUTES, BOOL, LPCWSTR) -> HANDLE;
}

type CMuteHook = unsafe extern "system" fn(LPSECURITY_ATTRIBUTES, BOOL, LPCWSTR) -> HANDLE;


pub unsafe fn run() {
    // 多开HOOK
    let mutex_address = get_module_symbol_address("Kernel32.dll", "CreateMutexW").unwrap();
    let target_mutex_hook: CMuteHook = mem::transmute(mutex_address);
    // CreateMutexWHook.initialize(target_mutex_hook, CreateMutexW_detour).unwrap().enable().unwrap();
    let init_mutex_hook =  match CreateMutexWHook.initialize(target_mutex_hook, CreateMutexW_detour) {
        Err(e) => {
            show_message_box("错误提示", e.to_string().as_str());
            panic!("Problem opening the file: {:?}", e)
        },
        Ok(s) => s
    };

    match init_mutex_hook.enable(){
        Ok(_) => {}
        Err(e) => {
            show_message_box("错误提示", e.to_string().as_str());
            panic!("Problem opening the file: {:?}", e)
        }
    }

}

fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    let module = module
        .encode_utf16()
        .chain(iter::once(0))
        .collect::<Vec<u16>>();
    let symbol = CString::new(symbol).unwrap();
    unsafe {
        let handle = GetModuleHandleW(module.as_ptr());
        match GetProcAddress(handle, symbol.as_ptr()) as usize {
            0 => None,
            n => Some(n),
        }
    }
}

fn CreateMutexW_detour(lp:LPSECURITY_ATTRIBUTES, bool: BOOL, lp2: LPCWSTR ) -> HANDLE{

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let name = wide_string(rand_string.as_str()).as_ptr();

    unsafe {
        CreateMutexWHook.call(lp, bool, name)
    }

}