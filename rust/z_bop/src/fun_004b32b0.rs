// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a90b20: i32;
    static mut DAT_00a90bf0: i32;
    static mut DAT_00a90bf4: i32;
    static mut DAT_00a90bf8: i32;
    static mut DAT_00a90bfc: i32;
    static mut DAT_00a91a34: *mut *mut core::ffi::c_void;
    static mut DAT_00528cc0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b1600(param_1: i32, param_2: *mut i32, param_3: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b32b0() -> i32 {
    let mut rect: [i32; 4] = [0; 4];
    let mut fx: [i32; 0x19] = [0; 0x19];
    fx[0] = 100;
    // SAFETY: the globals are the game's own memory; the vtable slots are
    // read and called as the original does, with the stack-laid rect and
    // blit-effects struct passed by pointer.
    unsafe {
        // local_14 lies inside the effects struct, at dword index 20.
        fx[20] = 0;
        if DAT_00a90b20 != 0x10 {
            fx[20] = 0xffff;
        }
        rect[0] = DAT_00a90bf0;
        rect[1] = DAT_00a90bf4;
        rect[2] = DAT_00a90bf8.wrapping_add(DAT_00a90bf0);
        rect[3] = DAT_00a90bfc.wrapping_add(DAT_00a90bf4);
        let mut r: i32;
        loop {
            let this = DAT_00a91a34;
            let vt = this.read_unaligned() as *const u8;
            let blt = vt
                .add(0x14)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let blt: unsafe extern "stdcall" fn(
                *mut core::ffi::c_void,
                *mut i32,
                i32,
                i32,
                i32,
                *mut i32,
            ) -> i32 = core::mem::transmute(blt);
            r = blt(
                this as *mut core::ffi::c_void,
                rect.as_mut_ptr(),
                0,
                0,
                0x3000000,
                fx.as_mut_ptr(),
            );
            if r == 0 {
                return 1;
            }
            if r == -0x7789fe3e {
                let vt2 = DAT_00a91a34.read_unaligned() as *const u8;
                let restore = vt2
                    .add(0x6c)
                    .cast::<*const core::ffi::c_void>()
                    .read_unaligned();
                let restore: unsafe extern "stdcall" fn(*mut core::ffi::c_void) -> i32 =
                    core::mem::transmute(restore);
                r = restore(DAT_00a91a34 as *mut core::ffi::c_void);
            }
            if r != 0 {
                break;
            }
        }
        FUN_004b1600(r, &raw mut DAT_00528cc0, 0x79);
        FUN_004079e0();
        0
    }
}
