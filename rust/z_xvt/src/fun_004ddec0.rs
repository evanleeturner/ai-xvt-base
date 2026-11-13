// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759aec: *mut core::ffi::c_void;
    static mut DAT_00759af4: *mut core::ffi::c_void;
    static mut DAT_00759af8: *mut core::ffi::c_void;
    static mut _DAT_00759af0: u32;
    static mut DAT_00759b04: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ddec0() -> u32 {
    // SAFETY: the globals are the game's own memory; the object at
    // DAT_00759aec holds a function pointer at offset 8, and the tables at
    // DAT_00759af4 and DAT_00759af8 are written where the original does.
    unsafe {
        if DAT_00759aec.is_null() {
            return 1;
        }
        let obj = DAT_00759aec;
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) = core::mem::transmute(
            obj.byte_add(8)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned(),
        );
        f(obj);
        let mut i_var1: i32 = 0;
        DAT_00759aec = core::ptr::null_mut();
        loop {
            i_var1 = i_var1.wrapping_add(0x145);
            DAT_00759af4
                .byte_offset((i_var1.wrapping_sub(5)) as isize)
                .cast::<u32>()
                .write_unaligned(0);
            DAT_00759af4
                .byte_offset((i_var1.wrapping_sub(0x145)) as isize)
                .cast::<u8>()
                .write(0);
            if !(i_var1 < 0xa280) {
                break;
            }
        }
        i_var1 = 0;
        loop {
            i_var1 = i_var1.wrapping_add(0xc);
            DAT_00759af8
                .byte_offset((i_var1.wrapping_sub(0xc)) as isize)
                .cast::<u32>()
                .write_unaligned(0xffff_ffff);
            DAT_00759af8
                .byte_offset((i_var1.wrapping_sub(8)) as isize)
                .cast::<u32>()
                .write_unaligned(0);
            DAT_00759af8
                .byte_offset((i_var1.wrapping_sub(4)) as isize)
                .cast::<u32>()
                .write_unaligned(0);
            if !(i_var1 < 0x90) {
                break;
            }
        }
        _DAT_00759af0 = 0;
        DAT_00759b04 = 0;
        1
    }
}
