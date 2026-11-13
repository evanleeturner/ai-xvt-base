// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070aee5: [u8; 1];
    static mut DAT_0070aee6: [u8; 1];
    static mut DAT_0070aee7: [u8; 1];
    static mut DAT_0070aee8: [u8; 1];
    static mut DAT_0070aee9: [u8; 1];
    static mut DAT_0070aef3: [u8; 1];
    static mut DAT_0071f2d8: u8;
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2dd: u8;
    static mut DAT_007092e8: [i32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a17b0(param_1: u16, param_2: u32, param_3: i32) {
    // SAFETY: the globals are the game's own memory; indices follow the original's
    // record layout (0x566 stride per entry, 0x4E per step).
    unsafe {
        let p5 = (&raw mut DAT_0070aee5).cast::<u8>();
        let p6 = (&raw mut DAT_0070aee6).cast::<u8>();
        let p7 = (&raw mut DAT_0070aee7).cast::<u8>();
        let p8 = (&raw mut DAT_0070aee8).cast::<u8>();
        let p9 = (&raw mut DAT_0070aee9).cast::<u8>();
        let pf3 = (&raw mut DAT_0070aef3).cast::<u8>();
        let tbl = (&raw mut DAT_007092e8).cast::<i32>();

        let mut i_var2: i32 = ((param_2 & 0xFFFF) as i32).wrapping_mul(0x566);
        let mut i_var1: i32 = 8;
        // The C leaves this uninitialized until first assigned; it persists across iterations.
        let mut local_4: i32 = 0;
        loop {
            let o = i_var2 as isize;
            if *p5.offset(o) == 0x02
                && u16::from(*p6.offset(o)) == param_1
                && (*p7.offset(o) == 0x12 || (*p7.offset(o) == 0x13 && param_3 == 1))
            {
                let lim = u32::from(*pf3.offset(o)).wrapping_mul(5);
                let now = u32::from(DAT_0071f2d8)
                    .wrapping_mul(0x3C)
                    .wrapping_add(u32::from(DAT_0071f2dc))
                    .wrapping_mul(0x3C)
                    .wrapping_add(u32::from(DAT_0071f2dd));
                if lim == 0 || now <= lim {
                    local_4 = (*p8.offset(o) as i8) as i32;
                }

                let mut i_var3: i32 = 0;
                local_4 = local_4.wrapping_mul(0xFA);
                loop {
                    if *p9.offset(i_var3.wrapping_add(i_var2) as isize) != 0 {
                        let t = tbl.offset(i_var3 as isize);
                        t.write(t.read().wrapping_add(local_4));
                    }
                    i_var3 = i_var3.wrapping_add(1);
                    if i_var3 >= 10 {
                        break;
                    }
                }
            }
            i_var2 = i_var2.wrapping_add(0x4E);
            i_var1 -= 1;
            if i_var1 == 0 {
                break;
            }
        }
    }
}
