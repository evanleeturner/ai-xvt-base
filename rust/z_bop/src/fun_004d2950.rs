// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69a89: u32;
    static mut DAT_00b641bd: i32;
    static mut DAT_00665050: i32;
    static mut DAT_0066505c: i32;
    static mut DAT_00665410: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d23b0(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2950(param_1: i32) -> i32 {
    // SAFETY: the callee and the globals are the game's own memory; the
    // table pointers are walked exactly as the original does.
    unsafe {
        let u_var1 = FUN_004d23b0(param_1) as u32;
        let i_var4: i32;
        if u_var1 < DAT_00b69a89 && u_var1 < 0x28 {
            let idx = u_var1.wrapping_mul(0x8e) as usize;
            i_var4 = (&raw mut DAT_00b641bd).add(idx).read_unaligned();
        } else {
            i_var4 = 0;
        }
        let mut i_var3: i32 = 0;
        let mut pi_var2: *mut i32 = &raw mut DAT_00665050;
        loop {
            if pi_var2.read_unaligned() == param_1 {
                FUN_004d23b0(param_1);
                let v = (&raw mut DAT_0066505c)
                    .offset((i_var3.wrapping_mul(6)) as isize)
                    .read_unaligned();
                return v.wrapping_add(i_var4);
            }
            pi_var2 = pi_var2.add(6);
            i_var3 = i_var3.wrapping_add(1);
            if (pi_var2 as usize) >= ((&raw mut DAT_00665410) as usize) {
                break;
            }
        }
        i_var4
    }
}
