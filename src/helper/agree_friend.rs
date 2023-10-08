use std::arch::asm;

use winapi::shared::minwindef::{DWORD, FALSE, BOOL};

use super::{wide_string, addr};

#[repr(C)]
struct VerifyFriendApplyParamStruct{
    handle: DWORD,
    StatusCode: *const DWORD,
    StatusCodeEndAddr1: DWORD,
    StatusCodeEndAddr2: DWORD,
    buffer: [DWORD; 0x3C]
}

static mut verify_friend_apply_call1: DWORD = 0x0;
static mut verify_friend_apply_call2: DWORD = 0x0;
static mut verify_friend_apply_param: DWORD = 0x0;

static mut w_v3_ptr: DWORD = 0x0;
static mut w_v4_ptr: DWORD = 0x0;

static mut nullbuffer_ptr: DWORD = 0x0;

static mut param_ptr2: DWORD = 0x0;

static mut success: BOOL = FALSE;

pub unsafe fn agree(wechat_win: DWORD, v3: String, v4: String) -> bool{
    
    let w_v3 = wide_string(&v3);
    let w_v4 = wide_string(&v4);

    // 基本消息结构体
    let w_v3_struct = wx_str{
        buffer: w_v3.as_ptr() as u32,
        length: w_v3.len() - 1,
        max_length: w_v3.len() - 1,
        fill1: 0,
        fill2: 0
    };

    let w_v4_struct = wx_str{
        buffer: w_v4.as_ptr() as u32,
        length: w_v4.len()  - 1,
        max_length: w_v4.len() - 1,
        fill1: 0,
        fill2: 0
    };

    w_v3_ptr = &w_v3_struct as *const _ as u32;
    w_v4_ptr = &w_v4_struct as *const _ as u32;

    verify_friend_apply_call1 = wechat_win + addr::ADD_FRIEND_CALL1;
    verify_friend_apply_call2 = wechat_win + addr::ADD_FRIEND_CALL2;
    verify_friend_apply_param = wechat_win + addr::ADD_FROEND_PARAM_OFFSET;

    let mut param = VerifyFriendApplyParamStruct{
        handle: verify_friend_apply_param,
        StatusCode: 0x0 as *const u32,
        StatusCodeEndAddr1: 0,
        StatusCodeEndAddr2: 0,
        buffer: [0; 0x3C]
    };

    let param_ptr = &param as *const _ as u32;

    let status_code: [DWORD; 9] = [
        0xB2, param_ptr, 0xB5, param_ptr, 0xB0, param_ptr, 0xB1, param_ptr, 0x0
    ];

    param.StatusCode = status_code.as_ptr();
    param.StatusCodeEndAddr1 = status_code[8];
    param.StatusCodeEndAddr2 = status_code[8];

    param_ptr2 = &param as *const _ as u32;

    let nullbuffer = vec![b""; 0x94];
    nullbuffer_ptr = nullbuffer.as_ptr() as DWORD;

    // 汇编
    asm!{
        "pushad",
        "pushfd",
        "push 0x0",
        "push 0x6",
        "sub esp, 0x14",
        "mov ecx, esp",
        "push [{}]",
        "call dword ptr ds:[{}]",
        "sub esp, 0x8",
        "push 0x0",
        "push [{}]",
        "mov eax, dword ptr ds:[{}]",
        "push eax",
        "mov ecx, [{}]",
        "call dword ptr ds:[{}]",
        "mov {}, eax",
        "popfd",
        "popad",
        sym w_v4_ptr,
        sym verify_friend_apply_call1,
        sym nullbuffer_ptr,
        sym w_v3_ptr,
        sym param_ptr2,
        sym verify_friend_apply_call2,
        sym success
    };

    // println!("结果：{}", success);

    success == 1
}


struct wx_str{
    // 指针
    buffer: u32,
    // 长度
    length: usize,
    // 最长长度
    max_length: usize,
    fill1: DWORD,
    fill2: DWORD
}

