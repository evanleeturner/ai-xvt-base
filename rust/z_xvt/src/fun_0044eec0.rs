// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4820: [i8; 1];
    static mut DAT_006d9d78: i8;
    static mut unaff_ESI: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f010(param_1: u32, param_2: u16, param_3: u16) -> i8;
    fn FUN_00429a20(param_1: u32) -> i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044eec0(
    param_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u32,
) {
    // SAFETY: the globals and callees are the game's own memory and code; the
    // pointers dereferenced are the game's tables, read and written where the
    // original does.
    unsafe {
        let mut u_var5: u32 = (DAT_00732a3c as u32) | (unaff_ESI & 0xffff_0000);
        if (DAT_00732a3c as i32) < DAT_006d4980 {
            loop {
                let i_var1 = DAT_006d4988;
                let u_var6 = (u_var5 & 0xffff) as u16;
                let idx = (u_var6 as u32)
                    .wrapping_mul(2)
                    .wrapping_add(param_6 & 0xffff);
                (&raw mut DAT_006d4820)
                    .cast::<i8>()
                    .byte_add(idx as usize)
                    .write(0);
                let off = (u_var6 as i32).wrapping_mul(0x23);
                let p = i_var1.wrapping_add(3).wrapping_add(off) as *const i8;
                if p.read() != 0 {
                    let b2 = FUN_0049f010(u_var5, param_1, param_2);
                    let mut u_var4 = b2 as u16;
                    let b3 = FUN_0049f010(u_var5, param_4, param_5);
                    let u_var3 = b3 as u16;
                    if param_3 == 1 {
                        u_var4 |= u_var3;
                    } else {
                        u_var4 &= u_var3;
                    }
                    if u_var4 != 0 {
                        let mut ok = DAT_006d9d78 == 0;
                        if !ok {
                            let a = DAT_006d4988.wrapping_add(0x1f).wrapping_add(off) as *const i32;
                            let b = a.read_unaligned().wrapping_add(0xa7) as *const i32;
                            let c = b.read_unaligned().wrapping_add(0x1df) as *const i16;
                            ok = c.read_unaligned() != 0;
                        }
                        if ok {
                            let b4 = FUN_00429a20(u_var6 as u32);
                            if b4 != 0 {
                                (&raw mut DAT_006d4820)
                                    .cast::<i8>()
                                    .byte_add(idx as usize)
                                    .write(1);
                            }
                        }
                    }
                }
                let u_var4 = (u_var5 as u16).wrapping_add(1);
                u_var5 = (u_var5 & 0xffff_0000) | (u_var4 as u32);
                if !((u_var4 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
    }
}
