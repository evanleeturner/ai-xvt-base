// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00a91ca0: i32;
    static mut DAT_00a91cc0: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d1400() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004facc0() -> i32 {
    let mut i_var3: i32 = 0;
    // SAFETY: the globals and the callee are the game's own; the loop walks
    // the game's int array from DAT_00a91ca0 up to DAT_00a91cc0.
    unsafe {
        if DAT_00b69e30 == 2 {
            return 0;
        }
        let i_var1 = FUN_004d1400();
        let mut pvar2: *mut i32 = &raw mut DAT_00a91ca0;
        let end: *mut i32 = &raw mut DAT_00a91cc0;
        loop {
            if pvar2.read_unaligned() != 0 {
                i_var3 = i_var3.wrapping_add(1);
            }
            pvar2 = pvar2.wrapping_add(1);
            if pvar2 >= end {
                break;
            }
        }
        (i_var3 == i_var1) as i32
    }
}
