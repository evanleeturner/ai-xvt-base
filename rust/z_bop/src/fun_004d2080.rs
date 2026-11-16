// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b53575: u32;
    static mut DAT_00b53579: i32;
    static mut DAT_00acf580: [i8; 1];
    static mut DAT_00acf57f: [i8; 1];
    static mut DAT_00acf57e: [i8; 1];
    static mut DAT_00b69a89: u32;
    static mut DAT_00b63f91: [i32; 1];
    static mut DAT_00acf571: [i32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2080(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    let _ = param_1;
    // SAFETY: all globals are the game's own tables, read where the original
    // reads them; indices stay within the game's 0x400-entry table.
    unsafe {
        let mut u_var3: u32 = DAT_00b53575;
        let mut i_var1: i32 = DAT_00b53579;
        loop {
            if i_var1 == 0 {
                return 0xffff_ffff;
            }
            let i_var4: i32 = u_var3.wrapping_mul(0x210) as i32;
            let base80 = (&raw mut DAT_00acf580).cast::<i8>();
            if base80.offset(i_var4 as isize).read() != 0 {
                let u_var5: u32 = u32::from(
                    (&raw mut DAT_00acf57f)
                        .cast::<i8>()
                        .offset(i_var4 as isize)
                        .read() as u8,
                );
                let c_var2: i8 = (&raw mut DAT_00acf57e)
                    .cast::<i8>()
                    .offset(i_var4 as isize)
                    .read();
                let mut u_var7: u32 = 0;
                if DAT_00b69a89 != 0 {
                    let mut pi_var6 = (&raw mut DAT_00b63f91).cast::<i32>();
                    loop {
                        let key = (&raw mut DAT_00acf571)
                            .cast::<i32>()
                            .add(u_var3.wrapping_mul(0x84) as usize)
                            .read_unaligned();
                        if pi_var6.read_unaligned() == key {
                            break;
                        }
                        pi_var6 = pi_var6.wrapping_add(0x8e);
                        u_var7 = u_var7.wrapping_add(1);
                        if !(u_var7 < DAT_00b69a89) {
                            break;
                        }
                    }
                }
                if u_var7 == param_5 {
                    if param_3 == 0 {
                        if param_4 == 0 {
                            if c_var2 != 0 && c_var2 != 2 && u_var5 == param_2 {
                                return u_var3;
                            }
                        } else if c_var2 == 2 && u_var5 == param_2 {
                            return u_var3;
                        }
                    } else if c_var2 == 0 && u_var5 == param_2 {
                        return u_var3;
                    }
                }
            }
            u_var3 = u_var3.wrapping_add(1);
            if 0x3ff < u_var3 {
                u_var3 = 0;
            }
            i_var1 = i_var1.wrapping_sub(1);
        }
    }
}
