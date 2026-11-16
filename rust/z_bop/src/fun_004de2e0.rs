// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fb7: *mut core::ffi::c_void;
    static mut DAT_00aa7fbf: *mut core::ffi::c_void;
    static mut DAT_00aa7fc3: *mut core::ffi::c_void;
    static mut _DAT_00aa7fbb: *mut core::ffi::c_void;
    static mut DAT_00aa7fcf: *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004de2e0() -> i32 {
    // SAFETY: the globals are the game's own memory; the object pointer,
    // its vtable slot and the two tables are written where the original does.
    unsafe {
        let local_fb7: i32 = DAT_00aa7fb7 as usize as i32;
        if local_fb7 == 0 {
            return 1;
        }
        let local_vtable: i32 = (local_fb7 as usize as *const u8)
            .byte_add(8)
            .cast::<i32>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(i32) =
            core::mem::transmute(local_vtable as usize as *const core::ffi::c_void);
        f(local_fb7);

        DAT_00aa7fb7 = core::ptr::null_mut();

        let mut i_var1: i32 = 0;
        loop {
            i_var1 = i_var1.wrapping_add(0x145);
            let base = DAT_00aa7fbf as *mut u8;
            base.wrapping_offset((i_var1.wrapping_sub(5)) as isize)
                .cast::<i32>()
                .write_unaligned(0);
            let base = DAT_00aa7fbf as *mut u8;
            *base.wrapping_offset((i_var1.wrapping_sub(0x145)) as isize) = 0;
            if i_var1 >= 0xa280 {
                break;
            }
        }

        i_var1 = 0;
        loop {
            i_var1 = i_var1.wrapping_add(0xc);
            let base = DAT_00aa7fc3 as *mut u8;
            base.wrapping_offset((i_var1.wrapping_sub(0xc)) as isize)
                .cast::<i32>()
                .write_unaligned(-1);
            let base = DAT_00aa7fc3 as *mut u8;
            base.wrapping_offset((i_var1.wrapping_sub(8)) as isize)
                .cast::<i32>()
                .write_unaligned(0);
            let base = DAT_00aa7fc3 as *mut u8;
            base.wrapping_offset((i_var1.wrapping_sub(4)) as isize)
                .cast::<i32>()
                .write_unaligned(0);
            if i_var1 >= 0x90 {
                break;
            }
        }

        _DAT_00aa7fbb = core::ptr::null_mut();
        DAT_00aa7fcf = core::ptr::null_mut();
        1
    }
}
