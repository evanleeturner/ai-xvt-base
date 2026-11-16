// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a20c0: u8;
    static mut DAT_009a20ca: u8;
    static mut DAT_009a20cc: u8;
    static mut DAT_009a20d4: u8;
    static mut DAT_009a20d8: u8;
    static mut DAT_009a20dc: u8;
    static mut DAT_009a20e0: u8;
    static mut DAT_009a20e8: u8;
    static mut DAT_009a20ea: u8;
    static mut DAT_009a20ec: u8;
    static mut DAT_009a20ee: u8;
    static mut DAT_009a20f6: u8;
    static mut DAT_009a2102: u8;
    static mut DAT_009a2103: u8;
    static mut DAT_009a2107: u8;
    static mut DAT_009a2109: u8;
    static mut DAT_009a210b: u8;
    static mut DAT_009a210d: u8;
    static mut DAT_009a2111: u8;
    static mut DAT_009a2118: u8;
    static mut DAT_009a1fe0: u32;
    static mut DAT_009a1fe8: i32;
    static mut _DAT_00a07cec: u32;
    static mut DAT_009a1834: u16;
    static mut DAT_009d695b: i8;
    static mut DAT_009d738a: u8;
    static mut DAT_009d738b: u8;
    static mut DAT_009d8c83: u8;
    static mut DAT_009d8c88: u8;
    static mut DAT_009d8c93: u8;
    static mut DAT_009d8cb8: u8;
    static mut DAT_009d8cbc: u8;
    static mut DAT_009d8ccd: u8;
    static mut DAT_009d8ccf: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431b50(param_1: *mut core::ffi::c_void, param_2: i32) -> u32;
    fn FUN_00433d30(param_1: u16, param_2: u16, param_3: i32);
    fn FUN_00458780(param_1: i32);
    fn FUN_00456040();
}

unsafe fn r8(a: u32) -> u8 {
    // SAFETY: the address lies in the game's own memory, as in the original.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address lies in the game's own memory, as in the original.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address lies in the game's own memory, as in the original.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn w8(a: u32, v: u8) {
    // SAFETY: the address lies in the game's own memory, as in the original.
    unsafe { (a as usize as *mut u8).write(v) }
}

