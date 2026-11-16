// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703c88: u32;
    static mut DAT_00732a38: u32;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0071b73c: [u8; 1];
    static mut DAT_0071b704: i32;
    static mut DAT_00732904: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_0052687a: [u8; 1];
    static mut DAT_0070ad48: [u8; 1];
    static mut DAT_0072f508: [i8; 1];
    static mut DAT_00732e54: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00441d10(
    param_1: u16,
    param_2: i16,
    param_3: i32,
    param_4: u32,
    param_5: u32,
) -> u32 {
    // SAFETY: all globals and the pointers derived from them are the game's
    // own memory, read at the offsets the original uses.
    unsafe {
        let s_var4: i16 = (DAT_00703c88 as u16).wrapping_add(DAT_00732a38 as u16) as i16;
        let mut u_var6: u32 = (((DAT_00703c88 >> 16) & 0xffff) << 16) | (param_1 as u32);
        let mut local_6: i16 = s_var4;

        'top: loop {
            let mut u_var10: u32;
            let mut i_var3: u32;
            loop {
                let s_var1 = local_6.wrapping_sub(1);
                let u_var7 = (u_var6 >> 16) as u16;
                if local_6 == 0 {
                    return ((u_var7 as u32) << 16) | 0xffff;
                }
                let u_var5 = (u_var6 as u16).wrapping_add(param_2 as u16);
                u_var6 = ((u_var7 as u32) << 16) | (u_var5 as u32);
                if u_var5 < 0x8000 {
                    if DAT_00703c88.wrapping_add(DAT_00732a38) == u_var5 as u32 {
                        u_var6 = (u_var7 as u32) << 16;
                    }
                } else {
                    u_var6 = ((u_var7 as u32) << 16) | (s_var4.wrapping_sub(1) as u16 as u32);
                }
                u_var10 = u_var6 & 0xffff;
                i_var3 = (param_3 as u32).wrapping_mul(0x5bd);
                local_6 = s_var1;
                let v32 = (&raw mut DAT_0071b730)
                    .cast::<u8>()
                    .byte_add(i_var3 as usize)
                    .cast::<u32>()
                    .read_unaligned();
                let c = v32 == u_var10
                    || ((param_5 & 4) != 0
                        && DAT_0071b704 <= (u_var10 as i32)
                        && (u_var10 as i32) < DAT_00732904)
                    || ((param_5 & 2) != 0 && DAT_00732904 <= (u_var10 as i32));
                if !c {
                    break;
                }
            }
            let i_var8: *mut u8 =
                u_var10.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32) as usize as *mut u8;
            let b3 = i_var8.byte_add(3).read() as u32;
            if b3 == 0
                || ((&raw mut DAT_0052687a)
                    .cast::<u8>()
                    .byte_add(b3.wrapping_mul(0x16) as usize)
                    .read()
                    & 1)
                    == 0
                || ((param_5 & 1) != 0 && i_var8.byte_add(2).read() == 8)
            {
                continue 'top;
            }

            let ent: *mut u8 =
                i_var8.byte_add(0x1f).cast::<u32>().read_unaligned() as usize as *mut u8;
            let f1b: u32 = i_var8.byte_add(0x1b).cast::<u32>().read_unaligned();
            let v16: u16 = (&raw mut DAT_0071b73c)
                .cast::<u8>()
                .byte_add(i_var3 as usize)
                .cast::<u16>()
                .read_unaligned();

            let b_var2: u8 = if ent.is_null() {
                (&raw mut DAT_0070ad48)
                    .cast::<u8>()
                    .byte_add((i_var8.byte_add(0x16).read() as u32).wrapping_mul(0x566) as usize)
                    .read()
            } else {
                ent.byte_add(0x87).read()
            };
            let u_var9 = b_var2 as u32;

            match param_4 {
                1 => {
                    if v16 as u32 != u_var9 {
                        continue 'top;
                    }
                }
                2 | 3 => {
                    let b11 = if u_var9 == v16 as u32 {
                        false
                    } else {
                        (&raw mut DAT_0072f508)
                            .cast::<i8>()
                            .byte_add(u_var9.wrapping_add((v16 as u32).wrapping_mul(0x1e5)) as usize)
                            .read()
                            == 0
                    };
                    if param_4 == 2 {
                        if b11 {
                            continue 'top;
                        }
                    } else if !b11 {
                        continue 'top;
                    }
                }
                4 => {
                    if f1b == 0xffff_ffff {
                        continue 'top;
                    }
                }
                _ => {}
            }

            if ent.is_null() {
                return u_var6;
            }
            if i_var8.byte_add(2).read() != 0x0d {
                let mut b11 = false;
                if (u_var6 as u16 as i16) != -1
                    && DAT_006d4980 > (u_var10 as i32)
                    && f1b != 0xffff_ffff
                {
                    let q: *mut u8 =
                        ent.byte_add(0xa7).cast::<u32>().read_unaligned() as usize as *mut u8;
                    if !q.is_null()
                        && (q.byte_add(0x1e0).read() & 1) != 0
                        && q.byte_add(0x237).read() != 0
                        && q.byte_add(0x233).read() == 3
                    {
                        b11 = true;
                        let s238 = q.byte_add(0x238).cast::<u16>().read_unaligned();
                        if s238 == 0 {
                            b11 = false;
                        }
                    }
                }
                if !b11 {
                    if ent.read() != 0 {
                        return u_var6;
                    }
                    DAT_00732e54 = ent.byte_add(0xa7).cast::<i32>().read_unaligned();
                    let t = (DAT_00732e54 as u32).wrapping_add(7) as usize as *const i8;
                    let tv = t.read();
                    if tv != 3 && tv != 4 {
                        return u_var6;
                    }
                }
            }
        }
    }
}
