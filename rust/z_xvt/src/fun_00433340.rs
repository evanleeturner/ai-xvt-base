// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005251b0: u32;
    static mut DAT_005251b4: i32;
    static mut DAT_00539a17: i8;
    static mut DAT_00539a16: i8;
    static mut DAT_00739290: u32;
    static mut DAT_00733798: i32;
    static mut DAT_00539a08: u32;
    static mut DAT_00539a15: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00433340(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    let _ = param_1;
    // SAFETY: the globals are the game's own tables; offsets are computed
    // exactly as the original does.
    unsafe {
        let mut u_var1: u32 = DAT_005251b0;
        let mut i_var4: i32 = DAT_005251b4;
        loop {
            if i_var4 == 0 {
                return 0xffff_ffff;
            }
            let i_var2: i32 = u_var1.wrapping_mul(0x210) as i32;
            let p17 = (&raw mut DAT_00539a17).byte_offset(i_var2 as isize);
            if p17.read() != 0 {
                let p16 = (&raw mut DAT_00539a16).byte_offset(i_var2 as isize);
                let p15 = (&raw mut DAT_00539a15).byte_offset(i_var2 as isize);
                let u_var5: u32 = u32::from(p16.cast::<u8>().read());
                let mut u_var3: u32 = 0;
                if DAT_00739290 != 0 {
                    let mut pi_var6: *mut i32 = &raw mut DAT_00733798;
                    let key = (&raw mut DAT_00539a08)
                        .add(u_var1.wrapping_mul(0x84) as usize)
                        .read_unaligned();
                    loop {
                        if (pi_var6.read_unaligned() as u32) == key {
                            break;
                        }
                        pi_var6 = pi_var6.wrapping_add(0x8e);
                        u_var3 = u_var3.wrapping_add(1);
                        if !(u_var3 < DAT_00739290) {
                            break;
                        }
                    }
                }
                if u_var3 == param_5 {
                    let c15 = p15.read();
                    if param_3 == 0 {
                        if param_4 == 0 {
                            if c15 != 0 && c15 != 2 && u_var5 == param_2 {
                                return u_var1;
                            }
                        } else if c15 == 2 && u_var5 == param_2 {
                            return u_var1;
                        }
                    } else if c15 == 0 && u_var5 == param_2 {
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
