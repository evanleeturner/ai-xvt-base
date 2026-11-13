// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0a880: [u8; 0];
    static mut DAT_00a0aaa0: [u8; 0];
    static mut DAT_00a0aa20: [u8; 0];
    static mut DAT_009a20a4: u32;
    static mut DAT_00a0fc80: [u32; 0];
    static mut DAT_00a0a910: [u16; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00430930(param_1: u32) {
    let mut param_1 = param_1;
    // SAFETY: all globals are the game's own arrays; indices are used as the
    // original does, through raw pointers.
    unsafe {
        let a880 = (&raw mut DAT_00a0a880).cast::<u8>();
        let aaa0 = (&raw mut DAT_00a0aaa0).cast::<u8>();
        let aa20 = (&raw mut DAT_00a0aa20).cast::<u8>();
        let fc80 = (&raw mut DAT_00a0fc80).cast::<u32>();
        let a910 = (&raw mut DAT_00a0a910).cast::<u16>();

        let mut i_var5: u32 = 1;
        if *a880.add(param_1.wrapping_add(1) as usize) != 0 {
            let mut u_var2 = param_1;
            loop {
                if (DAT_009a20a4 & 0xFF) <= u_var2.wrapping_add(1) {
                    break;
                }
                let idx = u_var2.wrapping_add(2) as usize;
                i_var5 = i_var5.wrapping_add(1);
                u_var2 = u_var2.wrapping_add(1);
                if *a880.add(idx) == 0 {
                    break;
                }
            }
        }
        let b_var6: u8 = (DAT_009a20a4 as u8).wrapping_sub(i_var5 as u8);
        DAT_009a20a4 = (DAT_009a20a4 & 0xFFFFFF00) | b_var6 as u32;
        if param_1 < b_var6 as u32 {
            let mut p4 = fc80.add(param_1.wrapping_add(i_var5) as usize);
            let mut p3 = fc80.add(param_1 as usize);
            let mut p7 = a910.add(param_1 as usize);
            let mut p8 = a910.add(param_1.wrapping_add(i_var5) as usize);
            loop {
                p3.write_unaligned(p4.read_unaligned());
                p4 = p4.add(1);
                let s = i_var5.wrapping_add(param_1) as usize;
                *aa20.add(param_1 as usize) = *aa20.add(s);
                p3 = p3.add(1);
                *aaa0.add(param_1 as usize) = *aaa0.add(s);
                *a880.add(param_1 as usize) = *a880.add(s);
                param_1 = param_1.wrapping_add(1);
                p7.write_unaligned(p8.read_unaligned());
                p7 = p7.add(1);
                p8 = p8.add(1);
                if param_1 >= b_var6 as u32 {
                    break;
                }
            }
        }
    }
}
