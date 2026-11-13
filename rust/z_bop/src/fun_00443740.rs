// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521ad0: i32;
    static mut DAT_00521ad4: i32;
    static mut DAT_00521ad8: i32;
    static mut DAT_00556904: *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00443740() {
    let mut local_100: [i8; 0x100] = [0; 0x100];
    // SAFETY: the globals and the object behind DAT_00556904 are the game's
    // own memory; the call target is computed exactly as the C does (the
    // first pointer of the object plus 0x24 bytes), and the buffer is a
    // local array of 0x100 bytes passed to it.
    unsafe {
        let base = DAT_00556904
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let target = base.byte_add(0x24);
        let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, i32, *mut i8) -> i32 =
            core::mem::transmute(target);
        let i_var1 = f(DAT_00556904, 0x100, local_100.as_mut_ptr());
        if i_var1 == 0 {
            if local_100[0x1E] == 0 && local_100[0x2A] == 0 {
                DAT_00521ad4 = 0;
            } else {
                DAT_00521ad4 = 1;
            }
            if local_100[0x11] == 0 && local_100[0x91] == 0 {
                DAT_00521ad0 = 0;
            } else {
                DAT_00521ad0 = 1;
            }
            if local_100[0x2C] == 0 && local_100[0xAC] == 0 {
                DAT_00521ad8 = 0;
                return;
            }
            DAT_00521ad8 = 1;
        }
    }
}
