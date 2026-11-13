// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a5aa: i32;
    static mut DAT_00b6a606: i32;
    static mut DAT_00b6a248: i32;
    static mut DAT_00b69e34: i32;
    static mut DAT_00b6a5c2: i32;
    static mut DAT_00b6a5ae: i32;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d3350();
    fn FUN_004d4f70();
    fn FUN_004d4970();
    fn FUN_004d4c10();
    fn FUN_004d49a0();
    fn FUN_004efce0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004d4f80();
    fn FUN_004d48e0() -> i32;
    fn FUN_004d4f60();
    fn FUN_004d3400();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df5f0(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the record table
    // at DAT_00b6a248 is read at the same offsets the original uses.
    unsafe {
        if DAT_00b6a5aa == 0 && DAT_00b6a606 == 1 {
            if DAT_00b6a248 == 0 {
                return 0;
            }
            let mut u_var4: u32 = 0;
            if DAT_00b69e34 != 0 {
                let mut i_var3: i32 = 0;
                loop {
                    let i_var2 = DAT_00b6a248.wrapping_add(i_var3);
                    if ((i_var2.wrapping_add(0x120) as *const i32).read_unaligned() == DAT_00b6a5c2)
                        && ((i_var2.wrapping_add(0x124) as *const i32).read_unaligned() == param_1)
                        && ((i_var2.wrapping_add(0x128) as *const i32).read_unaligned()
                            == DAT_00b6a5ae)
                    {
                        FUN_004d3350();
                        FUN_004d4f70();
                        FUN_004d4970();
                        FUN_004d4c10();
                        FUN_004d49a0();
                        FUN_004d4c10();
                        let b_var1 =
                            FUN_004efce0(DAT_00b6a248.wrapping_add(i_var3) as *mut i8, 1) as u8;
                        FUN_004d4c10();
                        FUN_004d49a0();
                        FUN_004d4c10();
                        FUN_004d4f80();
                        DAT_00aa6cfc = FUN_004d48e0();
                        FUN_004d4f60();
                        FUN_004d3400();
                        if b_var1 != 0 {
                            return 0;
                        }
                    }
                    i_var3 = i_var3.wrapping_add(300);
                    u_var4 = u_var4.wrapping_add(1);
                    if !(u_var4 < DAT_00b69e34 as u32) {
                        break;
                    }
                }
            }
            return 1;
        }
        0
    }
}
