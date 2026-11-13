// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527028: u32;
    static mut DAT_0052702c: i32;
    static mut DAT_005599b7: [i8; 1];
    static mut DAT_005599b6: [i8; 1];
    static mut DAT_0099f6e0: u32;
    static mut DAT_00999be8: [i32; 1];
    static mut DAT_005599a8: [i32; 1];
    static mut DAT_005599b5: [i8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046fce0(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    let _ = param_1;
    // SAFETY: all globals are the game's own memory; the tables are indexed
    // exactly as the original does.
    unsafe {
        let mut u_var1: u32 = DAT_00527028;
        let mut i_var4: i32 = DAT_0052702c;
        let b7 = (&raw mut DAT_005599b7).cast::<i8>();
        let b6 = (&raw mut DAT_005599b6).cast::<i8>();
        let b5 = (&raw mut DAT_005599b5).cast::<i8>();
        let a8 = (&raw mut DAT_005599a8).cast::<i32>();
        let base = (&raw mut DAT_00999be8).cast::<i32>();
        loop {
            if i_var4 == 0 {
                return 0xffffffff;
            }
            let i_var2: i32 = (u_var1 as i32).wrapping_mul(0x210);
            if *b7.offset(i_var2 as isize) != 0 {
                let u_var5: u32 = *b6.offset(i_var2 as isize) as u8 as u32;
                let mut u_var3: u32 = 0;
                if DAT_0099f6e0 != 0 {
                    let mut p = base;
                    loop {
                        let target = a8.add(u_var1.wrapping_mul(0x84) as usize).read_unaligned();
                        if p.read_unaligned() == target {
                            break;
                        }
                        p = p.add(0x8e);
                        u_var3 = u_var3.wrapping_add(1);
                        if !(u_var3 < DAT_0099f6e0) {
                            break;
                        }
                    }
                }
                if u_var3 == param_5 {
                    let c = *b5.offset(i_var2 as isize);
                    if param_3 == 0 {
                        if param_4 == 0 {
                            if c != 0 && c != 2 && u_var5 == param_2 {
                                return u_var1;
                            }
                        } else if c == 2 && u_var5 == param_2 {
                            return u_var1;
                        }
                    } else if c == 0 && u_var5 == param_2 {
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
