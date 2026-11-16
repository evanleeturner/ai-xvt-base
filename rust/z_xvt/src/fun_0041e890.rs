// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4a61: u8;
    static mut DAT_006d4a6a: i16;
    static mut DAT_006d4a8c: i16;
    static mut DAT_006d4a8e: i16;
    static mut DAT_006d4a6c: u8;
    static mut DAT_006d4a74: u8;
    static mut DAT_006d4a78: u8;
    static mut DAT_006d4a7c: u8;
    static mut DAT_006d4a80: u8;
    static mut DAT_006d4a88: u8;
    static mut DAT_006d4a8a: u8;
    static mut DAT_006d4a96: u8;
    static mut DAT_006d4aa2: i8;
    static mut DAT_006d4aa3: i8;
    static mut DAT_006d4aa7: i8;
    static mut DAT_006d4aa9: i8;
    static mut DAT_006d4aab: i8;
    static mut DAT_006d4aad: i8;
    static mut DAT_006d4ab1: i8;
    static mut DAT_006d4ab8: i8;
    static mut DAT_006d4980: u32;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_00732a3c: u32;
    static mut DAT_0073b374: u16;
    static mut DAT_007092db: i8;
    static mut DAT_00709d0a: i8;
    static mut DAT_00709d0b: i8;
    static mut DAT_0070ad42: u8;
    static mut DAT_0070ad43: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0070ad53: i8;
    static mut DAT_0070ad54: i8;
    static mut DAT_0070ad78: i8;
    static mut DAT_0070ad7c: i8;
    static mut DAT_0070ad8d: i8;
    static mut DAT_0070ad8f: i8;
    static mut DAT_005247e8: u8;
    static mut DAT_0052687a: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049d810(param_1: *mut core::ffi::c_void, param_2: i32) -> u32;
    fn FUN_0049f9c0(param_1: u16, param_2: u16, param_3: i32);
    fn FUN_0041eea0(param_1: u16);
    fn FUN_004215c0(param_1: u16);
}

unsafe fn add16(base: *mut u8, off: usize, v: i16) {
    // SAFETY: caller passes the game's own table address and offset.
    unsafe {
        let p = base.add(off).cast::<i16>();
        p.write_unaligned(p.read_unaligned().wrapping_add(v));
    }
}

