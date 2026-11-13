// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b730: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00482d90(param_1: u16, param_2: i16) -> u32 {
    let mut ab: [u8; 12] = [0; 12];
    let abp = ab.as_mut_ptr();
    // SAFETY: the globals and the structures they point to belong to the game;
    // reads and writes happen at the offsets the original uses, including its
    // unchecked index into the stack array.
    unsafe {
        let base = (&raw mut DAT_0071b730) as usize as i32;
        let p0 = base.wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd));
        let a = (p0 as usize as *const i32).read_unaligned();
        let p1 = a
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988);
        let b = (p1 as usize as *const i32).read_unaligned();
        let i_var2: i32 = (b.wrapping_add(0xa7) as usize as *const i32).read_unaligned();

        let ent = |idx: u8| -> i16 {
            (i_var2
                .wrapping_add(0x29e)
                .wrapping_add((idx as i32).wrapping_mul(2)) as usize as *const i16)
                .read_unaligned()
        };

        let mut i_var3: i32 = 0;
        loop {
            let v =
                ((i_var3.wrapping_add(0x293).wrapping_add(i_var2)) as usize as *const u8).read();
            abp.add(v as usize).write(i_var3 as u8);
            i_var3 = i_var3.wrapping_add(1);
            if i_var3 >= 10 {
                break;
            }
        }

        let mut u_var7: u16 = 0xffff;
        let mut i_var6: i32 = 0;
        let u_var5: u16 = ((i_var3 as u32) >> 0x10) as u16;

        loop {
            let b_var1 = abp.add(i_var6 as usize).read();
            if ent(b_var1) == 0 {
                if param_1 as u32 == b_var1 as u32 {
                    if param_2 != -1 {
                        loop {
                            let mut u_var4 = (((u_var5 as u32) << 16) | param_1 as u32) >> 8;
                            if 9 < i_var6.wrapping_add(1) {
                                let mut j: i32 = 0;
                                loop {
                                    let e = abp.add(j as usize).read();
                                    if ent(e) != 0 {
                                        return ((u_var4 << 8) | e as u32) & 0xffff00ff;
                                    }
                                    j = j.wrapping_add(1);
                                    if j >= 10 {
                                        break;
                                    }
                                }
                                j = 0;
                                loop {
                                    let e = abp.add(j as usize).read();
                                    if ent(e) == 0 {
                                        return ((u_var4 << 8) | e as u32) & 0xffff00ff;
                                    }
                                    j = j.wrapping_add(1);
                                    if j >= 10 {
                                        break;
                                    }
                                }
                                return ((u_var5 as u32) << 16) | param_1 as u32;
                            }
                            let k = i_var6.wrapping_add(1);
                            i_var6 = i_var6.wrapping_add(1);
                            let e = abp.add(k as usize).read();
                            if ent(e) == 0 {
                                u_var4 = (((u_var5 as u32) << 16) | param_1 as u32) >> 8;
                                return ((u_var4 << 8) | e as u32) & 0xffff00ff;
                            }
                        }
                    }
                    if u_var7 != 0xffff {
                        return ((u_var5 as u32) << 16) | u_var7 as u32;
                    }
                    let mut j: i32 = 9;
                    loop {
                        let e = abp.add(j as usize).read();
                        if ent(e) != 0 {
                            return ((((j as u32) >> 8) << 8) | e as u32) & 0xffff00ff;
                        }
                        j = j.wrapping_sub(1);
                        if !(-1 < j) {
                            break;
                        }
                    }
                    return ((((j as u32) >> 8) << 8) | abp.add(9).read() as u32) & 0xffff00ff;
                }
                u_var7 = b_var1 as u16;
            }
            i_var6 = i_var6.wrapping_add(1);
            if 9 < i_var6 {
                return (u_var5 as u32) << 0x10;
            }
        }
    }
}
