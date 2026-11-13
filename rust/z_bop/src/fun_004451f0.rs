// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0055693d: *mut c_void;
    static mut DAT_00556930: i32;
    static mut DAT_00556928: *mut c_void;
    static mut DAT_0055696d: *mut c_void;
    static mut DAT_00556959: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00444f00(param_1: u32);
    fn _malloc(size: i32) -> *mut c_void;
    fn FUN_004445e0(param_1: *mut c_void);
    fn _free(ptr: *mut c_void);
    fn _exit(code: i32) -> !;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004438b0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004451f0(mut param_1: u32, _param_2: u32, mut param_3: *mut u8) {
    // SAFETY: the globals and callees are the game's own; param_3 points at the
    // caller's output buffer and the global buffers are read as the original does.
    unsafe {
        FUN_00444f00(param_1);
        DAT_0055693d = _malloc(DAT_00556930.wrapping_mul(9));
        if !DAT_0055693d.is_null() {
            let mut i_var5: i32 = 0;
            DAT_00556930 = 0;
            FUN_004445e0(DAT_00556928);
            if 0 < (param_1 as i32) {
                loop {
                    let pbvar4 = (DAT_0055693d as *mut u8).offset(i_var5 as isize);
                    i_var5 = i_var5.wrapping_add(9);
                    let b1 = pbvar4.add(1).read();
                    let b2 = pbvar4.add(2).read();
                    param_3.write(pbvar4.read() >> 2);
                    param_1 = param_1.wrapping_sub(1);
                    param_3.add(1).write(b1 >> 2);
                    param_3.add(2).write(b2 >> 2);
                    param_3 = param_3.add(3);
                    if param_1 == 0 {
                        break;
                    }
                }
            }
            _free(DAT_0055693d);
            loop {
                let pv_var3 = (DAT_0055696d as *mut u8)
                    .byte_add(0x29000)
                    .cast::<*mut c_void>()
                    .read_unaligned();
                _free(DAT_0055696d);
                DAT_0055696d = pv_var3;
                if pv_var3.is_null() {
                    break;
                }
            }
            DAT_00556959 = (DAT_00556959 as *mut u8).wrapping_offset(-0x3fc) as *mut c_void;
            _free(DAT_00556959);
            return;
        }
        FUN_004438b0();
        _exit(1);
    }
}
