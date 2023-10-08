use std::ptr::copy_nonoverlapping;

use winapi::{shared::minwindef::{LPVOID, DWORD}, um::memoryapi::VirtualProtect};

// dest_addr 调用call地址
// stub_addr 跳转的地址
pub unsafe fn hook_any_address(dest_addr: DWORD, stub_addr: DWORD){

    let buf: &mut [u8] = &mut [0; 5];

    buf[0] = 0xE9;

    let rel_off = stub_addr as u32 - dest_addr as u32 - 5;
    buf[1..5].copy_from_slice(&rel_off.to_le_bytes());

    let mut old_prot: u32 = 0;
    let old_prot_ptr = &mut old_prot as *mut u32;
    // PAGE_EXECUTE_READWRITE = 0x40
    VirtualProtect(dest_addr as LPVOID, 5, 0x40, old_prot_ptr);
    
    copy_nonoverlapping(buf.as_mut_ptr(), dest_addr as *mut u8, 5);

    VirtualProtect(dest_addr as LPVOID, 5, old_prot, old_prot_ptr);
}