// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
    static mut DAT_00732950: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432cc0(param_1: i32) -> *mut i32;
    fn FUN_004a3490(param_1: i32, param_2: *mut i32);
    fn FUN_004a1d70(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00444330(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; the scan walks
    // the game's fixed-stride table exactly as the original does.
    unsafe {
        if DAT_0071ecf4 != param_1 {
            let pi_var1 = FUN_00432cc0(param_1);
            FUN_004a3490(0, pi_var1);
            FUN_004a1d70(0x173, DAT_0071ecf4);
            let mut i_var3: i32 = 0;
            let mut pc_var2: *const i8 = &raw mut DAT_0071b740 as *const i8;
            loop {
                if pc_var2.read() == 1 {
                    i_var3 = i_var3.wrapping_add(1);
                }
                pc_var2 = pc_var2.byte_add(0x5bd);
                if pc_var2 >= &raw mut DAT_0071e528 as *const i8 {
                    break;
                }
            }
            let idx = DAT_0071ecf4.wrapping_mul(0x5bd);
            let base = &raw mut DAT_0071b740 as *const i8;
            if base.offset(idx as isize).read() == 1 {
                if i_var3 == 1 {
                    FUN_004a1d70(0x175, DAT_0071ecf4);
                    return;
                }
                DAT_00732950 = i_var3 as u16;
                FUN_004a1d70(0x174, DAT_0071ecf4);
            }
        }
    }
}
