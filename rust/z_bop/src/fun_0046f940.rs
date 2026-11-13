// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009994fc: i32;
    static mut DAT_00999550: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046f5e0(param_1: *mut i32, param_2: *mut u32, param_3: i32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046f940() -> u32 {
    let mut local_8: u32 = 0;
    let mut local_4: i32 = 0;
    // SAFETY: the callee and globals are the game's own; the pointers returned
    // by the callee point at 0x3c bytes of data, copied 10 words at a time into
    // the game's table starting at DAT_00999550.
    unsafe {
        let pi_var1 = FUN_0046f5e0(&raw mut local_4, &raw mut local_8, 0x3c) as *mut i32;
        if pi_var1.is_null() {
            return 0;
        }
        DAT_009994fc = pi_var1.read_unaligned();
        let mut i_var4: i32 = 0;
        if 0 < DAT_009994fc {
            let mut pu_var5: *mut u32 = &raw mut DAT_00999550;
            loop {
                let mut pu_var2 = FUN_0046f5e0(&raw mut local_4, &raw mut local_8, 0x3c);
                if pu_var2.is_null() {
                    return 0;
                }
                let mut pu_var6 = pu_var5;
                let mut i_var3: i32 = 10;
                while i_var3 != 0 {
                    pu_var6.write_unaligned(pu_var2.read_unaligned());
                    pu_var2 = pu_var2.add(1);
                    pu_var6 = pu_var6.add(1);
                    i_var3 -= 1;
                }
                i_var4 = i_var4.wrapping_add(1);
                pu_var5 = pu_var5.add(10);
                if !(i_var4 < DAT_009994fc) {
                    break;
                }
            }
        }
        1
    }
}
