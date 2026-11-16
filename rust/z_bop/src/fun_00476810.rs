// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00999488: i32;
    static mut DAT_0099947e: [u8; 4];
    static mut DAT_0060f204: i32;
    static mut DAT_0060f1d8: *mut i32;
    static mut _DAT_0060f208: *mut i32;
    static mut DAT_0060f1c0: i32;
    static mut DAT_0060f1e8: i32;
    static mut _DAT_0060f1f0: i32;
}

#[inline(always)]
unsafe fn cstr_count(p: *const u8) -> u32 {
    // SAFETY: caller passes a NUL-terminated string.
    unsafe {
        let mut n: u32 = 0;
        loop {
            let c = p.add(n as usize).read();
            n = n.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        n
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00476810(param_1: *mut u32, param_2: *mut u32) -> i32 {
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: the pointers are the game's own structures; the null-base
    // reads reproduce the original's fixed low-address accesses.
    unsafe {
        let mut i_var5: i32 = 0x18;
        let s = param_1.read_unaligned() as *const u8;
        if !s.is_null() {
            i_var5 = cstr_count(s).wrapping_add(0x18) as i32;
        }
        let pi_var7 = param_1.add(5).read_unaligned() as *mut i32;
        let p1 = param_1.add(1).read_unaligned();
        let p4 = param_1.add(4).read_unaligned();
        'end: {
            if pi_var7.is_null() {
                if p1 != 0x14 {
                    break 'end;
                }
                let rd = |o: usize| (o as *const i32).read_volatile();
                let i_var3 = (rd(0x14) as u32).wrapping_mul(rd(0x10) as u32) as i32;
                let mut i_var4 = i_var3;
                if (i_var3 as u32).wrapping_sub(rd(8) as u32) == 0 {
                    i_var4 = rd(0xc);
                }
                i_var5 = i_var5.wrapping_add(0x18).wrapping_add(i_var4);
                if rd(4) != 0 {
                    i_var5 = i_var5.wrapping_add((rd(4) as u32).wrapping_mul(0x300) as i32);
                    break 'end;
                }
                let mut i_var3 = i_var3;
                if (i_var3 as u32).wrapping_sub(rd(8) as u32) == 0 {
                    i_var3 = rd(0xc);
                }
                if rd(0) as u32 != (i_var3 as u32).wrapping_add(0x18) {
                    break 'end;
                }
                i_var5 = i_var5.wrapping_add(0x3000);
            } else {
                match p1 {
                    1 | 0xf | 0x10 | 0x11 => {
                        if DAT_00999488 < pi_var7.read_unaligned() {
                            DAT_00999488 = pi_var7.read_unaligned();
                        }
                        i_var5 =
                            i_var5.wrapping_add(4u32.wrapping_add(p4.wrapping_mul(100)) as i32);
                        if param_2.add(0x21).read_unaligned() == 0 {
                            i_var5 = i_var5.wrapping_add(DAT_0060f204.wrapping_mul(0xc));
                        }
                    }
                    2 => i_var5 = i_var5.wrapping_add(0x30),
                    3 => {
                        DAT_0060f204 = p4 as i32;
                        i_var5 = i_var5.wrapping_add(DAT_0060f204.wrapping_mul(0xc));
                        let t = i32::from_ne_bytes(DAT_0099947e);
                        if t < p4 as i32 {
                            DAT_0099947e = (p4 as i32).to_ne_bytes();
                        }
                    }
                    4 => i_var5 = i_var5.wrapping_add(0xc),
                    5 => i_var5 = i_var5.wrapping_add(0x24),
                    6 => i_var5 = i_var5.wrapping_add(0xc),
                    7 => {
                        i_var5 = i_var5.wrapping_add(cstr_count(pi_var7 as *const u8) as i32);
                    }
                    9 => {
                        i_var5 = i_var5.wrapping_add(p4.wrapping_mul(0x38) as i32);
                        DAT_0060f1d8 = pi_var7;
                    }
                    0xb => {
                        i_var5 = i_var5.wrapping_add(p4.wrapping_mul(0xc) as i32);
                        _DAT_0060f208 = pi_var7;
                        param_2.add(0x21).write_unaligned(pi_var7 as usize as u32);
                    }
                    0xd => i_var5 = i_var5.wrapping_add(p4.wrapping_mul(8) as i32),
                    0x13 => i_var5 = i_var5.wrapping_add(0xc),
                    0x14 => {
                        let rd = |o: usize| pi_var7.add(o).read_unaligned();
                        let mut i_var4 = (rd(5) as u32).wrapping_mul(rd(4) as u32) as i32;
                        let mut i_var3 = i_var4;
                        if (i_var4 as u32).wrapping_sub(rd(2) as u32) == 0 {
                            i_var3 = rd(3);
                        }
                        i_var5 = i_var5.wrapping_add(0x18).wrapping_add(i_var3);
                        if rd(1) != 0 {
                            i_var5 = i_var5.wrapping_add((rd(1) as u32).wrapping_mul(0x300) as i32);
                            break 'end;
                        }
                        if (i_var4 as u32).wrapping_sub(rd(2) as u32) == 0 {
                            i_var4 = rd(3);
                        }
                        if rd(0)
                            != (pi_var7 as usize as i32)
                                .wrapping_add(i_var4)
                                .wrapping_add(0x18)
                        {
                            break 'end;
                        }
                        i_var5 = i_var5.wrapping_add(0x3000);
                    }
                    0x15 => i_var5 = i_var5.wrapping_add(p4.wrapping_mul(4) as i32),
                    0x16 => i_var5 = i_var5.wrapping_add(0x10),
                    0x17 => i_var5 = i_var5.wrapping_add(0x30),
                    0x19 => i_var5 = i_var5.wrapping_add(0x48),
                    _ => {}
                }
            }
        }
        let p2 = param_1.add(2).read_unaligned();
        if p2 != 0 {
            let mut local_d0 = [0u32; 52];
            let mut src = param_2;
            for k in 0..0x34usize {
                local_d0[k] = src.read_unaligned();
                src = src.add(1);
            }
            DAT_0060f1c0 = 0;
            DAT_0060f1e8 = 0;
            _DAT_0060f208 = core::ptr::null_mut();
            _DAT_0060f1f0 = 0;
            DAT_0060f1d8 = core::ptr::null_mut();
            let mut local_d4: i32 = 0;
            i_var5 = i_var5.wrapping_add(p2.wrapping_mul(4) as i32);
            if 0 < p2 as i32 {
                let mut local_d8: u32 = 0;
                loop {
                    let base = param_1.add(3).read_unaligned();
                    let child = (base.wrapping_add(local_d8) as usize as *const u32)
                        .read_unaligned() as usize as *mut u32;
                    let i_var4 = FUN_00476810(child, local_d0.as_mut_ptr());
                    i_var5 = i_var5.wrapping_add(i_var4);
                    local_d4 = local_d4.wrapping_add(1);
                    local_d8 = local_d8.wrapping_add(4);
                    if !(local_d4 < param_1.add(2).read_unaligned() as i32) {
                        break;
                    }
                }
            }
        }
        i_var5
    }
}
