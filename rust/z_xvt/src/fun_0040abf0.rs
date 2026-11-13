// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070a189: i32;
    static mut DAT_0085e792: i32;
    static mut DAT_0085e79b: i32;
    static mut DAT_0073ba30: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439830(a: i32, b: i32) -> u32;
    fn FUN_004398b0(a: i32);
    fn FUN_00490a50(a: *mut i8, b: i32, c: i32, d: i32, e: u32, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040abf0(param_1: i32) {
    // SAFETY: globals and callees are the game's own memory and code; the
    // computed address is an offset into the game's DAT_0073ba30 array.
    unsafe {
        if DAT_0070a189 != 0 && DAT_0085e792 != 0 && DAT_0085e79b != 0 {
            if param_1 == 0 {
                let u_var1 = FUN_00439830(0x27, 0x100);
                if u_var1 != 0 {
                    FUN_004398b0(0x27);
                }
                let u_var1 = FUN_00439830(0x28, 0x100);
                if u_var1 != 0 {
                    FUN_004398b0(0x28);
                    return;
                }
            } else {
                let mut u_var1: u32 = 0x7f;
                if DAT_0085e79b < 10 {
                    u_var1 = (DAT_0085e79b as u32).wrapping_mul(0xd);
                }
                let i_var3: i32;
                if param_1 == 1 {
                    i_var3 = 0x28;
                    u_var1 /= 3;
                } else {
                    i_var3 = 0x27;
                    u_var1 /= 2;
                }
                let u_var2 = FUN_00439830(i_var3, 0x100);
                if u_var2 == 0 {
                    let p = (&raw mut DAT_0073ba30)
                        .offset(i_var3.wrapping_mul(6) as isize)
                        .cast::<i8>();
                    FUN_00490a50(p, 1, 1, 0x7d, u_var1, 0x40);
                }
            }
        }
    }
}
