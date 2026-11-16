// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a20ce: i16;
    static mut DAT_009a20e6: i16;
    static mut DAT_0051a44f: [i8; 256];
    static mut DAT_009d77fc: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_009d6828: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_009cd260: i32;
    static mut DAT_00a08298: i32;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00434500(param_1: u32, param_2: u32);
    fn FUN_00406d10(param_1: u16, param_2: u16);
    fn FUN_00459750(param_1: i32) -> u16;
    fn FUN_00425d10() -> u16;
    fn FUN_0042e4d0(param_1: u32, param_2: u32, param_3: i32);
}

#[inline(always)]
unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address is the game's own memory.
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
unsafe fn wb(a: u32, v: u8) {
    // SAFETY: address is the game's own memory.
    unsafe { (a as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn rw(a: u32) -> u16 {
    // SAFETY: address is the game's own memory.
    unsafe { (a as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn ww(a: u32, v: u16) {
    // SAFETY: address is the game's own memory.
    unsafe { (a as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn rd(a: u32) -> u32 {
    // SAFETY: address is the game's own memory.
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wd(a: u32, v: u32) {
    // SAFETY: address is the game's own memory.
    unsafe { (a as *mut u32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00446960(param_1: u16, param_2: u32) {
    // SAFETY: all globals, callees and pointed-to records are the game's own
    // memory, accessed at the offsets the original uses.
    unsafe {
        let mut u_var6: u32 = param_1 as u32;
        let mut i_var7: u32 = (param_2 & 0xffff).wrapping_mul(0x23);
        let i_var4: u32 = (DAT_009a1fe8 as u32).wrapping_add(i_var7);
        let c5: i8;
        'lab: {
            if rb(i_var4.wrapping_add(2)) as i8 == 10 {
                let c = rb((param_1 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8 as u32)) as i8;
                if c != -0x73 && c != -0x72 {
                    c5 = -0x7d;
                    break 'lab;
                }
            } else {
                if (param_1 as i32) < DAT_009a1fe0 {
                    let p = (&raw mut DAT_009a20ce as u32)
                        .wrapping_add((rb(i_var4.wrapping_add(0x16)) as u32).wrapping_mul(0x126));
                    ww(p, rw(p).wrapping_add(1));
                    wb(i_var4.wrapping_add(3), 0);
                    FUN_00434500(u_var6, param_2);
                    c5 = -0x7f;
                    break 'lab;
                }
                let i_var2: u32 = (param_1 as u32).wrapping_mul(0x23);
                let i_var1: u32 = (DAT_009a1fe8 as u32).wrapping_add(i_var2);
                let c = rb((DAT_009a1fe8 as u32).wrapping_add(3).wrapping_add(i_var2)) as i8;
                if c != -0x73 && c != -0x72 {
                    let p = (&raw mut DAT_009a20ce as u32)
                        .wrapping_add((rb(i_var4.wrapping_add(0x16)) as u32).wrapping_mul(0x126));
                    ww(p, rw(p).wrapping_add(1));
                    if rb(i_var4.wrapping_add(3)) == b'M' {
                        let p_obj = rd(i_var1.wrapping_add(0x1f));
                        let us = rw(p_obj.wrapping_add(0x85));
                        FUN_00406d10(param_2 as u16, us);
                    }
                    wb(
                        (DAT_009a1fe8 as u32).wrapping_add(3).wrapping_add(i_var7),
                        0,
                    );
                    let p_obj = rd((DAT_009a1fe8 as u32)
                        .wrapping_add(0x1f)
                        .wrapping_add(i_var2));
                    let us = rw(p_obj.wrapping_add(0x85));
                    FUN_00434500((param_2 & 0xffff0000) | (us as u32), param_2);
                    c5 = -0x7f;
                    break 'lab;
                }
                ww(i_var4.wrapping_add(0x17), 0);
                let p = (&raw mut DAT_009a20e6 as u32).wrapping_add(
                    (rb((DAT_009a1fe8 as u32)
                        .wrapping_add(0x16)
                        .wrapping_add(i_var7)) as u32)
                        .wrapping_mul(0x126),
                );
                ww(p, rw(p).wrapping_add(1));
            }
            c5 = -0x7c;
        }
        let mut c5 = c5;
        if (param_1 as i32) < DAT_009a1fe0 {
            let u_var3 = FUN_00459750(0xd);
            u_var6 = u_var3 as u32;
            if u_var3 == 0xffff {
                return;
            }
            wb(
                u_var6
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8 as u32),
                0x89,
            );
        }
        u_var6 &= 0xffff;
        i_var7 = u_var6.wrapping_mul(0x23);

        let rec =
            |off: u32| -> u32 { (DAT_009a1fe8 as u32).wrapping_add(off).wrapping_add(i_var7) };

        wd(
            rec(4),
            (DAT_009d77fc as u32).wrapping_add(DAT_009ec604 as u32),
        );
        wd(
            rec(8),
            (DAT_009d6828 as u32).wrapping_add(DAT_00a081e8 as u32),
        );
        wd(
            rec(0xc),
            (DAT_009cd260 as u32).wrapping_add(DAT_00a08298 as u32),
        );

        let idx = rb(rec(3)) as usize;
        if (&raw const DAT_0051a44f).cast::<i8>().add(idx).read() != 0 {
            c5 = -0x7f;
        }
        wb(rec(3), c5 as u8);
        wb(rec(2), 0xd);

        let p_obj = rd(rec(0x1f));
        wb(p_obj, 5);
        wb(rec(0x19), 2);
        let p_obj = rd(rec(0x1f));
        ww(p_obj.wrapping_add(0x79), 0);
        let p_obj = rd(rec(0x1f));
        wb(p_obj.wrapping_add(1), 0);
        let p_obj = rd(rec(0x1f));
        ww(p_obj.wrapping_add(0x83), 0);
        let p_obj = rd(rec(0x1f));
        ww(p_obj.wrapping_add(0x81), 0);
        ww(rec(0x12), 0);
        ww(rec(0x10), 0);
        ww(rec(0x14), 0);
        let p_obj = rd(rec(0x1f));
        wb(p_obj.wrapping_add(0x92), 1);
        let p_obj = rd(rec(0x1f));
        wb(p_obj.wrapping_add(0x8b), rb(p_obj.wrapping_add(0x92)));

        if c5 != -0x7d && c5 != -0x7c {
            let i7 = DAT_009ecc34;
            let u_var3 = FUN_00425d10();
            FUN_0042e4d0(((u_var3 & 3) + 0x16) as u32, u_var6, i7);
            return;
        }
        FUN_0042e4d0(0x1c, u_var6, DAT_009ecc34);
    }
}
