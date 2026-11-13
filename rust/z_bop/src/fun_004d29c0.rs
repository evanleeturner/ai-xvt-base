// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69a89: u32;
    static mut DAT_00b641c1: [i32; 0];
    static mut DAT_00665050: [i32; 0];
    static mut DAT_00665060: [i32; 0];
    static mut DAT_00665410: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d23b0(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d29c0(param_1: i32) -> i32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointers walk the game's tables exactly as the original does.
    unsafe {
        let u_var1 = FUN_004d23b0(param_1);
        let i_var4: i32;
        if u_var1 < DAT_00b69a89 && u_var1 < 0x28 {
            i_var4 = (&raw mut DAT_00b641c1)
                .cast::<i32>()
                .add(u_var1.wrapping_mul(0x8e) as usize)
                .read_unaligned();
        } else {
            i_var4 = 0;
        }
        let mut i_var2: usize = 0;
        let mut pi_var3: *const i32 = (&raw mut DAT_00665050).cast::<i32>();
        let end: *const i32 = (&raw mut DAT_00665410).cast::<i32>();
        let base60: *const i32 = (&raw mut DAT_00665060).cast::<i32>();
        loop {
            if pi_var3.read_unaligned() == param_1 {
                return base60.add(i_var2 * 6).read_unaligned().wrapping_add(i_var4);
            }
            pi_var3 = pi_var3.add(6);
            i_var2 += 1;
            if !(pi_var3 < end) {
                break;
            }
        }
        i_var4
    }
}
