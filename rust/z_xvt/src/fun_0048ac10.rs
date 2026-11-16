// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a8884: i32;
    static mut DAT_006026bc: i32;
    static mut DAT_006a887a: [u8; 4];
    static mut DAT_00602678: i32;
    static mut DAT_006026a0: i32;
    static mut DAT_006026a8: i32;
    static mut DAT_00602690: *mut i32;
    static mut _DAT_006026c0: *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048ac10(param_1: *mut u32, param_2: *mut u32) -> i32 {
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: all pointers belong to the game's own data structures; the
    // absolute-address reads reproduce the original's null-pointer-based
    // accesses; globals are the game's own memory.
    unsafe {
        let mut i_var5: i32 = 0x18;
        let s = param_1.read() as *const i8;
        if !s.is_null() {
            let mut u_var2: u32 = 0xffff_ffff;
            let mut p = s;
            loop {
                if u_var2 == 0 {
                    break;
                }
                u_var2 = u_var2.wrapping_sub(1);
                let c = p.read();
                p = p.add(1);
                if c == 0 {
                    break;
                }
            }
            i_var5 = (!u_var2).wrapping_add(0x18) as i32;
        }
        let pi_var7 = param_1.add(5).read() as *mut i32;
        'end: {
            if pi_var7.is_null() {
                if param_1.add(1).read() != 0x14 {
                    break 'end;
                }
                let rd = |a: usize| -> i32 { core::ptr::read_volatile(a as *const i32) };
                let mut i_var3 = rd(0x14).wrapping_mul(rd(0x10));
                let mut i_var4 = i_var3;
                if i_var3.wrapping_sub(rd(8)) == 0 {
                    i_var4 = rd(0xc);
                }
                i_var5 = i_var5.wrapping_add(0x18).wrapping_add(i_var4);
                if rd(4) != 0 {
                    i_var5 = i_var5.wrapping_add(rd(4).wrapping_mul(0x300));
                    break 'end;
                }
                if i_var3.wrapping_sub(rd(8)) == 0 {
                    i_var3 = rd(0xc);
                }
                if rd(0) != i_var3.wrapping_add(0x18) {
                    break 'end;
                }
                i_var5 = i_var5.wrapping_add(0x3000);
            } else {
                let p4 = param_1.add(4).read();
                match param_1.add(1).read() {
                    1 | 0xf | 0x10 | 0x11 => {
                        if DAT_006a8884 < pi_var7.read() {
                            DAT_006a8884 = pi_var7.read();
                        }
                        i_var5 = i_var5
                            .wrapping_add(4)
                            .wrapping_add(p4.wrapping_mul(100) as i32);
                        if param_2.add(0x21).read() == 0 {
                            i_var5 = i_var5.wrapping_add(DAT_006026bc.wrapping_mul(0xc));
                        }
                    }
                    2 => i_var5 = i_var5.wrapping_add(0x30),
                    3 => {
                        DAT_006026bc = p4 as i32;
                        i_var5 = i_var5.wrapping_add(DAT_006026bc.wrapping_mul(0xc));
                        let ptr = (&raw mut DAT_006a887a).cast::<i32>();
                        let tmp3 = ptr.read_unaligned();
                        if tmp3 < p4 as i32 {
                            ptr.write_unaligned(p4 as i32);
                        }
                    }
                    4 => i_var5 = i_var5.wrapping_add(0xc),
                    5 => i_var5 = i_var5.wrapping_add(0x24),
                    6 => i_var5 = i_var5.wrapping_add(0xc),
                    7 => {
                        let mut u_var2: u32 = 0xffff_ffff;
                        let mut p = pi_var7 as *const i8;
                        loop {
                            if u_var2 == 0 {
                                break;
                            }
                            u_var2 = u_var2.wrapping_sub(1);
                            let c = p.read();
                            p = p.add(1);
                            if c == 0 {
                                break;
                            }
                        }
                        i_var5 = i_var5.wrapping_add((!u_var2) as i32);
                    }
                    9 => {
                        i_var5 = i_var5.wrapping_add(p4.wrapping_mul(0x38) as i32);
                        DAT_00602690 = pi_var7;
                    }
                    0xb => {
                        i_var5 = i_var5.wrapping_add(p4.wrapping_mul(0xc) as i32);
                        _DAT_006026c0 = pi_var7;
                        param_2.add(0x21).write(pi_var7 as u32);
                    }
                    0xd => i_var5 = i_var5.wrapping_add(p4.wrapping_mul(8) as i32),
                    0x13 => i_var5 = i_var5.wrapping_add(0xc),
                    0x14 => {
                        let mut i_var4 = pi_var7.add(5).read().wrapping_mul(pi_var7.add(4).read());
                        let mut i_var3 = i_var4;
                        if i_var4.wrapping_sub(pi_var7.add(2).read()) == 0 {
                            i_var3 = pi_var7.add(3).read();
                        }
                        i_var5 = i_var5.wrapping_add(0x18).wrapping_add(i_var3);
                        if pi_var7.add(1).read() != 0 {
                            i_var5 = i_var5.wrapping_add(pi_var7.add(1).read().wrapping_mul(0x300));
                            break 'end;
                        }
                        if i_var4.wrapping_sub(pi_var7.add(2).read()) == 0 {
                            i_var4 = pi_var7.add(3).read();
                        }
                        if (pi_var7.read() as u32)
                            != (pi_var7 as u32)
                                .wrapping_add(i_var4 as u32)
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
        let cnt = param_1.add(2).read();
        if cnt != 0 {
            let mut local_d0: [u32; 52] = [0; 52];
            let mut src = param_2;
            for i in 0..0x34usize {
                local_d0[i] = src.read();
                src = src.add(1);
            }
            DAT_00602678 = 0;
            DAT_006026a0 = 0;
            _DAT_006026c0 = core::ptr::null_mut();
            DAT_006026a8 = 0;
            DAT_00602690 = core::ptr::null_mut();
            let mut local_d4: i32 = 0;
            i_var5 = i_var5.wrapping_add(cnt.wrapping_mul(4) as i32);
            if 0 < cnt as i32 {
                let mut local_d8: u32 = 0;
                loop {
                    let addr = param_1.add(3).read().wrapping_add(local_d8);
                    let child = (addr as *const *mut u32).read_unaligned();
                    let r = FUN_0048ac10(child, local_d0.as_mut_ptr());
                    i_var5 = i_var5.wrapping_add(r);
                    local_d4 = local_d4.wrapping_add(1);
                    local_d8 = local_d8.wrapping_add(4);
                    if !(local_d4 < param_1.add(2).read() as i32) {
                        break;
                    }
                }
            }
        }
        i_var5
    }
}
