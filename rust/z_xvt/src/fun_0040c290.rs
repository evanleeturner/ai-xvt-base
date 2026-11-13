// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e796: u8;
    static mut DAT_00523ac8: u8;
    static mut DAT_00523ad8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004784d0() -> u16;
    fn FUN_0040c4c0(param_1: i32, param_2: i8, param_3: i8, param_4: i8, param_5: i16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040c290(param_1: i32) -> u32 {
    // SAFETY: the globals and callees are the game's own; the table reads
    // index the game's byte arrays exactly as the original does.
    unsafe {
        if DAT_0085e796 == 0 {
            return 0;
        }
        let b_var1 = (&raw mut DAT_00523ac8).offset(param_1 as isize).read();
        if !(0..=9).contains(&param_1) {
            return 1;
        }
        let b_var2 = (&raw mut DAT_00523ad8).offset(param_1 as isize).read();
        let mut u_var5: u32 = 0;
        if b_var2 != 0 {
            let u_var4 = FUN_004784d0();
            let u_var3 = u_var4 / (b_var2 as u16);
            if u_var3 != 0 {
                u_var5 = (u_var4 as u32) % (u_var3 as u32);
            }
        }
        FUN_0040c4c0(
            u_var5.wrapping_add(b_var1 as u32).wrapping_add(0x321) as i32,
            3,
            param_1 as i8,
            0,
            -1,
        );
        1
    }
}
