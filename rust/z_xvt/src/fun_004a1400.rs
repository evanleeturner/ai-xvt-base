// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i8;
    static mut DAT_0070b252: i8;
    static mut DAT_0070ad49: i8;
    static mut DAT_0071badb: i8;
    static mut DAT_0071b738: i8;
    static mut DAT_0052ad08: i8;
    static mut DAT_0071baa9: i8;
    static mut DAT_006d4a02: i8;
    static mut DAT_0071b7a1: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478420(a: u32, b: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a1400(param_1: i32, param_2: i32, param_3: i32) {
    if param_3 != 2 && param_3 != 3 {
        return;
    }
    // SAFETY: all globals and the pointed-to structures are the game's own
    // memory, accessed at the offsets the original uses.
    unsafe {
        let mut i_var5: i32 = param_2.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as i32);
        let mut i_var6: i32 = ((i_var5.wrapping_add(0x1b)) as usize as *const i32).read_unaligned();
        if i_var6 == -1 {
            let idx = ((i_var5.wrapping_add(0x16)) as usize as *const u8).read() as u32;
            i_var6 = (&raw mut DAT_0070b252)
                .wrapping_byte_offset(idx.wrapping_mul(0x566) as i32 as isize)
                .cast::<i32>()
                .read_unaligned();
            if i_var6 == -1 {
                return;
            }
        }
        let mut i_var7: i32 = 0;
        let p = ((i_var5.wrapping_add(0x1f)) as usize as *const i32).read_unaligned();
        i_var5 = ((p.wrapping_add(0xa7)) as usize as *const i32).read_unaligned();
        let a = ((i_var5.wrapping_add(0xd1)) as usize as *const u32).read_unaligned();
        let b = ((i_var5.wrapping_add(0xcd)) as usize as *const u32).read_unaligned();
        let u_var3 = FUN_00478420(a, b);
        if (u_var3 as u16) < 0x8000 {
            return;
        }
        i_var5 = (&raw mut DAT_0070b252)
            .wrapping_byte_offset(param_1.wrapping_mul(0x566) as isize)
            .cast::<i32>()
            .read_unaligned();
        let row6 = i_var6.wrapping_mul(0x5bd);
        'skip: {
            if i_var5 == -1 {
                let b2 = (&raw mut DAT_0070ad49)
                    .wrapping_byte_offset(param_1.wrapping_mul(0x566) as isize)
                    .cast::<u8>()
                    .read() as u32;
                if param_3 == 3 {
                    let q = (&raw mut DAT_0071badb)
                        .wrapping_byte_offset(
                            (b2.wrapping_mul(2) as i32).wrapping_add(row6) as isize
                        )
                        .cast::<i16>();
                    q.write_unaligned(q.read_unaligned().wrapping_add(1));
                }
                let u3 = (&raw mut DAT_0071b738)
                    .wrapping_byte_offset(row6 as isize)
                    .cast::<u16>()
                    .read_unaligned() as u32;
                let t = (&raw mut DAT_0052ad08)
                    .wrapping_byte_offset(b2.wrapping_mul(4) as i32 as isize)
                    .cast::<i32>()
                    .read_unaligned();
                if (u3 as i32) <= t.wrapping_add(4) {
                    break 'skip;
                }
                i_var7 = u3.wrapping_sub(t as u32) as i32;
            } else {
                let u3 = (&raw mut DAT_0071b738)
                    .wrapping_byte_offset(i_var5.wrapping_mul(0x5bd) as isize)
                    .cast::<u16>()
                    .read_unaligned() as u32;
                if param_3 == 3 {
                    let q = (&raw mut DAT_0071baa9)
                        .wrapping_byte_offset(
                            (u3.wrapping_mul(2) as i32).wrapping_add(row6) as isize
                        )
                        .cast::<i16>();
                    q.write_unaligned(q.read_unaligned().wrapping_add(1));
                }
                let u4 = (&raw mut DAT_0071b738)
                    .wrapping_byte_offset(row6 as isize)
                    .cast::<u16>()
                    .read_unaligned() as u32;
                if u4 <= u3.wrapping_add(4) {
                    break 'skip;
                }
                i_var7 = u4.wrapping_sub(u3) as i32;
            }
            i_var7 = i_var7.wrapping_mul(2);
            if param_3 == 2 {
                i_var7 /= 2;
            }
        }
        if DAT_006d4a02 != 2 {
            i_var7 = i_var7.wrapping_add((((i_var7 >> 31) as u32) & 3) as i32) >> 2;
        }
        let q = (&raw mut DAT_0071b7a1)
            .wrapping_byte_offset(row6 as isize)
            .cast::<i32>();
        q.write_unaligned(q.read_unaligned().wrapping_sub(i_var7));
    }
}
