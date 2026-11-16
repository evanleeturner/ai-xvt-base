// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db5a8: i32;
    static mut DAT_006db5a4: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bce5: u8;
    static mut DAT_0071bcb8: u8;
    static mut DAT_0071b740: u8;
    static mut DAT_0070aaf0: i32;
    static mut DAT_006a89a0: *mut u8;
    static mut DAT_006a89b0: *mut u8;
    static mut DAT_006a89c0: *mut u8;
    static mut DAT_006a89c4: *mut i8;
    static mut DAT_005373d0: u32;
    static mut DAT_005373d4: u32;
    static mut DAT_005375d0: u32;
    static mut DAT_00536f48: i32;
    static mut DAT_007092c0: u8;
    static mut DAT_0052556c: i32;
    static mut DAT_00525560: i32;
    static mut DAT_005250d4: i32;
    static mut DAT_00732a18: i32;
    static mut DAT_0071f2ec: i32;
    static mut DAT_0081f7de: u8;
    static mut DAT_0081f812: u8;
    static mut DAT_0081fa9a: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f530() -> i32;
    fn FUN_004a7510();
    fn FUN_00438a40();
    fn FUN_00438c30(a: i32, b: *mut u8, c: i32);
    fn FUN_0049a2f0(a: *mut u8);
    fn FUN_00498fb0() -> i32;
    fn FUN_00498fa0() -> *mut u8;
    fn FUN_0049a190(a: *mut u32, b: *mut u8, c: i32) -> i32;
    fn FUN_00432ca0() -> i32;
    fn FUN_00431110(a: i32, b: *mut u32, c: u32);
    fn FUN_004a53f0() -> i32;
    fn FUN_004a5470() -> u32;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_00431d40(a: *mut i32, b: *mut i32) -> *mut i32;
    fn FUN_00432c10(a: i32) -> i32;
    fn FUN_0049af60(a: i32);
    fn FUN_0049b0d0(a: i32, b: u32, c: *mut core::ffi::c_void) -> *mut i32;
    fn FUN_00431d30() -> i32;
    fn FUN_0049b8a0(a: *mut i32);
    fn FUN_0049bf90(a: *mut i32, b: i32, c: i32);
    fn FUN_004310c0(a: *mut u32, b: i32);
    fn FUN_00438b70();
    fn FUN_004a74f0();
    fn FUN_0049bfc0(a: i32, b: i32);
}