unsafe fn w16(a: u32, v: u16) {
    // SAFETY: the address lies in the game's own memory, as in the original.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00455c80(param_1: u16) {
    // SAFETY: all globals and callees are the game's own; every address
    // computed is read and written exactly where the original does.
    unsafe {
        let u_var8_in = param_1 as u32;
        let mut u_var8 = u_var8_in;
        let i_var5: u32 = u_var8.wrapping_mul(0x126);
        let b_c0 = (&raw mut DAT_009a20c0) as usize as u32;
        let b_ca = (&raw mut DAT_009a20ca) as usize as u32;
        if r8(b_c0.wrapping_add(i_var5).wrapping_add(1)) == 0 || r16(b_ca.wrapping_add(i_var5)) == 0
        {
            return;
        }
        let mut b_var3: bool = true;
        if _DAT_00a07cec < DAT_009a1fe0 {
            let mut i_var11: u32 = DAT_009a1fe0.wrapping_sub(_DAT_00a07cec);
            let mut p: u32 = _DAT_00a07cec
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32);
            loop {
                let pc_tmp = r32(p);
                if r8(p.wrapping_sub(0x1c)) != 0
                    && r8(pc_tmp) == 0
                    && (r8(p.wrapping_sub(9)) as u32) == u_var8
                {
                    let i_tmp = r32(pc_tmp.wrapping_add(0xa9));
                    let c = r8(i_tmp.wrapping_add(7));
                    if c != 3 && c != 4 {
                        b_var3 = false;
                    }
                }
                p = p.wrapping_add(0x23);
                i_var11 = i_var11.wrapping_sub(1);
                if i_var11 == 0 {
                    break;
                }
            }
        }
        if !b_var3 {
            return;
        }
        let i_var11: u32 = u_var8.wrapping_mul(0x566);
        let i_var6: u32 =
            (r8((&raw mut DAT_009d8c88) as usize as u32 + i_var11) as u32).wrapping_mul(3);
        let b_738a = (&raw mut DAT_009d738a) as usize as u32;
        let b_738b = (&raw mut DAT_009d738b) as usize as u32;
        let a = r8(b_738a.wrapping_add(i_var6));
        let b = r8(b_738b.wrapping_add(i_var6));
        if !((a != 1 || (b != 1 && b != 2)) && (a != 2 || b != 1)) {
            return;
        }
        let mut b_var3 = false;
        let b_cb8 = (&raw mut DAT_009d8cb8) as usize as u32;
        let b_cbc = (&raw mut DAT_009d8cbc) as usize as u32;
        if DAT_009d695b != 2 {
            if r8(b_cb8.wrapping_add(i_var11)) != 0 || r8(b_cbc.wrapping_add(i_var11)) != 0 {
                let u_var4 = FUN_00431b50(
                    b_cb8.wrapping_add(i_var11) as usize as *mut core::ffi::c_void,
                    0,
                );
                if (u_var4 & 1) != 0 {
                    b_var3 = true;
                }
            }
            let c_var1 = r8((&raw mut DAT_009d8c93) as usize as u32 + i_var11) as i8;
            if c_var1 == 1 && r16((&raw mut DAT_009a20ea) as usize as u32 + i_var5) != 0 {
                b_var3 = true;
            }
            let idx =
                (r8((&raw mut DAT_009d8c88) as usize as u32 + i_var11) as u32).wrapping_mul(3);
            if c_var1 == 2 && r8(b_738a.wrapping_add(idx)) == 1 {
                b_var3 = true;
            }
            let idx =
                (r8((&raw mut DAT_009d8c88) as usize as u32 + i_var11) as u32).wrapping_mul(3);
            if c_var1 == 3
                && (r8(b_738a.wrapping_add(idx)) == 2 || r8(b_738b.wrapping_add(idx)) == 1)
            {
                b_var3 = true;
            }
        }
        if b_var3 {
            let cc = (&raw mut DAT_009a20cc) as usize as u32 + i_var5;
            let s_var9: u16 = r16(b_ca.wrapping_add(i_var5)).wrapping_sub(r16(cc));
            w16(cc, r16(cc).wrapping_add(s_var9));
            let c_var7: u8 = r8((&raw mut DAT_009a2102) as usize as u32 + i_var5)
                .wrapping_sub(r8((&raw mut DAT_009a2103) as usize as u32 + i_var5));
            let a_f6 = (&raw mut DAT_009a20f6) as usize as u32 + i_var5;
            w16(a_f6, r16(a_f6).wrapping_add(s_var9));
            let a = (&raw mut DAT_009a2118) as usize as u32 + i_var5;
            w8(a, r8(a).wrapping_add(c_var7));
            let a_dc = (&raw mut DAT_009a20dc) as usize as u32 + i_var5;
            w16(a_dc, r16(a_dc).wrapping_add(s_var9));
            let a = (&raw mut DAT_009a210b) as usize as u32 + i_var5;
            w8(a, r8(a).wrapping_add(c_var7));
            let a_e8 = (&raw mut DAT_009a20e8) as usize as u32 + i_var5;
            w16(a_e8, r16(a_e8).wrapping_add(s_var9));
            let a = (&raw mut DAT_009a2111) as usize as u32 + i_var5;
            w8(a, r8(a).wrapping_add(c_var7));
            let a_d8 = (&raw mut DAT_009a20d8) as usize as u32 + i_var5;
            w16(a_d8, r16(a_d8).wrapping_add(s_var9));
            let a = (&raw mut DAT_009a2109) as usize as u32 + i_var5;
            w8(a, r8(a).wrapping_add(c_var7));
            let a_d4 = (&raw mut DAT_009a20d4) as usize as u32 + i_var5;
            w16(a_d4, r16(a_d4).wrapping_add(s_var9));
            let a = (&raw mut DAT_009a2107) as usize as u32 + i_var5;
            w8(a, r8(a).wrapping_add(c_var7));
            let a_e0 = (&raw mut DAT_009a20e0) as usize as u32 + i_var5;
            w16(a_e0, r16(a_e0).wrapping_add(s_var9));
            let c_var1 = r8((&raw mut DAT_009d8ccd) as usize as u32 + i_var11);
            let a = (&raw mut DAT_009a210d) as usize as u32 + i_var5;
            w8(a, r8(a).wrapping_add(c_var7));
            w8(b_c0.wrapping_add(i_var5).wrapping_add(1), 0);
            if c_var1 == 0 && r8((&raw mut DAT_009d8ccf) as usize as u32 + i_var11) == 0 {
                let a = (&raw mut DAT_009a20ec) as usize as u32 + i_var5;
                w16(a, r16(a).wrapping_add(s_var9));
                return;
            }
            let a = (&raw mut DAT_009a20ee) as usize as u32 + i_var5;
            w16(a, r16(a).wrapping_add(s_var9));
            return;
        }
        u_var8 = 0;
        DAT_009a1834 = param_1;
        if _DAT_00a07cec < DAT_009a1fe0 {
            let mut i5: u32 = DAT_009a1fe0.wrapping_sub(_DAT_00a07cec);
            let mut p: u32 = _DAT_00a07cec
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_009a1fe8 as u32);
            loop {
                if r8(p) == 0 {
                    u_var8 = u_var8.wrapping_add(1);
                }
                p = p.wrapping_add(0x23);
                i5 = i5.wrapping_sub(1);
                if i5 == 0 {
                    break;
                }
            }
        }
        let want = r8((&raw mut DAT_009d8c83) as usize as u32 + i_var11) as u32;
        if u_var8 < want {
            let mut i_var5: u32 = want.wrapping_sub(u_var8);
            if _DAT_00a07cec < DAT_009a1fe0 {
                let mut i11: u32 = _DAT_00a07cec.wrapping_mul(0x23);
                u_var8 = _DAT_00a07cec;
                loop {
                    let i6 = (DAT_009a1fe8 as u32).wrapping_add(i11);
                    let i_tmp = r32(i6.wrapping_add(0x1b)) as i32;
                    if r8(i6.wrapping_add(3)) != 0 && r8(i6.wrapping_add(2)) == 0x0d && i_tmp == -1
                    {
                        w8(i6.wrapping_add(3), 0);
                        FUN_00433d30(
                            u_var8 as u16,
                            r8((DAT_009a1fe8 as u32).wrapping_add(0x16).wrapping_add(i11)) as u16,
                            2,
                        );
                        i_var5 = i_var5.wrapping_sub(1);
                        if i_var5 == 0 {
                            break;
                        }
                    }
                    i11 = i11.wrapping_add(0x23);
                    u_var8 = u_var8.wrapping_add(1);
                    if u_var8 >= DAT_009a1fe0 {
                        break;
                    }
                }
            }
            if i_var5 != 0 && _DAT_00a07cec < DAT_009a1fe0 {
                let mut i11: u32 = _DAT_00a07cec.wrapping_mul(0x23);
                u_var8 = _DAT_00a07cec;
                loop {
                    let i6 = (DAT_009a1fe8 as u32).wrapping_add(i11);
                    let pc_tmp = r32(i6.wrapping_add(0x1f));
                    let i_tmp = r32(i6.wrapping_add(0x1b)) as i32;
                    if r8(i6.wrapping_add(3)) != 0 && r8(pc_tmp) == 0 && i_tmp == -1 {
                        let i_var2 = r32(pc_tmp.wrapping_add(0xa9));
                        let c = r8(i_var2.wrapping_add(7));
                        if c == 3 || c == 4 {
                            w8(i6.wrapping_add(3), 0);
                            FUN_00458780(i_var2 as i32);
                            FUN_00433d30(
                                u_var8 as u16,
                                r8((DAT_009a1fe8 as u32).wrapping_add(0x16).wrapping_add(i11))
                                    as u16,
                                2,
                            );
                            i_var5 = i_var5.wrapping_sub(1);
                            if i_var5 == 0 {
                                break;
                            }
                        }
                    }
                    i11 = i11.wrapping_add(0x23);
                    u_var8 = u_var8.wrapping_add(1);
                    if u_var8 >= DAT_009a1fe0 {
                        break;
                    }
                }
            }
        }
        FUN_00456040();
    }
}