unsafe fn add8(base: *mut i8, off: usize, v: i8) {
    // SAFETY: caller passes the game's own table address and offset.
    unsafe {
        let p = base.add(off);
        *p = (*p).wrapping_add(v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041e890(param_1: u16) {
    // SAFETY: all globals and callees belong to the game; pointers are
    // read and written at the same offsets the original uses.
    unsafe {
        let mut u_var9: u32 = param_1 as u32;
        let i_var5: usize = u_var9.wrapping_mul(0x126) as usize;
        let a61 = &raw mut DAT_006d4a61;
        let a6a = &raw mut DAT_006d4a6a;
        if *a61.add(i_var5) == 0 || *a6a.add(u_var9.wrapping_mul(0x93) as usize) == 0 {
            return;
        }
        let mut b_var3: i32 = 1;
        if DAT_00732a3c < DAT_006d4980 {
            let mut i_var12 = DAT_006d4980.wrapping_sub(DAT_00732a3c) as i32;
            let mut puvar10 =
                DAT_006d4988.add(DAT_00732a3c.wrapping_mul(0x23).wrapping_add(0x1f) as usize);
            loop {
                let p_obj: *mut u8 = puvar10.cast::<*mut u8>().read_unaligned();
                if *puvar10.sub(0x1c) != 0 && *p_obj == 0 && (*puvar10.sub(9) as u32) == u_var9 {
                    let p_tmp: *mut u8 = p_obj.add(0xa7).cast::<*mut u8>().read_unaligned();
                    let c = *p_tmp.add(7) as i8;
                    if c != 3 && c != 4 {
                        b_var3 = 0;
                    }
                }
                puvar10 = puvar10.add(0x23);
                i_var12 = i_var12.wrapping_sub(1);
                if i_var12 == 0 {
                    break;
                }
            }
        }
        if b_var3 == 0 {
            return;
        }
        let i_var12: usize = u_var9.wrapping_mul(0x566) as usize;
        let i_var2: usize = (*(&raw mut DAT_0070ad48).add(i_var12) as usize) * 3;
        let a_ = *(&raw mut DAT_00709d0a).add(i_var2);
        let b_ = *(&raw mut DAT_00709d0b).add(i_var2);
        if !((a_ != 1 || (b_ != 1 && b_ != 2)) && (a_ != 2 || b_ != 1)) {
            return;
        }
        b_var3 = 0;
        if DAT_007092db != 2 {
            let p78 = (&raw mut DAT_0070ad78).add(i_var12);
            if (*p78 != 0 || *(&raw mut DAT_0070ad7c).add(i_var12) != 0)
                && (FUN_0049d810(p78.cast(), 0) & 1) != 0
            {
                b_var3 = 1;
            }
            let c1 = *(&raw mut DAT_0070ad53).add(i_var12);
            if c1 == 1
                && (&raw mut DAT_006d4a8a)
                    .add(i_var5)
                    .cast::<i16>()
                    .read_unaligned()
                    != 0
            {
                b_var3 = 1;
            }
            if c1 == 2 && a_ == 1 {
                b_var3 = 1;
            }
            if c1 == 3 && (a_ == 2 || b_ == 1) {
                b_var3 = 1;
            }
        }
        if b_var3 != 0 {
            let p6c = (&raw mut DAT_006d4a6c).add(i_var5).cast::<i16>();
            let s_var8: i16 =
                (*a6a.add(u_var9.wrapping_mul(0x93) as usize)).wrapping_sub(p6c.read_unaligned());
            p6c.write_unaligned(p6c.read_unaligned().wrapping_add(s_var8));
            let c_var6: i8 = (*(&raw mut DAT_006d4aa2).add(i_var5))
                .wrapping_sub(*(&raw mut DAT_006d4aa3).add(i_var5));
            add16(&raw mut DAT_006d4a96, i_var5, s_var8);
            add8(&raw mut DAT_006d4ab8, i_var5, c_var6);
            add16(&raw mut DAT_006d4a7c, i_var5, s_var8);
            add8(&raw mut DAT_006d4aab, i_var5, c_var6);
            add16(&raw mut DAT_006d4a88, i_var5, s_var8);
            add8(&raw mut DAT_006d4ab1, i_var5, c_var6);
            add16(&raw mut DAT_006d4a78, i_var5, s_var8);
            add8(&raw mut DAT_006d4aa9, i_var5, c_var6);
            add16(&raw mut DAT_006d4a74, i_var5, s_var8);
            add8(&raw mut DAT_006d4aa7, i_var5, c_var6);
            add16(&raw mut DAT_006d4a80, i_var5, s_var8);
            let c1 = *(&raw mut DAT_0070ad8d).add(i_var12);
            add8(&raw mut DAT_006d4aad, i_var5, c_var6);
            *a61.add(i_var5) = 0;
            let idx = u_var9.wrapping_mul(0x93) as usize;
            if c1 == 0 && *(&raw mut DAT_0070ad8f).add(i_var12) == 0 {
                let p = (&raw mut DAT_006d4a8c).add(idx);
                *p = (*p).wrapping_add(s_var8);
                return;
            }
            let p = (&raw mut DAT_006d4a8e).add(idx);
            *p = (*p).wrapping_add(s_var8);
            return;
        }
        u_var9 = 0;
        DAT_0073b374 = param_1;
        if DAT_00732a3c < DAT_006d4980 {
            let mut i_var5b = DAT_006d4980.wrapping_sub(DAT_00732a3c) as i32;
            let mut pc = DAT_006d4988.add(DAT_00732a3c.wrapping_mul(0x23).wrapping_add(3) as usize);
            loop {
                if *pc == 0 {
                    u_var9 = u_var9.wrapping_add(1);
                }
                pc = pc.add(0x23);
                i_var5b = i_var5b.wrapping_sub(1);
                if i_var5b == 0 {
                    break;
                }
            }
        }
        if u_var9 < *(&raw mut DAT_0070ad43).add(i_var12) as u32 {
            let mut i_stack_4: i32 =
                (*(&raw mut DAT_0070ad43).add(i_var12) as u32).wrapping_sub(u_var9) as i32;
            if DAT_00732a3c < DAT_006d4980 {
                let mut i5: i32 = DAT_00732a3c.wrapping_mul(0x23) as i32;
                u_var9 = DAT_00732a3c;
                loop {
                    let ent = DAT_006d4988.byte_offset(i5 as isize);
                    let i_link: i32 = ent.add(0x1b).cast::<i32>().read_unaligned();
                    if *ent.add(3) != 0 && *ent.add(2) == 0x0d && i_link == -1 {
                        *ent.add(3) = 0;
                        FUN_0049f9c0(
                            u_var9 as u16,
                            *DAT_006d4988.byte_offset(0x16 + i5 as isize) as u16,
                            2,
                        );
                        i_stack_4 = i_stack_4.wrapping_sub(1);
                        if i_stack_4 == 0 {
                            break;
                        }
                    }
                    i5 = i5.wrapping_add(0x23);
                    u_var9 = u_var9.wrapping_add(1);
                    if !(u_var9 < DAT_006d4980) {
                        break;
                    }
                }
            }
            if i_stack_4 != 0 && DAT_00732a3c < DAT_006d4980 {
                let mut i5: i32 = DAT_00732a3c.wrapping_mul(0x23) as i32;
                u_var9 = DAT_00732a3c;
                loop {
                    let ent = DAT_006d4988.byte_offset(i5 as isize);
                    let i_link: i32 = ent.add(0x1b).cast::<i32>().read_unaligned();
                    let p_obj: *mut u8 = ent.add(0x1f).cast::<*mut u8>().read_unaligned();
                    if *ent.add(3) != 0 && *p_obj == 0 && i_link == -1 {
                        let p_sub: *mut u8 = p_obj.add(0xa7).cast::<*mut u8>().read_unaligned();
                        if *p_sub.add(7) == 3 || *p_sub.add(7) == 4 {
                            *ent.add(3) = 0;
                            let p_ptr: *mut u8 =
                                p_sub.add(0x432).cast::<*mut u8>().read_unaligned();
                            if !p_ptr.is_null() {
                                *p_ptr.add(3) = 0;
                                p_sub
                                    .add(0x432)
                                    .cast::<*mut u8>()
                                    .write_unaligned(core::ptr::null_mut());
                            }
                            let mut pv = p_sub.add(0x3f2);
                            let mut n = 0x10;
                            loop {
                                let q: *mut u8 = pv.cast::<*mut u8>().read_unaligned();
                                if !q.is_null() {
                                    *q.add(3) = 0;
                                    pv.cast::<*mut u8>().write_unaligned(core::ptr::null_mut());
                                }
                                pv = pv.add(4);
                                n -= 1;
                                if n == 0 {
                                    break;
                                }
                            }
                            FUN_0049f9c0(
                                u_var9 as u16,
                                *DAT_006d4988.byte_offset(0x16 + i5 as isize) as u16,
                                2,
                            );
                            i_stack_4 = i_stack_4.wrapping_sub(1);
                            if i_stack_4 == 0 {
                                break;
                            }
                        }
                    }
                    i5 = i5.wrapping_add(0x23);
                    u_var9 = u_var9.wrapping_add(1);
                    if !(u_var9 < DAT_006d4980) {
                        break;
                    }
                }
            }
        }
        let o = *(&raw mut DAT_0070ad42).add((DAT_0073b374 as u32).wrapping_mul(0x566) as usize)
            as usize;
        let t = *(&raw mut DAT_005247e8).add(o) as usize;
        if (*(&raw mut DAT_0052687a).add(t * 0x16) & 0x80) == 0 {
            FUN_0041eea0(0xffff);
        } else {
            FUN_004215c0(0xffff);
        }
        u_var9 = DAT_0073b374 as u32;
        let ix = u_var9.wrapping_mul(0x126) as usize;
        let c_var1 = *a61.add(ix);
        if c_var1 != 0 {
            if *(&raw mut DAT_0070ad54).add(u_var9.wrapping_mul(0x566) as usize) == 0 {
                *a61.add(ix) = c_var1.wrapping_sub(1);
            } else if DAT_007092db != 2 {
                *a61.add(ix) = c_var1.wrapping_sub(1);
                return;
            }
        }
    }
}
