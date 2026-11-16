// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f1f8: i32;
    static mut DAT_0060f1dc: i32;
    static mut DAT_005272bc: i32;
    static mut DAT_0060f1c8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00475d50(param_1: i32, param_2: *mut u32, param_3: u32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00476280(param_1: i32, param_2: *mut u32, param_3: i32, param_4: i32) {
    // SAFETY: the globals and the callee are the game's own; DAT_0060f1c8
    // holds an address of a structure whose count at +8 and array pointer at
    // +0xc are read where the original reads them.
    unsafe {
        DAT_0060f1f8 = DAT_0060f1dc;
        let mut i_var2: i32 = 0;
        DAT_005272bc = 0;
        if 0 < ((DAT_0060f1c8 as usize + 8) as *const i32).read_unaligned() {
            let mut i_var1: i32 = 0;
            loop {
                let arr = ((DAT_0060f1c8 as usize + 0xc) as *const i32).read_unaligned();
                let elem = (arr.wrapping_add(i_var1) as usize as *const u32).read_unaligned();
                FUN_00475d50(param_1, param_2, elem, param_3, param_4);
                if DAT_005272bc == 0 {
                    return;
                }
                i_var1 = i_var1.wrapping_add(4);
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= ((DAT_0060f1c8 as usize + 8) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
    }
}
