// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4980: u32;
    static mut DAT_006d4988: u32;
    static mut DAT_006d9d56: u32;
    static mut DAT_00732e54: u32;
    static mut DAT_00525bf7: [u8; 1];
    static mut DAT_0052b0b4: [u8; 1];
    static mut DAT_0052b0b6: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00455d80() -> u32 {
    // SAFETY: all globals and the memory they point at belong to the game;
    // reads happen in the original's order with its offsets.
    unsafe {
        let idx: u16 = ((DAT_006d9d56.wrapping_add(0x18)) as *const u16).read_unaligned();
        let c_var4: u8;
        if (idx as i32) < (DAT_006d4980 as i32) {
            let c = *((u32::from(idx)
                .wrapping_mul(0x23)
                .wrapping_add(2)
                .wrapping_add(DAT_006d4988)) as *const u8);
            if c == 3 || c == 5 || c == 4 {
                c_var4 = 2;
            } else {
                c_var4 = 1;
            }
        } else {
            c_var4 = 1;
        }

        let mut u_var6: u32 = 0;
        let b_var1: u8 = *((DAT_00732e54.wrapping_add(0x228)) as *const u8);
        let mut u_var3: u32 = (DAT_00732e54 & 0xFFFF_FF00) | u32::from(b_var1);

        if b_var1 != 0 {
            loop {
                let t = *((DAT_00732e54.wrapping_add(0x229).wrapping_add(u_var6)) as *const u8);
                let tbl = (&raw const DAT_00525bf7).cast::<u8>();
                if *tbl.add(t as usize) == c_var4 {
                    let i_var2: u32 = u32::from(*((DAT_00732e54.wrapping_add(4)) as *const u8))
                        .wrapping_mul(0xef);
                    let p4 = (&raw const DAT_0052b0b4).cast::<u8>();
                    let p6 = (&raw const DAT_0052b0b6).cast::<u8>();
                    u_var3 = u32::from(*p4.add(i_var2.wrapping_add(u_var6) as usize));
                    let mut u_var5: u16 = u_var3 as u16;
                    while u32::from(u_var5)
                        <= u32::from(*p6.add(u_var6.wrapping_add(i_var2) as usize))
                    {
                        let b = *((DAT_00732e54
                            .wrapping_add(0x346)
                            .wrapping_add(u_var3.wrapping_mul(4)))
                            as *const u8);
                        if b != 0 {
                            return 1;
                        }
                        u_var5 = (u_var3.wrapping_add(1)) as u16;
                        u_var3 = u32::from(u_var5);
                    }
                }
                u_var3 = u_var6.wrapping_add(1);
                u_var6 = u_var3;
                if !(u_var3 < u32::from(b_var1)) {
                    break;
                }
            }
        }
        u_var3 & 0xFFFF_0000
    }
}
