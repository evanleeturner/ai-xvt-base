// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070a189: i8;
    static mut DAT_0085e792: i8;
    static mut DAT_0085e79b: i8;
    static mut DAT_0073bef8: i8;
    static mut DAT_0073bee0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439830(param_1: i32, param_2: i32) -> u32;
    fn FUN_004398b0(param_1: i32) -> i32;
    fn FUN_00490a50(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040a840(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the addresses
    // passed are of the game's own data.
    unsafe {
        if DAT_0070a189 == 0 {
            return 0;
        }
        if DAT_0085e792 == 0 {
            return 0;
        }
        if DAT_0085e79b == 0 {
            return 0;
        }
        let mut i_var2: i32 = 0x7f;
        if DAT_0085e79b < 10 {
            i_var2 = (DAT_0085e79b as i32).wrapping_mul(0xd);
        }
        if param_1 == 0 || param_1 == 1 {
            let u_var1 = FUN_00439830(0x33, 0x100);
            if u_var1 != 0 {
                FUN_004398b0(0x33);
                return 1;
            }
            let u_var1 = FUN_00439830(0x32, 0x100);
            if u_var1 != 0 {
                FUN_004398b0(0x32);
            }
        } else if param_1 == 3 {
            let u_var1 = FUN_00439830(0x32, 0x100);
            if u_var1 != 0 {
                FUN_004398b0(0x32);
            }
            let u_var1 = FUN_00439830(0x33, 0x100);
            if u_var1 == 0 {
                FUN_00490a50((&raw mut DAT_0073bef8).cast(), 1, 1, 0x7d, i_var2, 0x40);
                return 1;
            }
        } else {
            let u_var1 = FUN_00439830(0x33, 0x100);
            if u_var1 != 0 {
                FUN_004398b0(0x33);
            }
            let u_var1 = FUN_00439830(0x32, 0x100);
            if u_var1 == 0 {
                FUN_00490a50((&raw mut DAT_0073bee0).cast(), 1, 1, 0x7d, i_var2, 0x40);
                return 1;
            }
        }
        1
    }
}
