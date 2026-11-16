// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006026bc: i32;
    static mut _DAT_006026c0: i32;
    static mut _DAT_006026a8: i32;
    static mut DAT_00602690: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048eb30(param_1: *mut i8) -> i32;
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address comes from the game's own data structures.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

unsafe fn count(p: *const i8) -> u32 {
    let mut u: u32 = 0xffff_ffff;
    let mut p = p;
    // SAFETY: p points at a NUL-terminated string in the game's memory.
    unsafe {
        loop {
            if u == 0 {
                break;
            }
            u = u.wrapping_sub(1);
            let c = p.read();
            p = p.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
    }
    !u
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048d5e0(param_1: u32, param_2: *mut u32, param_3: *mut u32) -> i32 {
    if param_2.is_null() {
        return 0;
    }
    // SAFETY: all pointers are the game's own structures, read and written
    // where the original does; the globals are the game's memory.
    unsafe {
        let mut i_var8: i32 = 0x18;
        let name = param_2.read_unaligned();
        if name != 0 {
            i_var8 = count(name as usize as *const i8).wrapping_add(0x18) as i32;
        }
        let mut i_var10: i32 = param_2.add(5).read_unaligned() as i32;
        if i_var10 != 0 && param_2.add(4).read_unaligned() != 0 && rd(i_var10.wrapping_add(8)) != 0
        {
            let mut pi_var7: *mut i32 = rd(i_var10.wrapping_add(8)) as u32 as usize as *mut i32;
            match param_2.add(1).read_unaligned() {
                1 => {
                    let mut i_var6: i32 = 0;
                    let mut i_var3: i32 = 0;
                    if 0 < rd(i_var10.wrapping_add(4)) {
                        loop {
                            let mut pi_var5 =
                                pi_var7.wrapping_offset(i_var3.wrapping_add(1) as isize);
                            let mut i_var12 = i_var3.wrapping_add(1);
                            'lab: {
                                loop {
                                    i_var3 = i_var12;
                                    i_var6 = i_var6.wrapping_add(1);
                                    if pi_var5.wrapping_add(2).read_unaligned() == -1 {
                                        i_var3 = i_var3.wrapping_add(3);
                                        break 'lab;
                                    }
                                    let pi_var1 = pi_var5.wrapping_add(3);
                                    pi_var5 = pi_var5.wrapping_add(2);
                                    i_var12 = i_var3.wrapping_add(2);
                                    if pi_var1.read_unaligned() == -1 {
                                        break;
                                    }
                                }
                                i_var3 = i_var3.wrapping_add(4);
                            }
                            if !(i_var3 < rd(i_var10.wrapping_add(4))) {
                                break;
                            }
                        }
                    }
                    i_var8 = i_var8
                        .wrapping_add(4)
                        .wrapping_add(i_var6.wrapping_mul(100))
                        .wrapping_add(DAT_006026bc.wrapping_mul(0xc));
                }
                2 => {
                    i_var8 = i_var8.wrapping_add(0x30);
                }
                3 => {
                    DAT_006026bc = rd(i_var10.wrapping_add(4));
                    i_var8 = i_var8.wrapping_add(DAT_006026bc.wrapping_mul(0xc));
                }
                4 => {
                    i_var8 = i_var8.wrapping_add(0xc);
                }
                5 => {
                    i_var8 = i_var8.wrapping_add(0x24);
                }
                6 => {
                    i_var8 = i_var8.wrapping_add(0xc);
                }
                7 => {
                    i_var8 = i_var8.wrapping_add(count(pi_var7 as *const i8) as i32);
                }
                9 => {
                    let mut i_var6 = rd(i_var10.wrapping_add(4));
                    if rd(i_var10.wrapping_add(4)) < rd(i_var10.wrapping_add(0x10)) {
                        i_var6 = rd(i_var10.wrapping_add(0x10));
                    }
                    if i_var6 < rd(i_var10.wrapping_add(0x1c)) {
                        i_var6 = rd(i_var10.wrapping_add(0x1c));
                    }
                    if i_var6 < rd(i_var10.wrapping_add(0x28)) {
                        i_var6 = rd(i_var10.wrapping_add(0x28));
                    }
                    if i_var6 < rd(i_var10.wrapping_add(0x34)) {
                        i_var6 = rd(i_var10.wrapping_add(0x34));
                    }
                    if i_var6 < rd(i_var10.wrapping_add(0x40)) {
                        i_var6 = rd(i_var10.wrapping_add(0x40));
                    }
                    i_var8 = i_var8.wrapping_add(i_var6.wrapping_mul(0x38));
                }
                0xb => {
                    i_var8 = i_var8.wrapping_add(rd(i_var10.wrapping_add(4)).wrapping_mul(0xc));
                    _DAT_006026c0 = i_var10;
                    param_3.add(0x21).write_unaligned(i_var10 as u32);
                }
                0xd => {
                    i_var8 = i_var8.wrapping_add(rd(i_var10.wrapping_add(4)).wrapping_mul(8));
                }
                0xf => {
                    if rd(i_var10.wrapping_add(0x14)) != 0 && rd(i_var10.wrapping_add(0x20)) != 0 {
                        let a = rd(rd(i_var10.wrapping_add(0x20))).wrapping_sub(1);
                        let b = rd(rd(i_var10.wrapping_add(0x14))).wrapping_sub(1);
                        i_var8 = i_var8
                            .wrapping_add(4)
                            .wrapping_add(a.wrapping_mul(b).wrapping_mul(100))
                            .wrapping_add(DAT_006026bc.wrapping_mul(0xc));
                    }
                }
                0x10 => {
                    let mut i_var3: i32 = 0;
                    let mut i_var6: i32 = 0;
                    let mut i_var_d4: i32 = rd(i_var10.wrapping_add(4));
                    if 0 < i_var_d4 {
                        loop {
                            i_var10 = pi_var7.read_unaligned();
                            let mut i_var12 = i_var3;
                            if 2 < i_var10 {
                                let mut i_var11 = i_var3.wrapping_add(1);
                                let mut i_var9 = i_var11.wrapping_sub(i_var3);
                                loop {
                                    i_var12 = i_var11;
                                    if !(i_var9 < i_var10) {
                                        break;
                                    }
                                    i_var6 = i_var6.wrapping_add(1);
                                    let c = if i_var11.wrapping_add(2).wrapping_sub(i_var10)
                                        == i_var3
                                    {
                                        true
                                    } else {
                                        i_var12 = i_var11.wrapping_add(3);
                                        i_var12.wrapping_sub(i_var10) != i_var3
                                    };
                                    if !c {
                                        break;
                                    }
                                    i_var11 = i_var11.wrapping_add(2);
                                    i_var9 = i_var11.wrapping_sub(i_var3);
                                }
                            }
                            pi_var7 = pi_var7.wrapping_add(1);
                            i_var_d4 = i_var_d4.wrapping_sub(1);
                            i_var3 = i_var12;
                            if i_var_d4 == 0 {
                                break;
                            }
                        }
                    }
                    i_var8 = i_var8
                        .wrapping_add(4)
                        .wrapping_add(i_var6.wrapping_mul(100))
                        .wrapping_add(DAT_006026bc.wrapping_mul(0xc));
                }
                0x11 => {
                    let mut p = rd(i_var10.wrapping_add(0x14)) as u32 as usize as *const i32;
                    if !p.is_null() {
                        let mut i_var6: i32 = 0;
                        let mut n = rd(i_var10.wrapping_add(0x10));
                        if 0 < n {
                            loop {
                                let i_var3 = p.read_unaligned();
                                p = p.wrapping_add(1);
                                i_var6 = i_var6.wrapping_add(i_var3).wrapping_sub(2);
                                n = n.wrapping_sub(1);
                                if n == 0 {
                                    break;
                                }
                            }
                        }
                        i_var8 = i_var8
                            .wrapping_add(4)
                            .wrapping_add(i_var6.wrapping_mul(100))
                            .wrapping_add(DAT_006026bc.wrapping_mul(0xc));
                    }
                }
                0x14 => {
                    let r = FUN_0048eb30(pi_var7 as *mut i8);
                    i_var8 = i_var8.wrapping_add(r);
                }
                0x15 => {
                    i_var8 = i_var8.wrapping_add(rd(i_var10.wrapping_add(4)).wrapping_mul(4));
                }
                0x16 => {
                    i_var8 = i_var8.wrapping_add(0x10);
                }
                0x17 => {
                    i_var8 = i_var8.wrapping_add(0x30);
                }
                0x19 => {
                    i_var8 = i_var8.wrapping_add(0x48);
                }
                _ => {}
            }
        }
        i_var10 = 0;
        let count_sub = param_2.add(2).read_unaligned();
        if count_sub != 0 {
            let mut au_stack_d0: [u32; 52] = [0; 52];
            let mut src = param_3;
            let mut idx = 0usize;
            let mut k: i32 = 0x34;
            while k != 0 {
                au_stack_d0[idx] = src.read_unaligned();
                src = src.add(1);
                idx += 1;
                k = k.wrapping_sub(1);
            }
            let mut i_var6: i32 = 0;
            _DAT_006026c0 = 0;
            _DAT_006026a8 = 0;
            DAT_00602690 = 0;
            i_var8 = i_var8.wrapping_add(count_sub.wrapping_mul(4) as i32);
            if 0 < count_sub as i32 {
                loop {
                    i_var6 = i_var6.wrapping_add(1);
                    i_var10 = i_var10.wrapping_add(4);
                    let base = param_2.add(3).read_unaligned() as i32;
                    let child =
                        rd(base.wrapping_sub(4).wrapping_add(i_var10)) as u32 as usize as *mut u32;
                    let i_var3 = FUN_0048d5e0(param_1, child, au_stack_d0.as_mut_ptr());
                    i_var8 = i_var8.wrapping_add(i_var3);
                    if !(i_var6 < param_2.add(2).read_unaligned() as i32) {
                        break;
                    }
                }
            }
        }
        i_var8
    }
}
