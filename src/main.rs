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

fn write_to_memory(&process: &HANDLE, addr: DWORD, value_to_write: &[u8]) -> bool {
    let addr = addr as *mut _;

    let sucess = unsafe {
        WriteProcessMemory(
            process,
            addr,
            value_to_write.as_ptr() as _,
            value_to_write.len(),
            ptr::null_mut(),
        )
    };

    if sucess == 0 {
        println!("Failed to write memory to process");
        return false;
    }

    true
}

fn main() {
    let westnoth_win: HWND =
        unsafe { FindWindowW(std::ptr::null(), w!("The Battle for Wesnoth - 1.14.9")) };

    let mut process_id: u32 = 0;
    unsafe { GetWindowThreadProcessId(westnoth_win, &mut process_id as *mut u32) };

    let true_flag = true as BOOL;

    let westnoth_process: HANDLE =
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, true_flag, process_id) };

    let addr = 0x017EED18;

    let value = get_from_memory(&westnoth_process, addr).unwrap();
    let addr = value + 0xa90;

    let value = get_from_memory(&westnoth_process, addr).unwrap();
    let addr = value + 4;

    let new_gold_value: DWORD = 1000;
    let value_to_write = new_gold_value.to_le_bytes();
    let result = write_to_memory(&westnoth_process, addr, &value_to_write);

    if !result {
        panic!("Didn't change money");
    }

    println!("END");
    return;
}
