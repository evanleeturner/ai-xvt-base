// SPDX-License-Identifier: MIT

#[repr(C)]
struct Rec {
    a: u32,
    b: u16,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8c2c: i32;
    static mut DAT_009a8c24: i32;
    static mut DAT_00a60a00: *mut u8;
    static mut DAT_00a60a10: *mut u8;
    static mut DAT_00a60a20: *mut u8;
    static mut DAT_00a60a24: *mut i8;
    static mut DAT_00557360: u32;
    static mut DAT_00557364: u32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d6940: u8;
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009d8a30: [i32; 0];
    static mut DAT_00556ed8: [u32; 0];
    static mut DAT_00557560: [u32; 0];
    static mut DAT_0052342c: i32;
    static mut DAT_005242dc: i32;
    static mut DAT_00523420: i32;
    static mut DAT_009ed23c: i32;
    static mut DAT_00a07cc8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac760() -> i32;
    fn FUN_00448da0();
    fn FUN_00448f90(a: i32, b: *mut u8, c: i32);
    fn FUN_00417a60(a: *mut u8);
    fn FUN_00416710() -> i32;
    fn FUN_00416700() -> *mut u8;
    fn FUN_00417900(a: *mut u32, b: *mut u8, c: i32) -> i32;
    fn FUN_0046f700() -> i32;
    fn FUN_0046d350(a: i32, b: *mut u32, c: u32);
    fn FUN_004aa7f0() -> i32;
    fn FUN_004aa870() -> u32;
    fn sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_0046e050(a: *mut i32, b: *mut i32) -> *mut i32;
    fn FUN_0046f660(a: i32) -> i32;
    fn FUN_00418650(a: i32);
    fn FUN_00418760(a: i32, b: u32, c: *mut u32) -> *mut i32;
    fn FUN_0046e040() -> i32;
    fn FUN_00418f80(a: *mut i32);
    fn FUN_00463cc0(a: i32);
    fn FUN_00419570(a: *mut i32, b: i32, c: i32);
    fn FUN_00463c50(a: i32);
    fn FUN_00448ed0();
    fn FUN_004195a0(a: i32, b: i32);
}

unsafe fn e680(i: i32) -> *mut i8 {
    // SAFETY: the game's own array, indexed as the original does.
    unsafe {
        (&raw mut DAT_009e9680)
            .cast::<i8>()
            .offset(i.wrapping_mul(0x5bd) as isize)
    }
}

unsafe fn d8a30(i: i32) -> *mut i32 {
    // SAFETY: the game's own array, indexed as the original does.
    unsafe { (&raw mut DAT_009d8a30).cast::<i32>().offset(i as isize) }
}

unsafe fn a6ed8(i: i32) -> *mut u32 {
    // SAFETY: the game's own array, indexed as the original does.
    unsafe { (&raw mut DAT_00556ed8).cast::<u32>().offset(i as isize) }
}

