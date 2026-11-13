// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009e9640: i32;
    static mut DAT_009d9192: i32;
    static mut DAT_009e96f7: i16;
    static mut DAT_009e96fb: i16;
    static mut DAT_009e96ff: i16;
    static mut DAT_009e96dd: i32;
    static mut DAT_0051a3de: u16;
    static mut DAT_009d6990: u32;
    static mut DAT_009d8c88: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00435070(param_1: u16) {
    // SAFETY: all addresses are the game's own memory, accessed at the offsets
    // the original uses.
    unsafe {
        let i_var3: i32 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let p1 = (DAT_009a1fe8.wrapping_add(i_var3).wrapping_add(0x1f)) as usize as *const i32;
        let rec = p1.read_unaligned();
        let u_var6: u32 = (rec.wrapping_add(0x85) as usize as *const u16).read_unaligned() as u32;
        if (u_var6 as i32) < DAT_009a1fe0 {
            let i_var4: i32 = u_var6.wrapping_mul(0x23) as i32;
            let i_var7a: i32 = DAT_009a1fe8.wrapping_add(i_var4);
            if (i_var7a.wrapping_add(3) as usize as *const i8).read_unaligned() != 0 {
                let inner = (i_var7a.wrapping_add(0x1f) as usize as *const i32).read_unaligned();
                let i_var2: i32 =
                    (inner.wrapping_add(0xa9) as usize as *const i32).read_unaligned();
                let u_var5: u16 = (DAT_009e9640 as u16).wrapping_add(0x80);
                let team_idx: u32 =
                    (i_var7a.wrapping_add(0x16) as usize as *const u8).read_unaligned() as u32;
                let i_var7: i32 = ((&raw mut DAT_009d9192 as usize as i32)
                    .wrapping_add(team_idx.wrapping_mul(0x566) as i32)
                    as usize as *const i32)
                    .read_unaligned();
                let kind: u8 = (DAT_009a1fe8.wrapping_add(i_var3).wrapping_add(3) as usize
                    as *const u8)
                    .read_unaligned();
                match kind {
                    0x89..=0x8c => {
                        let p = i_var2.wrapping_add(0x24e) as usize as *mut i16;
                        p.write_unaligned(p.read_unaligned().wrapping_add(1));
                        if param_1 < u_var5 && i_var7 != -1 {
                            let q = (&raw mut DAT_009e96f7)
                                .wrapping_offset(i_var7.wrapping_mul(0x5bd) as isize);
                            q.write_unaligned(q.read_unaligned().wrapping_add(1));
                            return;
                        }
                    }
                    0x8d | 0x8e => {
                        let p = i_var2.wrapping_add(0x252) as usize as *mut i16;
                        p.write_unaligned(p.read_unaligned().wrapping_add(1));
                        if param_1 < u_var5 && i_var7 != -1 {
                            let q = (&raw mut DAT_009e96fb)
                                .wrapping_offset(i_var7.wrapping_mul(0x5bd) as isize);
                            q.write_unaligned(q.read_unaligned().wrapping_add(1));
                            return;
                        }
                    }
                    0x8f | 0x90 | 0x94..=0x9a => {
                        let p = i_var2.wrapping_add(0x255) as usize as *mut u8;
                        p.write_unaligned(p.read_unaligned().wrapping_add(1));
                        let i_var2b = DAT_009a1fe8;
                        let b_var1: u8 = (i_var2b.wrapping_add(0x16).wrapping_add(i_var4) as usize
                            as *const u8)
                            .read_unaligned();
                        if param_1 < u_var5 && i_var7 != -1 {
                            let q = (&raw mut DAT_009e96ff)
                                .wrapping_offset(i_var7.wrapping_mul(0x5bd) as isize);
                            q.write_unaligned(q.read_unaligned().wrapping_add(1));
                            let k: u8 = (i_var2b.wrapping_add(3).wrapping_add(i_var3) as usize
                                as *const u8)
                                .read_unaligned();
                            let add: u32 =
                                (&raw mut DAT_0051a3de).add(k as usize).read_unaligned() as u32;
                            let r = ((&raw mut DAT_009e96dd as usize as i32)
                                .wrapping_add(i_var7.wrapping_mul(0x5bd))
                                as usize) as *mut u32;
                            r.write_unaligned(r.read_unaligned().wrapping_add(add));
                        }
                        let idx: u8 = (&raw mut DAT_009d8c88)
                            .byte_add((b_var1 as u32).wrapping_mul(0x566) as usize)
                            .read();
                        let k2: u8 = (DAT_009a1fe8.wrapping_add(3).wrapping_add(i_var3) as usize
                            as *const u8)
                            .read_unaligned();
                        let add2: u32 =
                            (&raw mut DAT_0051a3de).add(k2 as usize).read_unaligned() as u32;
                        let t = (&raw mut DAT_009d6990).add(idx as usize);
                        t.write_unaligned(t.read_unaligned().wrapping_add(add2));
                    }
                    _ => {}
                }
            }
        }
    }
}
