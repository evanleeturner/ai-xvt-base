// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d1308: u32;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009e9640: i32;
    static mut DAT_00a00854: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08104: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e967c: [u8; 0];
    static mut DAT_00521f2e: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009fd458: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(param_1: u16) -> u32;
}

#[inline(always)]
unsafe fn addr(a: i32) -> *const u8 {
    (a as u32 as usize) as *const u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004823e0(
    param_1: u16,
    param_2: i16,
    param_3: i32,
    param_4: u32,
    param_5: u32,
) -> u32 {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // at the offsets and in the order the original uses.
    unsafe {
        let mut local_2: i16 = DAT_009d1308.wrapping_add(DAT_00a07ce8) as i16;
        let mut u_var9: u32 = param_1 as u32;
        'outer: loop {
            let mut pc_var7: u32;
            let mut i_var6: i32;
            loop {
                let s_var2 = local_2.wrapping_sub(1);
                if local_2 == 0 {
                    return 0xffff;
                }
                let u_var8: u16 = (u_var9 as u16).wrapping_add(param_2 as u16);
                u_var9 = u_var8 as u32;
                if u_var8 < 0x8000 {
                    if DAT_009d1308.wrapping_add(DAT_00a07ce8) == u_var8 as u32 {
                        u_var9 = 0;
                    }
                } else {
                    u_var9 = DAT_009d1308
                        .wrapping_sub(1)
                        .wrapping_add(DAT_00a07ce8 & 0xffff);
                }
                i_var6 = (param_3 as u32).wrapping_mul(0x5bd) as i32;
                pc_var7 = u_var9 & 0xffff;
                local_2 = s_var2;
                let local_i32: i32 = (&raw mut DAT_009e9670)
                    .cast::<u8>()
                    .offset(i_var6 as isize)
                    .cast::<i32>()
                    .read_unaligned();
                let p = pc_var7 as i32;
                if local_i32 as u32 == pc_var7
                    || ((param_5 & 4) != 0 && DAT_009e9640 <= p && p < DAT_00a00854)
                    || ((param_5 & 2) != 0 && DAT_00a00854 <= p)
                {
                    continue;
                }
                break;
            }
            let i_var5: i32 = pc_var7.wrapping_mul(0x23) as i32;
            let i_var1: i32 = DAT_009a1fe8.wrapping_add(i_var5);
            let b3: u8 = addr(DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var5)).read();
            if b3 == 0
                || ((&raw mut DAT_00521f2e)
                    .cast::<u8>()
                    .add((b3 as usize).wrapping_mul(0x1a))
                    .read()
                    & 1)
                    == 0
                || ((param_5 & 1) != 0 && addr(i_var1.wrapping_add(2)).read() == 8)
            {
                continue 'outer;
            }

            let local_i32: i32 = addr(i_var1.wrapping_add(0x1f))
                .cast::<i32>()
                .read_unaligned();
            let b: u8 = if local_i32 == 0 {
                let idx = (addr(i_var1.wrapping_add(0x16)).read() as usize).wrapping_mul(0x566);
                (&raw mut DAT_009d8c88).cast::<u8>().add(idx).read()
            } else {
                addr(local_i32.wrapping_add(0x89)).read()
            };
            pc_var7 = b as u32;
            let local_u16: u16 = (&raw mut DAT_009e967c)
                .cast::<u8>()
                .offset(i_var6 as isize)
                .cast::<u16>()
                .read_unaligned();
            match param_4 {
                1 => {
                    if local_u16 as u32 != pc_var7 {
                        continue 'outer;
                    }
                }
                2 | 3 => {
                    let b10 = if local_u16 as u32 == pc_var7 {
                        false
                    } else {
                        let idx = (pc_var7 as usize)
                            .wrapping_add((local_u16 as usize).wrapping_mul(0x1e5));
                        (&raw mut DAT_009fd458).cast::<u8>().add(idx).read() == 0
                    };
                    if param_4 == 2 {
                        if b10 {
                            continue 'outer;
                        }
                    } else if !b10 {
                        continue 'outer;
                    }
                }
                4 => {
                    let l: i32 = addr(i_var1.wrapping_add(0x1b))
                        .cast::<i32>()
                        .read_unaligned();
                    if l == -1 {
                        continue 'outer;
                    }
                }
                _ => {}
            }
            if local_i32 == 0 {
                return u_var9 as u16 as u32;
            }
            if addr(i_var1.wrapping_add(2)).read() != 0x0d
                && (FUN_00484f80(u_var9 as u16) as u8) == 0
            {
                let local_ptr: *const u8 =
                    addr(DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var5))
                        .cast::<*const u8>()
                        .read_unaligned();
                if local_ptr.read() != 0 {
                    return u_var9 as u16 as u32;
                }
                DAT_00a08104 = local_ptr.add(0xa9).cast::<i32>().read_unaligned();
                let c = addr(DAT_00a08104.wrapping_add(7)).read();
                if c != 3 && c != 4 {
                    return u_var9 as u16 as u32;
                }
            }
        }
    }
}
