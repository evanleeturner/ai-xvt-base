// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10514: i32;
    static mut DAT_00a606d0: i32;
    static mut DAT_00a6057c: i32;
    static mut DAT_00a60580: i32;
    static mut DAT_00a6052c: i32;
    static mut DAT_00a60574: i32;
    static mut DAT_00a60578: i32;
    static mut DAT_00a6056c: i32;
    static mut DAT_00a60570: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
    #[link_name = "FID_conflict__memcpy"]
    fn FID_conflict__memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: u32);
    fn _strncpy(dest: *mut i8, src: *const i8, n: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042d120(
    param_1: *mut i8,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's string, and the table writes land where the original's do.
    unsafe {
        if DAT_00a10514 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        if FUN_0042dda0(param_1) == -1 {
            return 0;
        }
        let mut i_var1: i32 = 0;
        if 0 < DAT_00a606d0 {
            let mut pi_var2: *mut i32 = &raw mut DAT_00a6057c;
            loop {
                if pi_var2.read_unaligned() < param_4 {
                    break;
                }
                pi_var2 = pi_var2.wrapping_add(0x15);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < DAT_00a606d0) {
                    break;
                }
            }
        }
        if DAT_00a606d0 == i_var1 && i_var1 == 4 {
            return 0;
        }
        let off = i_var1.wrapping_mul(0x54) as isize;
        FID_conflict__memcpy(
            (&raw mut DAT_00a60580).cast::<u8>().offset(off).cast(),
            (&raw mut DAT_00a6052c).cast::<u8>().offset(off).cast(),
            (DAT_00a606d0.wrapping_sub(i_var1) as u32).wrapping_mul(0x54),
        );
        _strncpy(
            (&raw mut DAT_00a6052c).cast::<u8>().offset(off).cast(),
            param_1,
            0x40,
        );
        let idx = i_var1.wrapping_mul(0x15) as isize;
        (&raw mut DAT_00a60574).offset(idx).write_unaligned(param_3);
        (&raw mut DAT_00a60578).offset(idx).write_unaligned(param_2);
        (&raw mut DAT_00a6057c).offset(idx).write_unaligned(param_4);
        (&raw mut DAT_00a6056c).offset(idx).write_unaligned(param_5);
        (&raw mut DAT_00a60570).offset(idx).write_unaligned(param_6);
        DAT_00a606d0 = DAT_00a606d0.wrapping_add(1);
        if 4 < DAT_00a606d0 {
            DAT_00a606d0 = 4;
        }
        1
    }
}