unsafe fn a7560(i: i32) -> *mut u32 {
    // SAFETY: the game's own array, indexed as the original does.
    unsafe { (&raw mut DAT_00557560).cast::<u32>().offset(i as isize) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00465a20(param_1: *mut i32) {
    // SAFETY: all globals and callees are the game's own; param_1 and the
    // packet pointers it yields point at the game's buffers.
    unsafe {
        let mut local_5c: u32 = 0;
        if param_1.read() != 0x3d {
            return;
        }
        let mut i_var3: i32;
        let mut i_var6: i32;
        let mut u_var5: u32;
        let mut pb4: *mut u8;
        let mut pb9: *mut u8;
        let mut pi7: *mut i32 = core::ptr::null_mut();
        let mut pi8: *mut i32;
        let mut rec = Rec { a: 0, b: 0 };
        let mut i_stack_64: i32 = 0;
        let mut u_stack_60: u32;
        let mut local_58: i32;
        let mut local_54: i32;
        let mut ab_stack_50: [u8; 80] = [0; 80];

        i_var3 = FUN_004ac760();
        DAT_009a8c2c = DAT_009a8c2c.wrapping_add(i_var3);
        FUN_00448da0();
        FUN_00448f90(1, DAT_00a60a00, 0x30);
        FUN_00417a60(param_1.add(2).cast::<u8>());
        DAT_00557360 = 0x3c;
        i_var3 = FUN_00416710();
        pb4 = FUN_00416700();
        local_58 = FUN_00417900(&raw mut DAT_00557364, pb4, i_var3);
        local_58 = local_58.wrapping_mul(4);
        u_var5 = (local_58 as u32).wrapping_add(4);
        loop {
            let puvar10 = &raw mut DAT_00557360;
            i_var3 = FUN_0046f700();
            FUN_0046d350(i_var3, puvar10, u_var5);
            i_var3 = FUN_004aa7f0();
            if i_var3 != 0 {
                u_var5 = FUN_004aa870();
                if u_var5 == 0x1b {
                    FUN_00463c50(DAT_009ecc34);
                    DAT_009d6940 = 1;
                    e680(DAT_009ecc34).write(0);
                    i_var3 = DAT_009ecc34;
                    d8a30(DAT_009ecc34).write(1);
                    FUN_00463cc0(i_var3);
                    return;
                }
            }
            local_54 = 0;
            loop {
                i_var3 = DAT_009a8c2c;
                i_var6 = FUN_004ac760();
                DAT_009a8c24 = DAT_009a8c24
                    .wrapping_add(DAT_009a8c2c.wrapping_add(i_var6).wrapping_sub(i_var3));
                DAT_009a8c2c = i_var3;
                if 0x1ba8 < DAT_009a8c24 {
                    FUN_00448ed0();
                    return;
                }
                u_var5 = (0x1ba8i32.wrapping_sub(DAT_009a8c24) / 0x76) as u32;
                if local_5c != u_var5 {
                    local_5c = u_var5;
                    if (u_var5 as i32) < 0x32 {
                        sprintf(
                            ab_stack_50.as_mut_ptr().cast::<i8>(),
                            DAT_00a60a24,
                            (u_var5 as i32) / 2,
                            (u_var5 & 1).wrapping_mul(5),
                        );
                        pb4 = ab_stack_50.as_mut_ptr();
                    } else {
                        pb4 = DAT_00a60a10;
                        if (u_var5 & 1) != 0 {
                            pb4 = DAT_00a60a20;
                        }
                    }
                    FUN_00448f90(3, pb4, 0x30);
                }
                pi7 = FUN_0046e050(&raw mut i_stack_64, &raw mut local_58);
                i_var3 = DAT_009a8c2c;
                DAT_009a8c2c = i_var3;
                if !pi7.is_null() {
                    match pi7.read() {
                        1 => {
                            i_var3 = FUN_0046f660(i_stack_64);
                            if e680(i_var3).read() == 0 {
                                DAT_00557360 = 0x15;
                                DAT_00557364 = i_var3 as u32;
                                FUN_0046d350(i_stack_64, &raw mut DAT_00557360, 8);
                            } else {
                                if 0 < (a6ed8(i_var3).read() as i32) {
                                    a6ed8(i_var3).write(0);
                                }
                                FUN_00418650(i_var3);
                                let p = pi7.cast::<u8>();
                                rec.a &= 0xff000000;
                                let b_var1 = p.add(4).read();
                                if (b_var1 & 0x7f) == 0x7f {
                                    u_var5 = p.add(5).cast::<u32>().read_unaligned();
                                    if (b_var1 & 0x80) == 0 {
                                        pb4 = p.add(9);
                                    } else {
                                        pb4 = p.add(10);
                                        rec.a = (rec.a & 0xff000000)
                                            | ((p.add(9).read() as u32) << 0x10);
                                    }
                                } else {
                                    u_var5 = a7560(i_var3).read();
                                    u_stack_60 = (b_var1 & 0x7f) as u32;
                                    if u_stack_60 < (u_var5 & 0x7f) {
                                        u_var5 = u_var5.wrapping_add(0x80);
                                    }
                                    u_var5 = (u_var5 & 0xffffff80) | u_stack_60;
                                    if (b_var1 & 0x80) == 0 {
                                        pb4 = p.add(5);
                                    } else {
                                        pb4 = p.add(6);
                                        rec.a = (rec.a & 0xff000000)
                                            | ((p.add(5).read() as u32) << 0x10);
                                    }
                                }
                                a7560(i_var3).write(u_var5);
                                rec.a = (((pb4.read() as u32) << 24) | (rec.a & 0x00ffffff))
                                    & 0xfeffffff;
                                let t = (((pb4.add(1).read() & 1) * 2) | (pb4.read() & 1)) as u16;
                                rec.b = ((t << 8) | pb4.add(1).read() as u16) & 0xfffe;
                                pi8 = FUN_00418760(i_var3, u_var5, (&raw mut rec).cast::<u32>());
                                if !pi8.is_null() {
                                    i_var3 = FUN_0046e040();
                                    pi8.write(1);
                                    if i_var3 == 0 {
                                        pi8.write(0);
                                    }
                                    pi8.add(1).write(1);
                                }
                            }
                        }
                        2 => {
                            FUN_00418f80(pi7);
                            FUN_004ac760();
                            DAT_009a8c2c = i_var3;
                            if DAT_009d6940 != 0 {
                                return;
                            }
                        }
                        8 => {
                            DAT_009d6940 = 1;
                            DAT_0052342c = 1;
                            e680(DAT_009ecc34).write(0);
                            return;
                        }
                        0x15 => {
                            i_var3 = pi7.add(1).read();
                            if -1 < i_var3 && i_var3 < 8 {
                                d8a30(i_var3).write(1);
                            }
                            if DAT_009ecc34 == i_var3 {
                                DAT_009d6940 = 1;
                                e680(DAT_009ecc34).write(0);
                                i_var3 = DAT_009ecc34;
                                d8a30(DAT_009ecc34).write(1);
                                FUN_00463cc0(i_var3);
                                return;
                            }
                        }
                        0x16 => {
                            i_var3 = FUN_0046f660(i_stack_64);
                            if e680(i_var3).read() == 0 {
                                DAT_00557360 = 0x15;
                                DAT_00557364 = i_var3 as u32;
                                FUN_0046d350(i_stack_64, &raw mut DAT_00557360, 8);
                            } else {
                                if 0 < (a6ed8(i_var3).read() as i32) {
                                    a6ed8(i_var3).write(0);
                                }
                                FUN_00418650(i_var3);
                                pb4 = pi7.cast::<u8>().add(5);
                                rec.a = 0;
                                rec.b = 0;
                                u_var5 = pi7.add(1).cast::<u8>().read() as u32;
                                if pi7.add(1).cast::<u8>().read() != 0 {
                                    loop {
                                        u_stack_60 = u_var5;
                                        let b_var1 = pb4.read();
                                        let b_var2 = b_var1 & 0x7f;
                                        if b_var2 == 0x7f {
                                            u_var5 = pb4.add(1).cast::<u32>().read_unaligned();
                                            if (b_var1 & 0x80) == 0 {
                                                pb9 = pb4.add(5);
                                            } else {
                                                pb9 = pb4.add(6);
                                                rec.a = (rec.a & 0xffff)
                                                    | ((pb4.add(5).read() as u32) << 16);
                                                rec.a &= 0x00ffffff;
                                            }
                                        } else {
                                            u_var5 = a7560(i_var3).read();
                                            if (b_var2 as u32) < (u_var5 & 0x7f) {
                                                u_var5 = u_var5.wrapping_add(0x80);
                                            }
                                            u_var5 = (u_var5 & 0xffffff80) | b_var2 as u32;
                                            if (b_var1 & 0x80) == 0 {
                                                pb9 = pb4.add(1);
                                            } else {
                                                pb9 = pb4.add(2);
                                                rec.a = (rec.a & 0xffff)
                                                    | ((pb4.add(1).read() as u32) << 16);
                                                rec.a &= 0x00ffffff;
                                            }
                                        }
                                        pb4 = pb9.add(2);
                                        a7560(i_var3).write(u_var5);
                                        rec.a = (((pb9.read() as u32) << 24)
                                            | (rec.a & 0x00ffffff))
                                            & 0xfeffffff;
                                        let t = (((pb9.add(1).read() & 1) * 2) | (pb9.read() & 1))
                                            as u16;
                                        rec.b = ((t << 8) | pb9.add(1).read() as u16) & 0xfffe;
                                        pi8 = FUN_00418760(
                                            i_var3,
                                            u_var5,
                                            (&raw mut rec).cast::<u32>(),
                                        );
                                        if !pi8.is_null() {
                                            i_var6 = FUN_0046e040();
                                            pi8.write(1);
                                            if i_var6 == 0 {
                                                pi8.write(0);
                                            }
                                            pi8.add(1).write(1);
                                        }
                                        u_var5 = u_stack_60.wrapping_sub(1);
                                        if u_stack_60.wrapping_sub(1) == 0 {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        0x17 => {
                            DAT_005242dc = pi7.add(1).read();
                        }
                        0x1d => {
                            i_var3 = FUN_0046f700();
                            if i_var3 == i_stack_64 {
                                DAT_009a8c24 = 0;
                            } else {
                                i_var3 = FUN_0046f660(i_stack_64);
                                if e680(i_var3).read() != 0 && 0 < (a6ed8(i_var3).read() as i32) {
                                    a6ed8(i_var3).write(0);
                                }
                            }
                        }
                        0x3d => {
                            FUN_00463c50(DAT_009ecc34);
                            DAT_009d6940 = 1;
                            e680(DAT_009ecc34).write(0);
                            i_var3 = DAT_009ecc34;
                            d8a30(DAT_009ecc34).write(1);
                            FUN_00463cc0(i_var3);
                            return;
                        }
                        0x3e | 0x3f => {
                            if pi7.add(1).read() == DAT_00523420 {
                                local_54 = 1;
                            }
                        }
                        _ => {}
                    }
                }
                i_var3 = DAT_009ecc34;
                if local_54 != 0 {
                    break;
                }
            }
            if pi7.read() == 0x3e {
                break;
            }
            i_var3 = pi7.add(2).read();
            i_var6 = pi7.add(3).read();
            while i_var6 != -1 {
                FUN_00419570(pi7.add(5), pi7.add(3).read(), pi7.add(4).read());
                pi7 = pi7.byte_offset(pi7.add(4).read().wrapping_add(8) as isize);
                i_var6 = pi7.add(3).read();
            }
            u_var5 = 8;
            DAT_00557360 = 0x10;
            DAT_00557364 = i_var3 as u32;
        }
        if d8a30(DAT_009ecc34).read() != 0 {
            d8a30(DAT_009ecc34).write(1);
            DAT_009d6940 = 1;
            e680(i_var3).write(0);
            FUN_00463cc0(DAT_009ecc34);
            return;
        }
        FUN_004ac760();
        FUN_004195a0(pi7.add(2).read(), pi7.add(1).read());
        DAT_00557360 = 0x1b;
        let puvar10 = &raw mut DAT_00557360;
        i_var3 = FUN_0046f700();
        FUN_0046d350(i_var3, puvar10, 4);
        DAT_009a8c2c = DAT_009ed23c.wrapping_add(DAT_00a07cc8);
        FUN_00448ed0();
    }
}
