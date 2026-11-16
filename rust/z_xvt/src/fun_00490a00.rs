// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a8810: i32;
    static mut DAT_006a86b0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00490be0(param_1: *mut i8, param_2: i32, param_3: u32, param_4: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00490a00() {
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_006a86b0 is walked in 0x15-dword records as the original does.
    unsafe {
        let mut i_var5: i32 = 0;
        if DAT_006a8810 > 0 {
            let mut pu_var6: *mut u32 = (&raw mut DAT_006a86b0).cast::<u32>();
            loop {
                let pu_var1 = pu_var6.wrapping_add(3);
                let pu_var2 = pu_var6.wrapping_add(1);
                let pi_var3 = pu_var6.wrapping_add(2).cast::<i32>();
                let pc_var4 = pu_var6.wrapping_sub(0x11).cast::<i8>();
                pu_var6 = pu_var6.wrapping_add(0x15);
                i_var5 = i_var5.wrapping_add(1);
                let a2 = pi_var3.read_unaligned();
                let a3 = pu_var2.read_unaligned();
                let a4 = pu_var1.read_unaligned() as usize as *mut i8;
                FUN_00490be0(pc_var4, a2, a3, a4);
                if !(i_var5 < DAT_006a8810) {
                    break;
                }
            }
        }
        DAT_006a8810 = 0;
    }
}
