// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052bf50: i32;
    static mut DAT_00518278: i32;
    static mut DAT_0051827c: i32;
    static mut DAT_00518280: i32;
    static mut DAT_00518284: i32;
    static mut DAT_00bb72d0: u8;
    static mut DAT_00bb73a7: i8;
    static mut DAT_00aa6cf4: u32;
    static mut DAT_00aa62b0: u8;
    static mut DAT_00aa62d0: u8;
    static mut DAT_0052bff4: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfc20(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        buf: *mut core::ffi::c_void,
        cap: u32,
        flag: u32,
    ) -> u32;
    fn FUN_004d9280(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_004de5a0(name: *const i8, a: i32, b: i32, c: i32);
    fn FUN_004d9680();
}

unsafe fn guid_call(p: *const u8) -> i32 {
    // SAFETY: p points at 16 readable bytes of a GUID in the game's tables.
    unsafe {
        let g = p.cast::<[i32; 4]>().read_unaligned();
        FUN_004d9280(g[0], g[1], g[2], g[3])
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d92b0() -> u32 {
    let mut local_600 = [0i32; 0x180];
    let mut b_var3: i32 = 0;
    // SAFETY: globals and callees are the game's own; all pointers stay inside
    // the game's 0x3e-byte entry tables and the local enumeration buffer.
    unsafe {
        let local_5e0: *mut u8 = local_600.as_mut_ptr().add(8).cast::<u8>();
        if DAT_0052bf50 != -1 {
            return 0;
        }
        let mut s_var4 = FUN_004cfc20(
            DAT_00518278,
            DAT_0051827c,
            DAT_00518280,
            DAT_00518284,
            local_600.as_mut_ptr().cast::<core::ffi::c_void>(),
            0x20,
            DAT_00bb72d0 as u32,
        );
        let mut local_608: u32 = 0;
        if DAT_00aa6cf4 != 0 {
            let mut p = &raw mut DAT_00aa62d0;
            loop {
                let i_var5 = guid_call(p);
                if i_var5 != 0 {
                    let mut s_var13: u32 = 0;
                    if s_var4 != 0 {
                        let mut p12 = local_5e0;
                        loop {
                            let i_var6 = guid_call(p12);
                            if i_var6 != 0 && i_var6 == i_var5 {
                                core::ptr::write_bytes(p12, 0, 16);
                                break;
                            }
                            p12 = p12.byte_add(0x30);
                            s_var13 = s_var13.wrapping_add(1);
                            if !(s_var13 < s_var4) {
                                break;
                            }
                        }
                    }
                    if s_var13 == s_var4 {
                        core::ptr::write_bytes(p.byte_sub(0x20), 0, 0x3e);
                    }
                }
                p = p.byte_add(0x3e);
                local_608 = local_608.wrapping_add(1);
                if !(local_608 < DAT_00aa6cf4) {
                    break;
                }
            }
        }
        if s_var4 != 0 {
            let mut p = local_5e0;
            loop {
                let i_var5 = guid_call(p);
                if i_var5 != 0 {
                    let off = DAT_00aa6cf4.wrapping_mul(0x3e) as i32 as isize;
                    let name = p.byte_sub(0x20);
                    let mut n: usize = 0;
                    while *name.add(n) != 0 {
                        n += 1;
                    }
                    n += 1;
                    let dst = (&raw mut DAT_00aa62b0).byte_offset(off);
                    core::ptr::copy(name, dst, n);
                    let i_var6 = p.cast::<i32>().add(1).read_unaligned();
                    let i_var1 = p.cast::<i32>().add(2).read_unaligned();
                    let i_tmp = p.cast::<i32>().read_unaligned();
                    let g = (&raw mut DAT_00aa62d0).byte_offset(off);
                    g.cast::<i32>().write_unaligned(i_tmp);
                    DAT_00aa6cf4 = DAT_00aa6cf4.wrapping_add(1);
                    let i_var2 = p.cast::<i32>().add(3).read_unaligned();
                    g.byte_add(4).cast::<i32>().write_unaligned(i_var6);
                    b_var3 = 1;
                    g.byte_add(8).cast::<i32>().write_unaligned(i_var1);
                    g.byte_add(0xc).cast::<i32>().write_unaligned(i_var2);
                    g.byte_add(0x10).cast::<i32>().write_unaligned(9);
                    g.byte_add(0x14).cast::<i32>().write_unaligned(0);
                    g.byte_add(0x18).cast::<i32>().write_unaligned(0x65);
                    g.byte_add(0x1c).write(0);
                }
                p = p.byte_add(0x30);
                s_var4 = s_var4.wrapping_sub(1);
                if s_var4 == 0 {
                    break;
                }
            }
        }
        if b_var3 != 0 && DAT_00bb73a7 != 0 {
            FUN_004de5a0((&raw mut DAT_0052bff4).cast::<i8>(), 1, 0, 0xff);
        }
        let mut u8_v: u32 = 0;
        if DAT_00aa6cf4 != 0 {
            let base = &raw mut DAT_00aa62d0;
            let mut p = base;
            loop {
                let i_var5 = guid_call(p);
                let mut u9 = DAT_00aa6cf4;
                if i_var5 == 0 {
                    let mut u15 = u8_v.wrapping_add(1);
                    if u15 < DAT_00aa6cf4 {
                        let mut p12 = base.byte_add((u15 as usize).wrapping_mul(0x3e));
                        loop {
                            let r = guid_call(p12);
                            u9 = DAT_00aa6cf4;
                            if r == 0 {
                                p12 = p12.byte_add(0x3e);
                                u15 = u15.wrapping_add(1);
                                if DAT_00aa6cf4 <= u15 {
                                    break;
                                }
                                continue;
                            }
                            let src =
                                (&raw mut DAT_00aa62b0).byte_add((u15 as usize).wrapping_mul(0x3e));
                            core::ptr::copy(src, p.byte_sub(0x20), 0x3e);
                            core::ptr::write_bytes(src, 0, 0x3e);
                            break;
                        }
                    }
                }
                p = p.byte_add(0x3e);
                u8_v = u8_v.wrapping_add(1);
                if !(u8_v < u9) {
                    break;
                }
            }
        }
        DAT_00aa6cf4 = 0;
        loop {
            let off = DAT_00aa6cf4.wrapping_mul(0x3e) as i32 as isize;
            let r = guid_call((&raw mut DAT_00aa62d0).byte_offset(off));
            if r == 0 {
                break;
            }
            DAT_00aa6cf4 = DAT_00aa6cf4.wrapping_add(1);
            if !(DAT_00aa6cf4 < 0x20) {
                break;
            }
        }
        FUN_004d9680();
        DAT_00aa6cf4
    }
}
