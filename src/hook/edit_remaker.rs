// 修改备注

use std::arch::asm;

use winapi::shared::minwindef::DWORD;

use crate::{helper::{WxStr, addr}, hook::get_wechatwin_addr};

static mut call_addr: DWORD = 0;

pub unsafe fn edit_remaker(name: String, wx_id: String) -> bool {
    let wechat_win = get_wechatwin_addr();

    let (name_data, wstr) = WxStr::new_with_chinese(name);
    let name_data_ptr = name_data.get_ptr();

    let (wx_id_data, wstr) = WxStr::new_without_chinese(wx_id);
    let wx_id_data_ptr = wx_id_data.get_ptr();

    call_addr = wechat_win.clone() + addr::EDIT_REMAKER_CALL;


    asm!{
        "pushad",
        "pushfd",
        "push eax",
        "mov eax, {wx_id_data_ptr}",
        "push eax",
        "call dword ptr ds:[{0}]",
        // "add esp, 0x8",
        "popfd",
        "popad",
        sym call_addr,
        wx_id_data_ptr = in(reg) wx_id_data_ptr,
        in("eax") name_data_ptr,
    };

    true
}