unsafe fn tail(off: i32) {
    // SAFETY: the tables are the game's own memory, read and written where the original does.
    unsafe {
        let mut p = (&raw mut DAT_0081f7de) as *mut u8;
        let mut i: i32 = 0;
        let limit = ((&raw mut DAT_0081fa9a) as usize).wrapping_add(3);
        loop {
            let a = p.cast::<i32>().read_unaligned();
            let b = (&raw mut DAT_0071bcb8)
                .cast::<u8>()
                .offset(off as isize)
                .cast::<i32>()
                .read_unaligned();
            if a == b {
                break;
            }
            p = p.byte_add(0x58);
            i = i.wrapping_add(1);
            if (p as usize) > limit {
                i = 0;
                break;
            }
        }
        (&raw mut DAT_0081f812)
            .cast::<u8>()
            .offset((i.wrapping_mul(0x58)) as isize)
            .cast::<i32>()
            .write_unaligned(1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042e7d0(param_1: *mut i32) {
    // SAFETY: all globals and callees are the game's own; pointers come from
    // the caller and the game's packet buffers, accessed as the original does.
    unsafe {
        let mut local_5c: u32 = 0;
        let mut local_58: i32 = 0;
        let mut i_stack_64: i32 = 0;
        let mut ab_stack_50: [u8; 80] = [0; 80];
        let mut a6c: [u32; 2] = [0; 2];
        let mut i_var3: i32;
        let mut u_var5: u32;
        let mut pi_var7: *mut i32;

        if *param_1 != 0x3d {
            return;
        }
        i_var3 = FUN_0048f530();
        DAT_006db5a8 = DAT_006db5a8.wrapping_add(i_var3);
        let t = (&raw mut DAT_0071bce5)
            .cast::<u8>()
            .offset(DAT_0071ecf4.wrapping_mul(0x5bd) as isize)
            .cast::<i32>()
            .read_unaligned();
        if t != 0 {
            FUN_004a7510();
        }
        FUN_00438a40();
        FUN_00438c30(1, DAT_006a89a0, 0x30);
        FUN_0049a2f0(param_1.add(2).cast::<u8>());
        DAT_005373d0 = 0x3c;
        i_var3 = FUN_00498fb0();
        let pb = FUN_00498fa0();
        local_58 = FUN_0049a190(&raw mut DAT_005373d4, pb, i_var3);
        local_58 = local_58.wrapping_mul(4);
        u_var5 = (local_58 as u32).wrapping_add(4);

        'outer: loop {
            let puv = &raw mut DAT_005373d0;
            i_var3 = FUN_00432ca0();
            FUN_00431110(i_var3, puv, u_var5);
            i_var3 = FUN_004a53f0();
            if i_var3 != 0 {
                u_var5 = FUN_004a5470();
                if u_var5 == 0x1b {
                    DAT_005373d4 = DAT_0071ecf4 as u32;
                    DAT_005373d0 = 0x15;
                    FUN_004310c0(&raw mut DAT_005373d0, 8);
                    let i6 = DAT_0071ecf4;
                    DAT_007092c0 = 1;
                    let off = DAT_0071ecf4.wrapping_mul(0x5bd);
                    (&raw mut DAT_0071b740).offset(off as isize).write(0);
                    (&raw mut DAT_0070aaf0).offset(i6 as isize).write(1);
                    tail(off);
                    return;
                }
            }
            let mut local_54: i32 = 0;
            loop {
                i_var3 = DAT_006db5a8;
                let i_var6 = FUN_0048f530();
                DAT_006db5a4 = DAT_006db5a4
                    .wrapping_add(DAT_006db5a8.wrapping_add(i_var6).wrapping_sub(i_var3));
                DAT_006db5a8 = i_var3;
                if 0x1ba8 < DAT_006db5a4 {
                    FUN_00438b70();
                    return;
                }
                u_var5 = ((0x1ba8u32.wrapping_sub(DAT_006db5a4 as u32)) as i32 / 0x76) as u32;
                if u_var5 != local_5c {
                    local_5c = u_var5;
                    let pbv: *mut u8;
                    if (u_var5 as i32) < 0x32 {
                        sprintf(
                            ab_stack_50.as_mut_ptr().cast::<i8>(),
                            DAT_006a89c4,
                            (u_var5 as i32) / 2,
                            ((u_var5 & 1).wrapping_mul(5)) as i32,
                        );
                        pbv = ab_stack_50.as_mut_ptr();
                    } else {
                        let mut q = DAT_006a89b0;
                        if (u_var5 & 1) != 0 {
                            q = DAT_006a89c0;
                        }
                        pbv = q;
                    }
                    FUN_00438c30(3, pbv, 0x30);
                }
                pi_var7 = FUN_00431d40(&raw mut i_stack_64, &raw mut local_58);
                i_var3 = DAT_006db5a8;
                DAT_006db5a8 = i_var3;
                if !pi_var7.is_null() {
                    match *pi_var7 {
                        1 => {
                            i_var3 = FUN_00432c10(i_stack_64);
                            if (&raw mut DAT_0071b740)
                                .offset(i_var3.wrapping_mul(0x5bd) as isize)
                                .read()
                                == 0
                            {
                                DAT_005373d0 = 0x15;
                                DAT_005373d4 = i_var3 as u32;
                                FUN_00431110(i_stack_64, &raw mut DAT_005373d0, 8);
                            } else {
                                if 0 < (&raw mut DAT_00536f48).offset(i_var3 as isize).read() {
                                    (&raw mut DAT_00536f48).offset(i_var3 as isize).write(0);
                                }
                                FUN_0049af60(i_var3);
                                a6c[0] = 0;
                                let base = pi_var7.cast::<u8>();
                                let b1 = base.add(4).read();
                                let pb4: *mut u8;
                                if (b1 & 0x7f) == 0x7f {
                                    u_var5 = base.add(5).cast::<u32>().read_unaligned();
                                    if (b1 & 0x80) == 0 {
                                        pb4 = base.add(9);
                                    } else {
                                        pb4 = base.add(10);
                                        a6c[0] = (base.add(9).read() as u32) << 0x10;
                                    }
                                } else {
                                    u_var5 = (&raw mut DAT_005375d0).offset(i_var3 as isize).read();
                                    let u60 = (b1 & 0x7f) as u32;
                                    if u60 < (u_var5 & 0x7f) {
                                        u_var5 = u_var5.wrapping_add(0x80);
                                    }
                                    u_var5 = (u_var5 & 0xffffff80) | u60;
                                    if (b1 & 0x80) == 0 {
                                        pb4 = base.add(5);
                                    } else {
                                        pb4 = base.add(6);
                                        a6c[0] = (base.add(5).read() as u32) << 0x10;
                                    }
                                }
                                (&raw mut DAT_005375d0)
                                    .offset(i_var3 as isize)
                                    .write(u_var5);
                                a6c[0] = (((pb4.read() as u32) << 0x18) | (a6c[0] & 0xffffff))
                                    & 0xfeffffff;
                                let p0 = pb4.read();
                                let p1 = pb4.add(1).read();
                                let u68 = (((((p1 & 1) * 2 | (p0 & 1)) as u32) << 8 | p1 as u32)
                                    & 0xfffe) as u16;
                                a6c.as_mut_ptr()
                                    .cast::<u8>()
                                    .add(4)
                                    .cast::<u16>()
                                    .write_unaligned(u68);
                                let pi8 = FUN_0049b0d0(i_var3, u_var5, a6c.as_mut_ptr().cast());
                                if !pi8.is_null() {
                                    let r = FUN_00431d30();
                                    *pi8 = 1;
                                    if r == 0 {
                                        *pi8 = 0;
                                    }
                                    *pi8.add(1) = 1;
                                }
                            }
                        }
                        2 => {
                            FUN_0049b8a0(pi_var7);
                            FUN_0048f530();
                            DAT_006db5a8 = i_var3;
                            if DAT_007092c0 != 0 {
                                return;
                            }
                        }
                        8 => {
                            DAT_007092c0 = 1;
                            DAT_0052556c = 1;
                            (&raw mut DAT_0071b740)
                                .offset(DAT_0071ecf4.wrapping_mul(0x5bd) as isize)
                                .write(0);
                            return;
                        }
                        0x15 => {
                            i_var3 = *pi_var7.add(1);
                            if -1 < i_var3 && i_var3 < 8 {
                                (&raw mut DAT_0070aaf0).offset(i_var3 as isize).write(1);
                            }
                            let i6 = DAT_0071ecf4;
                            if DAT_0071ecf4 == i_var3 {
                                DAT_007092c0 = 1;
                                let off = DAT_0071ecf4.wrapping_mul(0x5bd);
                                (&raw mut DAT_0071b740).offset(off as isize).write(0);
                                (&raw mut DAT_0070aaf0).offset(i6 as isize).write(1);
                                tail(off);
                                return;
                            }
                        }
                        0x16 => {
                            i_var3 = FUN_00432c10(i_stack_64);
                            if (&raw mut DAT_0071b740)
                                .offset(i_var3.wrapping_mul(0x5bd) as isize)
                                .read()
                                == 0
                            {
                                DAT_005373d0 = 0x15;
                                DAT_005373d4 = i_var3 as u32;
                                FUN_00431110(i_stack_64, &raw mut DAT_005373d0, 8);
                            } else {
                                if 0 < (&raw mut DAT_00536f48).offset(i_var3 as isize).read() {
                                    (&raw mut DAT_00536f48).offset(i_var3 as isize).write(0);
                                }
                                FUN_0049af60(i_var3);
                                let base = pi_var7.cast::<u8>();
                                let mut pb4 = base.add(5);
                                a6c[0] = 0;
                                a6c[1] = 0;
                                u_var5 = base.add(4).read() as u32;
                                if base.add(4).read() != 0 {
                                    loop {
                                        let u60 = u_var5;
                                        let b1 = pb4.read();
                                        let b2 = b1 & 0x7f;
                                        let pb10: *mut u8;
                                        if b2 == 0x7f {
                                            u_var5 = pb4.add(1).cast::<u32>().read_unaligned();
                                            if (b1 & 0x80) == 0 {
                                                pb10 = pb4.add(5);
                                            } else {
                                                pb10 = pb4.add(6);
                                                a6c[0] = ((pb4.add(5).read() as u32) << 0x10)
                                                    | (a6c[0] & 0xffff);
                                                a6c[0] &= 0xffffff;
                                            }
                                        } else {
                                            u_var5 = (&raw mut DAT_005375d0)
                                                .offset(i_var3 as isize)
                                                .read();
                                            if (b2 as u32) < (u_var5 & 0x7f) {
                                                u_var5 = u_var5.wrapping_add(0x80);
                                            }
                                            u_var5 = (u_var5 & 0xffffff80) | b2 as u32;
                                            if (b1 & 0x80) == 0 {
                                                pb10 = pb4.add(1);
                                            } else {
                                                pb10 = pb4.add(2);
                                                a6c[0] = ((pb4.add(1).read() as u32) << 0x10)
                                                    | (a6c[0] & 0xffff);
                                                a6c[0] &= 0xffffff;
                                            }
                                        }
                                        pb4 = pb10.add(2);
                                        (&raw mut DAT_005375d0)
                                            .offset(i_var3 as isize)
                                            .write(u_var5);
                                        a6c[0] = (((pb10.read() as u32) << 0x18)
                                            | (a6c[0] & 0xffffff))
                                            & 0xfeffffff;
                                        let p0 = pb10.read();
                                        let p1 = pb10.add(1).read();
                                        let u68 = (((((p1 & 1) * 2 | (p0 & 1)) as u32) << 8
                                            | p1 as u32)
                                            & 0xfffe)
                                            as u16;
                                        a6c.as_mut_ptr()
                                            .cast::<u8>()
                                            .add(4)
                                            .cast::<u16>()
                                            .write_unaligned(u68);
                                        let pi8 =
                                            FUN_0049b0d0(i_var3, u_var5, a6c.as_mut_ptr().cast());
                                        if !pi8.is_null() {
                                            let r = FUN_00431d30();
                                            *pi8 = 1;
                                            if r == 0 {
                                                *pi8 = 0;
                                            }
                                            *pi8.add(1) = 1;
                                        }
                                        u_var5 = u60.wrapping_sub(1);
                                        if u60.wrapping_sub(1) == 0 {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        0x17 => {
                            DAT_005250d4 = *pi_var7.add(1);
                        }
                        0x1d => {
                            i_var3 = FUN_00432ca0();
                            if i_var3 == i_stack_64 {
                                DAT_006db5a4 = 0;
                            } else {
                                i_var3 = FUN_00432c10(i_stack_64);
                                if (&raw mut DAT_0071b740)
                                    .offset(i_var3.wrapping_mul(0x5bd) as isize)
                                    .read()
                                    != 0
                                    && 0 < (&raw mut DAT_00536f48).offset(i_var3 as isize).read()
                                {
                                    (&raw mut DAT_00536f48).offset(i_var3 as isize).write(0);
                                }
                            }
                        }
                        0x3d => {
                            DAT_005373d4 = DAT_0071ecf4 as u32;
                            DAT_005373d0 = 0x15;
                            FUN_004310c0(&raw mut DAT_005373d0, 8);
                            let i6 = DAT_0071ecf4;
                            DAT_007092c0 = 1;
                            let off = DAT_0071ecf4.wrapping_mul(0x5bd);
                            (&raw mut DAT_0071b740).offset(off as isize).write(0);
                            (&raw mut DAT_0070aaf0).offset(i6 as isize).write(1);
                            tail(off);
                            return;
                        }
                        0x3e | 0x3f => {
                            if *pi_var7.add(1) == DAT_00525560 {
                                local_54 = 1;
                            }
                        }
                        _ => {}
                    }
                }
                i_var3 = DAT_0071ecf4;
                if local_54 != 0 {
                    break;
                }
            }
            if *pi_var7 == 0x3e {
                break 'outer;
            }
            i_var3 = *pi_var7.add(2);
            let mut i_var6 = *pi_var7.add(3);
            while i_var6 != -1 {
                FUN_0049bf90(pi_var7.add(5), *pi_var7.add(3), *pi_var7.add(4));
                pi_var7 = pi_var7
                    .cast::<u8>()
                    .offset((*pi_var7.add(4)).wrapping_add(8) as isize)
                    .cast::<i32>();
                i_var6 = *pi_var7.add(3);
            }
            u_var5 = 8;
            DAT_005373d0 = 0x10;
            DAT_005373d4 = i_var3 as u32;
        }

        if (&raw mut DAT_0070aaf0).offset(DAT_0071ecf4 as isize).read() != 0 {
            (&raw mut DAT_0070aaf0)
                .offset(DAT_0071ecf4 as isize)
                .write(1);
            DAT_007092c0 = 1;
            let off = i_var3.wrapping_mul(0x5bd);
            (&raw mut DAT_0071b740).offset(off as isize).write(0);
            tail(off);
            return;
        }
        FUN_0048f530();
        let t = (&raw mut DAT_0071bce5)
            .cast::<u8>()
            .offset(DAT_0071ecf4.wrapping_mul(0x5bd) as isize)
            .cast::<i32>()
            .read_unaligned();
        if t != 0 {
            FUN_004a74f0();
        }
        FUN_0049bfc0(*pi_var7.add(2), *pi_var7.add(1));
        DAT_005373d0 = 0x1b;
        u_var5 = 4;
        let puv = &raw mut DAT_005373d0;
        i_var3 = FUN_00432ca0();
        FUN_00431110(i_var3, puv, u_var5);
        DAT_006db5a8 = DAT_00732a18.wrapping_add(DAT_0071f2ec);
        FUN_00438b70();
    }
}
