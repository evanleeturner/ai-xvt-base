// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0072f444: i32;
    static mut DAT_0052552c: i32;
    static mut DAT_0081e016: [u8; 4];
    static mut DAT_0071bcb6: u16;
    static mut DAT_0071b738: u16;
    static mut DAT_005373d0: [u32; 1];
    static mut DAT_006a89c8: *mut i8;
    static mut DAT_006a89cc: *mut i8;
    static mut DAT_006a89d0: *mut i8;
    static mut DAT_006a89d4: *mut i8;
    static mut DAT_00732e88: i32;
    static mut DAT_0071ecf4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432ca0() -> i32;
    fn FUN_00431d30() -> i32;
    fn FUN_004330b0() -> i32;
    fn FUN_00438a40();
    fn FUN_00438c30(a: i32, b: *mut i8, c: i32);
    fn FUN_00438b70();
    fn FUN_00432b70(a: *mut i32, b: *mut i32, c: i32) -> *mut i32;
    fn FUN_00432c10(a: i32) -> i32;
    fn FUN_00432cc0(a: i32) -> *mut i32;
    fn FUN_004310c0(a: *mut u32, b: i32);
    fn FUN_00431110(a: i32, b: *mut u32, c: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

unsafe fn str_len(s: *const u8) -> usize {
    let mut n = 0usize;
    // SAFETY: s points at a nul-terminated string owned by the game.
    unsafe {
        while *s.add(n) != 0 {
            n += 1;
        }
    }
    n
}

unsafe fn copy_str(dst: *mut u8, src: *const u8) {
    // SAFETY: src is nul-terminated; dst has room as in the original.
    unsafe {
        let n = str_len(src) + 1;
        let mut i = 0usize;
        while i < n {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}

unsafe fn chat_block(local_114: i32, bvar3: &mut i32) {
    let mut local_100 = [0u8; 256];
    let buf = local_100.as_mut_ptr();
    // SAFETY: the callees and globals are the game's own; strings are
    // nul-terminated and copied where the original copies them.
    unsafe {
        let i_var5 = FUN_00432c10(local_114);
        let pi_var9 = FUN_00432cc0(i_var5);
        if pi_var9.is_null() {
            copy_str(buf, DAT_006a89cc as *const u8);
        } else {
            copy_str(buf, pi_var9 as *const u8);
            *bvar3 = (*bvar3 == 0) as i32;
            let src = if *bvar3 != 0 {
                DAT_006a89d0
            } else {
                DAT_006a89d4
            };
            let end = buf.add(str_len(buf));
            copy_str(end, src as *const u8);
        }
        FUN_00438c30(3, buf as *mut i8, 0x30);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042bc90() -> u32 {
    let mut bvar3: i32 = 1;
    let mut local_110: u32 = 0;
    let mut local_114: i32 = 0;
    let mut local_104: i32 = 0;
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers returned by the game are used as the original uses them.
    unsafe {
        let mut i_var5 = FUN_00432ca0();
        if DAT_0072f444 < 2 {
            DAT_0071bcb6 = DAT_0052552c as u16;
            let uv = (&raw const DAT_0081e016).cast::<u32>().read_unaligned();
            DAT_0071b738 = uv as u16;
        } else {
            let i_var6 = FUN_00431d30();
            let t = (&raw mut DAT_005373d0).cast::<u32>();
            let base = (&raw mut DAT_0071bcb6).cast::<u8>();
            if i_var6 == 0 {
                t.add(1).write(DAT_0052552c as u32);
                t.add(2)
                    .write_unaligned((&raw const DAT_0081e016).cast::<u32>().read_unaligned());
                t.write(0x11);
                FUN_00431110(i_var5, t, 0xc);
                FUN_00438a40();
                FUN_00438c30(1, DAT_006a89c8, 0x30);
                let mut pi_var7: *mut i32;
                loop {
                    pi_var7 = FUN_00432b70(&raw mut local_114, &raw mut local_104, 0x3c);
                    if pi_var7.is_null() {
                        return 0;
                    }
                    if *pi_var7 == 0x1d {
                        let d = timeGetTime();
                        if 200 < d.wrapping_sub(local_110) as i32 {
                            local_110 = d;
                            chat_block(local_114, &mut bvar3);
                        }
                    }
                    if *pi_var7 == 0x12 {
                        break;
                    }
                }
                FUN_00438b70();
                DAT_00732e88 = pi_var7.add(1).read();
                if 0 < DAT_0072f444 {
                    i_var5 = DAT_0072f444;
                    let mut p = base;
                    loop {
                        p.cast::<u16>()
                            .write_unaligned(pi_var7.add(2).read() as u16);
                        i_var5 = i_var5.wrapping_sub(1);
                        p.byte_sub(0x57e)
                            .cast::<u16>()
                            .write_unaligned(pi_var7.add(3).read() as u16);
                        p = p.byte_add(0x5bd);
                        pi_var7 = pi_var7.add(2);
                        if i_var5 == 0 {
                            break;
                        }
                    }
                }
            } else {
                if 0 < DAT_0072f444 {
                    i_var5 = DAT_0072f444;
                    let mut p = base;
                    loop {
                        p.cast::<u16>().write_unaligned(0x13);
                        p.byte_sub(0x57e).cast::<u16>().write_unaligned(0);
                        i_var5 = i_var5.wrapping_sub(1);
                        p = p.byte_add(0x5bd);
                        if i_var5 == 0 {
                            break;
                        }
                    }
                }
                FUN_004330b0();
                let uv = (&raw const DAT_0081e016).cast::<u32>().read_unaligned();
                let off = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as i32;
                base.offset(off as isize)
                    .cast::<u16>()
                    .write_unaligned(DAT_0052552c as u16);
                (&raw mut DAT_0071b738)
                    .cast::<u8>()
                    .offset(off as isize)
                    .cast::<u16>()
                    .write_unaligned(uv as u16);
                FUN_00438a40();
                FUN_00438c30(1, DAT_006a89c8, 0x30);
                let mut local_10c: i32 = 0;
                let mut local_108: u32 = timeGetTime();
                loop {
                    let r = FUN_004330b0();
                    if !(local_10c < (r as u32).wrapping_sub(1) as i32) {
                        break;
                    }
                    let pi_var7 = FUN_00432b70(&raw mut local_114, &raw mut local_104, 0x3c);
                    let d = timeGetTime();
                    if pi_var7.is_null() {
                        if 60000 < d.wrapping_sub(local_108) {
                            return 0;
                        }
                    } else {
                        local_108 = d;
                        if *pi_var7 == 0x1d {
                            let d2 = timeGetTime();
                            if 200 < d2.wrapping_sub(local_110) as i32 {
                                local_110 = d2;
                                chat_block(local_114, &mut bvar3);
                            }
                        }
                        if *pi_var7 == 0x11 {
                            let r5 = FUN_00432c10(local_114);
                            let o = (r5 as u32).wrapping_mul(0x5bd) as i32;
                            base.offset(o as isize)
                                .cast::<u16>()
                                .write_unaligned(pi_var7.add(1).read() as u16);
                            (&raw mut DAT_0071b738)
                                .cast::<u8>()
                                .offset(o as isize)
                                .cast::<u16>()
                                .write_unaligned(pi_var7.add(2).read() as u16);
                            local_10c = local_10c.wrapping_add(1);
                        }
                    }
                }
                FUN_00438b70();
                t.add(1).write(DAT_00732e88 as u32);
                t.write(0x12);
                if 0 < DAT_0072f444 {
                    i_var5 = DAT_0072f444;
                    let mut pu = t.add(2);
                    let mut p = base;
                    loop {
                        pu.write(p.cast::<u16>().read_unaligned() as u32);
                        i_var5 = i_var5.wrapping_sub(1);
                        pu.add(1)
                            .write(p.byte_sub(0x57e).cast::<u16>().read_unaligned() as u32);
                        pu = pu.add(2);
                        p = p.byte_add(0x5bd);
                        if i_var5 == 0 {
                            break;
                        }
                    }
                }
                FUN_004310c0(
                    t,
                    (DAT_0072f444 as u32).wrapping_mul(8).wrapping_add(8) as i32,
                );
                loop {
                    let pi_var7 = FUN_00432b70(&raw mut local_114, &raw mut local_104, 0x3c);
                    if pi_var7.is_null() {
                        return 0;
                    }
                    if *pi_var7 == 0x12 {
                        break;
                    }
                }
            }
        }
    }
    1
}
