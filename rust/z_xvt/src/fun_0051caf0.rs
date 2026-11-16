// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536704: i8;
    static mut DAT_007427a0: [u8; 24];
    static mut DAT_0074244c: u16;
    static mut DAT_006550f4: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn EnterCriticalSection(lpCriticalSection: *mut c_void);
    fn LeaveCriticalSection(lpCriticalSection: *mut c_void);
    fn FUN_0051ca60() -> u16;
    fn FUN_0048f510();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: u32) -> i32;
    fn FUN_0051c130(param_1: i32);
    fn _fread(dst: *mut c_void, size: u32, count: u32, stream: i32) -> u32;
    fn _fclose(stream: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0051caf0() -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers returned
    // by FUN_0048f4f0 are read and written at the original's offsets.
    unsafe {
        if DAT_00536704 == 0 {
            return 0;
        }
        let mut tmp = FUN_0051ca60();
        if tmp == 0 {
            return 0;
        }
        EnterCriticalSection((&raw mut DAT_007427a0).cast::<c_void>());
        let dst_buf = FUN_0048f4f0(u32::from(tmp)) as *mut c_void;
        let read_count = _fread(dst_buf, 1, 0x2000, DAT_006550f4);
        let ptr = FUN_0048f4f0((read_count & 0xFFFF0000) | u32::from(DAT_0074244c));
        let p = (ptr.wrapping_add(0x42)) as *mut i32;
        let val = p.read_unaligned().wrapping_add(read_count as i32);
        p.write_unaligned(val);
        if read_count == 0x2000 {
            FUN_0048f510();
        } else {
            FUN_0051c130(0);
            *((ptr.wrapping_add(0x46)) as *mut i8) = 1;
            tmp = ((ptr.wrapping_add(0x48)) as *const u16).read_unaligned();
            FUN_0048f510();
            DAT_0074244c = tmp;
            _fclose(DAT_006550f4);
            DAT_006550f4 = 0;
        }
        FUN_0048f510();
        LeaveCriticalSection((&raw mut DAT_007427a0).cast::<c_void>());
        1
    }
}
