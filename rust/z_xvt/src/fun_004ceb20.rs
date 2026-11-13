// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_008050aa: u32;
    static mut DAT_008050ae: i32;
    static mut DAT_007810b5: i8;
    static mut DAT_007810b4: i8;
    static mut DAT_0081b5be: u32;
    static mut DAT_00815ac6: i32;
    static mut DAT_007810a6: i32;
    static mut DAT_007810b3: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ceb20(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    // SAFETY: the globals are the game's own memory; the table accesses
    // are at the same offsets the original computes.
    unsafe {
        let mut u_var1: u32 = DAT_008050aa;
        let mut i_var4: i32 = DAT_008050ae;
        let _ = param_1;
        loop {
            if i_var4 == 0 {
                return 0xffff_ffff;
            }
            let i_var2: i32 = (u_var1 as i32).wrapping_mul(0x210);
            let off = i_var2 as isize as usize;
            let b5 = (&raw mut DAT_007810b5)
                .cast::<u8>()
                .byte_add(off)
                .cast::<i8>()
                .read();
            if b5 != 0 {
                let u_var5: u32 = (&raw mut DAT_007810b4).cast::<u8>().byte_add(off).read() as u32;
                let mut u_var3: u32 = 0;
                if DAT_0081b5be != 0 {
                    let mut pi_var6: *mut i32 = &raw mut DAT_00815ac6;
                    let idx = u_var1.wrapping_mul(0x84);
                    let key = (&raw mut DAT_007810a6)
                        .cast::<u8>()
                        .byte_add(idx.wrapping_mul(4) as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    loop {
                        if pi_var6.read_unaligned() == key {
                            break;
                        }
                        pi_var6 = pi_var6.wrapping_add(0x8e);
                        u_var3 = u_var3.wrapping_add(1);
                        if !(u_var3 < DAT_0081b5be) {
                            break;
                        }
                    }
                }
                if param_5 == u_var3 {
                    let b3 = (&raw mut DAT_007810b3)
                        .cast::<u8>()
                        .byte_add(off)
                        .cast::<i8>()
                        .read();
                    if param_3 == 0 {
                        if param_4 == 0 {
                            if b3 != 0 && b3 != 2 && param_2 == u_var5 {
                                return u_var1;
                            }
                        } else if b3 == 2 && param_2 == u_var5 {
                            return u_var1;
                        }
                    } else if b3 == 0 && param_2 == u_var5 {
                        return u_var1;
                    }
                }
            }
            u_var1 = u_var1.wrapping_add(1);
            if 0x3ff < u_var1 {
                u_var1 = 0;
            }
            i_var4 = i_var4.wrapping_sub(1);
        }
    }
}
