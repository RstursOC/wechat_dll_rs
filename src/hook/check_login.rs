use log::info;
use winapi::shared::minwindef::DWORD;

use regex::Regex;

use crate::helper::{read::{read_wx_id}, addr};

// 等待登录成功
// 通过检测内存中微信账号是否为空判断登录状态
// 包括退出登录，会清空内存中的微信账号
// 微信账号比微信ID加载快：界面没出来，微信账号加载；界面出来，就是微信ID加载
pub unsafe fn is_login(wechat_win: DWORD) -> bool{

    // 验证
    let test_addr = (wechat_win.clone() + addr::NEW_WXID) as *mut u32;
    if *test_addr == 0 {
        // info!("未登录：{}", *test_addr);
        return false;
    } else {
        let test_addr2 = (*test_addr) as *mut u8;
        // info!("当前WXID状态：{}", *test_addr2);
        if *test_addr2 == 0 {
            // 未登录
            return false;
        } else {
            // 已登录
            return true;
        }
    }
}
