use std::ptr;

use windows_sys::{
    Win32::{
        Foundation::HANDLE,
        Foundation::{BOOL, HWND},
        System::{
            Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
            Threading::{OpenProcess, PROCESS_ALL_ACCESS},
        },
        UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId},
    },
    *,
};

type DWORD = u32;

fn get_from_memory(&process: &HANDLE, addr: DWORD) -> Option<DWORD> {
    let mut buffer: [u8; 4] = [0; 4];
    let addr = addr as *mut _;

    let sucess = unsafe {
        ReadProcessMemory(
            process,
            addr,
            buffer.as_mut_ptr() as _,
            buffer.len(),
            ptr::null_mut(),
        )
    };

    if sucess == 0 {
        println!("Failed to read memory from process");
        return None;
    }
    let value = u32::from_le_bytes(buffer);

    Some(value)
}

fn main() {
    let westnoth_win: HWND =
        unsafe { FindWindowW(std::ptr::null(), w!("The Battle for Wesnoth - 1.14.9")) };

    let mut process_id: u32 = 0;
    unsafe { GetWindowThreadProcessId(westnoth_win, &mut process_id as *mut u32) };

    let true_flag = true as BOOL;

    let westnoth_process: HANDLE =
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, true_flag, process_id) };

    // let mut gold_value_buffer: [u8; 4] = [0; 4];
    // let mut bytes_read: usize = 0;
    // let addr = 0x017EED18 as *mut _;
    let addr = 0x017EED18;


    // let sucess = unsafe {
    //     ReadProcessMemory(
    //         westnoth_process,
    //         addr,
    //         gold_value_buffer.as_mut_ptr() as _,
    //         gold_value_buffer.len(),
    //         ptr::null_mut(),
    //     )
    // };

    // if sucess == 0 {
    //     println!("Failed to read memory from process");
    // }

    // let value = u32::from_le_bytes(gold_value_buffer);
    let value = get_from_memory(&westnoth_process, addr).unwrap();

    let addr = value + 0xa90;

    // let sucess = unsafe {
    //     ReadProcessMemory(
    //         westnoth_process,
    //         addr,
    //         gold_value_buffer.as_mut_ptr() as _,
    //         gold_value_buffer.len(),
    //         ptr::null_mut(),
    //     )
    // };

    // if sucess == 0 {
    //     println!("Failed to read memory from process");
    // }

    let value = get_from_memory(&westnoth_process, addr).unwrap();

    // let value = u32::from_le_bytes(gold_value_buffer);
    // let next_value = value + 4;
    // let mut gold_value_buffer = next_value.to_le_bytes();

    // let sucess = unsafe {
    //     ReadProcessMemory(
    //         westnoth_process,
    //         addr,
    //         gold_value_buffer.as_mut_ptr() as _,
    //         gold_value_buffer.len(),
    //         ptr::null_mut(),
    //     )
    // };

    // if sucess == 0 {
    //     println!("Failed to read memory from process");
    // }

    // // gold_value += 0xa90;

    // // ReadProcessMemory(westnoth_process,gold_value  as *const libc::c_void, gold_value  as *mut libc::c_void, 4, &mut bytes_read as *mut usize);
    // // gold_value += 4;

    // let  new_gold_value: DWORD = 1000;
    // let buffer = new_gold_value.to_le_bytes();
    // let addr = u32::from_le_bytes(gold_value_buffer);
    // // let mut bytes_written: usize = 0;

    // let sucess = unsafe {
    //     WriteProcessMemory(
    //         westnoth_process,
    //         addr as *mut _,
    //         buffer.as_ptr() as _,
    //         buffer.len(),
    //         ptr::null_mut(),
    //     )
    // };

    // if sucess == 0 {
    //     println!("Failed to read memory from process");
    // }
    // WriteProcessMemory(westnoth_process, gold_value  as *const libc::c_void, new_gold_value  as *mut libc::c_void, 4, &mut bytes_written  as *mut usize);
    println!("END");
    return;
}
