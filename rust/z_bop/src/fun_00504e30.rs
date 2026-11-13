// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6138: u32;
    static mut DAT_00aa6114: u32;
    static mut DAT_00b6a5ae: i32;
    static mut DAT_00b6a5aa: u32;
    static mut DAT_00b69d20: i8;
    static mut DAT_0051a854: i8;
    static mut PTR_s_train_0052c1e8: *mut i8;
    static mut s__s__0052c300: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004cc4a0(path: *mut i8, mode: *mut core::ffi::c_void) -> i32;
    fn FUN_004cc660(f: i32, buf: *mut core::ffi::c_void);
    fn FUN_004cc5c0(f: i32, offset: i32, whence: i32);
    fn FUN_004cc6c0(f: i32, buf: *mut core::ffi::c_void, size: u32);
    fn FUN_004cc590(f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00504e30(param_1: *mut u32, param_2: i32) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a caller buffer of at least 0x1000 bytes, accessed as the original does.
    unsafe {
        let mut u_var4: u32 = 0;
        let mut puVar5 = param_1;
        let mut i_var2: i32 = 0x400;
        while i_var2 != 0 {
            puVar5.write_unaligned(0);
            puVar5 = puVar5.add(1);
            i_var2 -= 1;
        }
        if DAT_00aa6138 != 0 {
            let mut pi_var3 = DAT_00aa6114.wrapping_add(0x140) as *const i32;
            loop {
                let idx = DAT_00b6a5aa as usize;
                let v = (&raw mut DAT_00b6a5ae).add(idx).read_unaligned();
                if pi_var3.read_unaligned() == v {
                    break;
                }
                pi_var3 = pi_var3.wrapping_add(0x52);
                u_var4 = u_var4.wrapping_add(1);
                if !(u_var4 < DAT_00aa6138) {
                    break;
                }
            }
        }
        if u_var4 < DAT_00aa6138 {
            let name = (&raw mut PTR_s_train_0052c1e8)
                .add(DAT_00b6a5aa as usize)
                .read_unaligned();
            _sprintf(
                &raw mut DAT_00b69d20,
                &raw mut s__s__0052c300 as *const i8,
                name,
                u_var4.wrapping_mul(0x148).wrapping_add(DAT_00aa6114),
            );
            let p_fvar1 = FUN_004cc4a0(
                &raw mut DAT_00b69d20,
                (&raw mut DAT_0051a854).cast::<core::ffi::c_void>(),
            );
            if p_fvar1 != 0 {
                if DAT_00b6a5aa != 2 && DAT_00b6a5aa != 4 && DAT_00b6a5aa != 5 {
                    let mut local_2: i16 = 0;
                    FUN_004cc660(p_fvar1, (&raw mut local_2).cast::<core::ffi::c_void>());
                    if local_2 == 0xe {
                        let l_var6: i32 = if param_2 == 0 { -0x2000 } else { -0x3000 };
                        FUN_004cc5c0(p_fvar1, l_var6, 2);
                        FUN_004cc6c0(p_fvar1, param_1.cast::<core::ffi::c_void>(), 0x1000);
                        param_1.cast::<u8>().byte_add(0xfff).write(0);
                    }
                }
                FUN_004cc590(p_fvar1);
            }
        }
    }
}
