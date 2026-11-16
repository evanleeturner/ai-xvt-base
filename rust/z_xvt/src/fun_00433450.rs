// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005251b0: u32;
    static mut DAT_005251b4: u32;
    static mut DAT_005251ac: u32;
    static mut DAT_00539a08: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00433450(param_1: u32) -> u32 {
    let mut param_1 = param_1;
    // SAFETY: the globals and the table at DAT_00539a08 are the game's own
    // memory; the copies read and write the same words the original does.
    unsafe {
        if param_1 == DAT_005251b0 {
            DAT_005251b0 = DAT_005251b0.wrapping_add(1);
            DAT_005251b4 = DAT_005251b4.wrapping_sub(1);
            if 0x3ff < (DAT_005251b0 as i32) {
                DAT_005251b0 = 0;
            }
            return 1;
        }
        let mut u_var2: u32 = param_1.wrapping_add(1);
        if 0x3ff < u_var2 {
            u_var2 = 0;
        }
        let mut u_var3: u32 = DAT_005251b4.wrapping_add(DAT_005251b0);
        if 0x3ff < u_var3 {
            u_var3 = u_var3.wrapping_sub(0x400);
        }
        let base = (&raw mut DAT_00539a08).cast::<u32>();
        while u_var3 != u_var2 {
            let mut u_var4: u32 = param_1.wrapping_add(1);
            let mut p5 = base.wrapping_add(u_var2.wrapping_mul(0x84) as usize);
            let mut p6 = base.wrapping_add(param_1.wrapping_mul(0x84) as usize);
            let mut i_var1: i32 = 0x84;
            while i_var1 != 0 {
                p6.write_unaligned(p5.read_unaligned());
                p5 = p5.wrapping_add(1);
                p6 = p6.wrapping_add(1);
                i_var1 -= 1;
            }
            if 0x3ff < u_var4 {
                u_var4 = 0;
            }
            u_var2 = u_var2.wrapping_add(1);
            param_1 = u_var4;
            if 0x3ff < u_var2 {
                u_var2 = 0;
            }
        }
        DAT_005251b4 = DAT_005251b4.wrapping_sub(1);
        if DAT_005251ac == 0 {
            DAT_005251ac = 0x3ff;
            return 0;
        }
        DAT_005251ac = DAT_005251ac.wrapping_sub(1);
        0
    }
}
