// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a2062: i8;
    static mut DAT_009d6990: [i32; 8];
    static mut DAT_009d6968: [i32; 8];
    static mut DAT_00a07cec: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009fd458: [u8; 0];
    static mut DAT_009d6959: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(a: u16) -> u32;
    fn FUN_00482eb0(a: u16, b: u16, c: u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00482be0() {
    // SAFETY: all globals and pointers are the game's own memory; reads
    // happen at the offsets and in the order the original uses.
    unsafe {
        let mut local_4: i32 = 10;
        let mut local_10: i32 = 0;
        if DAT_009a2062 == 2 {
            let mut i_var4: i32 = 0;
            local_10 = 0;
            let a = (&raw const DAT_009d6968).cast::<i32>();
            let b = (&raw const DAT_009d6990).cast::<i32>();
            for i in 0..8usize {
                let s = (a.add(i).read() as u32).wrapping_add(b.add(i).read() as u32) as i32;
                if local_10 < s {
                    local_10 = s;
                    local_4 = i_var4;
                }
                i_var4 = i_var4.wrapping_add(1);
            }
        }
        if DAT_00a07cec < DAT_009a1fe0 {
            let mut local_c: u32 = (DAT_00a07cec as u32).wrapping_mul(0x23);
            let mut u_var6: u32 = DAT_00a07cec as u32;
            loop {
                'body: {
                    let p_entry = (DAT_009a1fe8 as u32).wrapping_add(local_c) as *const u8;
                    if p_entry.byte_add(3).read() == 0 {
                        break 'body;
                    }
                    let p_player = (&raw mut DAT_009e9670)
                        .cast::<u8>()
                        .byte_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize);
                    let pl0 = p_player.cast::<u32>().read_unaligned();
                    if pl0 == u_var6 {
                        break 'body;
                    }
                    let mut b_var5: u8 = 0;
                    let p1f = p_entry.byte_add(0x1f).cast::<u32>().read_unaligned() as *const u8;
                    let pl_c = p_player.byte_add(0xc).cast::<u16>().read_unaligned();
                    let mut u_var2: u32 = p1f.byte_add(0x89).read() as u32;
                    let d8c88 = (&raw const DAT_009d8c88).cast::<u8>();
                    let fd458 = (&raw const DAT_009fd458).cast::<u8>();
                    if DAT_009a2062 == 2
                        && (pl_c as u32 == u_var2)
                        && p_entry.byte_add(2).read() == 0
                    {
                        b_var5 = 0x2f;
                    } else if local_4 == 10
                        || ((&raw const DAT_009d6990)
                            .cast::<i32>()
                            .add(u_var2 as usize)
                            .read() as u32)
                            .wrapping_add(
                                (&raw const DAT_009d6968)
                                    .cast::<i32>()
                                    .add(u_var2 as usize)
                                    .read() as u32,
                            ) as i32
                            != local_10
                    {
                        let i1b = p_entry.byte_add(0x1b).cast::<i32>().read_unaligned();
                        if i1b != -1 && i1b != DAT_009ecc34 {
                            if DAT_009a2062 == 2 {
                                let idx = ((u_var6 & 0xffff).wrapping_mul(0x23))
                                    .wrapping_add(0x16)
                                    .wrapping_add(DAT_009a1fe8 as u32)
                                    as *const u8;
                                let idx = idx.read() as u32;
                                let v = d8c88.add(idx.wrapping_mul(0x566) as usize).read() as u32;
                                let b_var7: u32 = if v == pl_c as u32 {
                                    0
                                } else {
                                    (fd458
                                        .add(v.wrapping_add((pl_c as u32).wrapping_mul(0x1e5))
                                            as usize)
                                        .read()
                                        == 0) as u32
                                };
                                let n = (b_var7 == 0) as u32;
                                b_var5 = (n.wrapping_neg() & 0xa0).wrapping_add(0x33) as u8;
                            } else {
                                b_var5 = match p1f.byte_add(0x88).read() {
                                    0 => 0x3f,
                                    1 | 4 => 0x37,
                                    2 => 0x33,
                                    _ => 0x3b,
                                };
                            }
                        }
                    } else {
                        b_var5 = 0xd4;
                    }
                    if b_var5 != 0 {
                        if DAT_009d6959 == 0 {
                            u_var2 = pl_c as u32;
                            let pa9 =
                                p1f.byte_add(0xa9).cast::<u32>().read_unaligned() as *const u8;
                            if pa9.add(u_var2.wrapping_add(0x1f6) as usize).read() == 0 {
                                let idx = ((u_var6 & 0xffff).wrapping_mul(0x23))
                                    .wrapping_add(0x16)
                                    .wrapping_add(DAT_009a1fe8 as u32)
                                    as *const u8;
                                let idx = idx.read() as u32;
                                let v = d8c88.add(idx.wrapping_mul(0x566) as usize).read() as u32;
                                let b_var7: u32 = if v == u_var2 {
                                    0
                                } else {
                                    (fd458
                                        .add(v.wrapping_add(u_var2.wrapping_mul(0x1e5)) as usize)
                                        .read()
                                        == 0) as u32
                                };
                                if b_var7 != 0 {
                                    break 'body;
                                }
                            }
                        }
                        let r = FUN_00484f80(u_var6 as u16);
                        let pl1a = p_player.byte_add(0x1a).cast::<u16>().read_unaligned();
                        if (r as u8) == 0 && (pl1a as u32) != u_var6 {
                            FUN_00482eb0(u_var6 as u16, 0xffff, b_var5);
                        }
                    }
                }
                u_var6 = u_var6.wrapping_add(1);
                local_c = local_c.wrapping_add(0x23);
                if (u_var6 as i32) >= DAT_009a1fe0 {
                    break;
                }
            }
        }
    }
}
