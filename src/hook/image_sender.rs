use std::arch::asm;

use log::info;
use winapi::shared::minwindef::DWORD;

use crate::{helper::{addr, WxStr}, hook::get_wechatwin_addr};

static mut WXID_PTR: DWORD = 0;
static mut PATH_PTR: DWORD = 0;

static mut CALL1: DWORD = 0;
static mut CALL2: DWORD = 0;
static mut CALL3: DWORD = 0;
static mut CALL4: DWORD = 0;

static mut BUFF_1: DWORD = 0;
static mut BUFF_2: DWORD = 0;

static mut TEMP_EAX: DWORD = 0;

static mut SUCCESS: DWORD = 0;


pub unsafe fn send_image(wxid: String, path: String){

    let wechat_win = get_wechatwin_addr();

    // info!("发送图片中的基础地址：{}", wechat_win);

    CALL1 = wechat_win.clone() + addr::SEND_IMAGE_CALL1;
    CALL2 = wechat_win.clone() + addr::SEND_IMAGE_CALL2;
    CALL3 = wechat_win.clone() + addr::SEND_IMAGE_CALL3;
    CALL4 = wechat_win.clone() + addr::SEND_IMAGE_CALL4;
    // CALL4 = wechat_win.clone() + 0x237D85;

    // JUMP = wechat_win.clone() + addr::SEND_IMAGE_JUMP;

    let (path_data, wstr) = WxStr::new_with_chinese(path);

    PATH_PTR = path_data.get_ptr();

    let (wxid_data, wstr) = WxStr::new_without_chinese(wxid);

    WXID_PTR = wxid_data.get_ptr();

    // let buff_1: [DWORD; 0x50] = [0; 0x50];
    // BUFF_1 =&buff_1 as *const _ as u32;
    let buff_1 = WxStr::new_null();
    BUFF_1 = buff_1.get_ptr();

    let buff_2: [DWORD; 0x2A8] = [0; 0x2A8];
    BUFF_2 = &buff_2 as *const _ as u32;

    asm!{
        "pushad",
        // "pushfd",
        "call dword ptr ds:[{}]",
        "sub esp, 0x14",
        "mov dword ptr ds:[{}], eax",
        "mov eax, dword ptr ds:[{}]",
        "mov ecx, esp",
        "mov edi, dword ptr ss:[{}]",
        "push eax",
        "call dword ptr ds:[{}]",
        "mov ecx, dword ptr ds:[{}]",
        "mov eax, dword ptr ds:[{}]",
        "push edi",
        "push eax",
        "lea eax, dword ptr ds:[{}]",
        "push eax",
        "call dword ptr ds:[{}]",
        // "mov dword ptr ds:[{}], eax",
        "mov ecx, dword ptr ds:[{}]",
        "call dword ptr ds:[{}]",
        // "popfd",
        "popad",
        
        sym CALL1,
        sym TEMP_EAX,
        sym BUFF_1,
        sym PATH_PTR,
        sym CALL2,
        sym TEMP_EAX,
        sym WXID_PTR,
        sym BUFF_2,
        sym CALL3,
        // sym SUCCESS,
        sym BUFF_2,
        sym CALL4,

    };
}


// 79127D4E    E8 3D9BF0FF     call WeChatWi.79031890                         ; 开始位置：修改eax call
// 79127D53    83EC 14         sub esp,0x14
// 79127D56    8945 98         mov dword ptr ss:[ebp-0x68],eax                ; 这里存的数据后面给ecx
// 79127D59    8D46 1C         lea eax,dword ptr ds:[esi+0x1C]                ; 清空eax
// 79127D5C    8BCC            mov ecx,esp
// 79127D5E    8D7E 08         lea edi,dword ptr ds:[esi+0x8]                 ; 文件路径
// 79127D61    50              push eax                                       ; 缓冲区,大小：0x28
// 79127D62    E8 39EB5D00     call WeChatWi.797068A0
// 79127D67    8B4D 98         mov ecx,dword ptr ss:[ebp-0x68]
// 79127D6A    8D45 9C         lea eax,dword ptr ss:[ebp-0x64]                ; wxid
// 79127D6D    57              push edi                                       ; 文件路径
// 79127D6E    50              push eax                                       ; wxid
// 79127D6F    8D85 58FCFFFF   lea eax,dword ptr ss:[ebp-0x3A8]
// 79127D75    50              push eax                                       ; 指针，指向0x0
// 79127D76    E8 55503900     call WeChatWi.794BCDD0                         ; 结束位置

