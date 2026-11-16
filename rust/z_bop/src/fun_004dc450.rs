// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acc131: i32;
    static mut DAT_00acebbd: i32;
    static mut DAT_00aa7b82: i32;
    static mut DAT_00aa7b86: i32;
    static mut DAT_00aa7b8a: i32;
    static mut DAT_00aa7b8e: i32;
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7b0d: u8;
    static mut DAT_00aa7b24: u8;
    static mut DAT_00aa6cfc: i32;

    static mut DAT_00acc575: u8;
    static mut DAT_00acc55d: u8;
    static mut DAT_00acc565: u8;
    static mut DAT_00acc561: u8;
    static mut DAT_00acc569: u8;
    static mut DAT_00acc139: u8;
    static mut DAT_00acc13d: u8;
    static mut DAT_00acc149: u8;
    static mut DAT_00acc145: u8;
    static mut DAT_00acc141: u8;
    static mut DAT_00acc54d: u8;
    static mut DAT_00acc56d: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut u32, param_2: *mut i32);
    fn malloc(size: u32) -> *mut c_void;
    fn FUN_004d4e20();
    fn FUN_004d48e0() -> i32;
    fn FUN_004d4ec0(param_1: i32);
    fn FUN_004d4970();
    fn FUN_004b75b0(param_1: *mut i32);
    fn FUN_004dc9b0();
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d4d20(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dc450(param_1: u32, param_2: *mut i32) -> u32 {
    let mut param_2 = param_2;
    let mut local_10: [i32; 4] = [0; 4];
    let mut local_20: *mut u32 = core::ptr::null_mut();
    // SAFETY: all globals, callees and pointers are the game's own memory,
    // accessed at the addresses and widths the original uses.
    unsafe {
        let i_var2: i32 = (&raw mut DAT_00acc131).read_unaligned();
        if 8 < i_var2 {
            return 0;
        }
        let i_var3: u32 = (i_var2 as u32).wrapping_mul(0x440);
        let off = i_var3 as usize;

        let t = (&raw mut DAT_00acebbd).read_unaligned();
        (&raw mut DAT_00acc575)
            .byte_add(off)
            .cast::<i32>()
            .write_unaligned(t);
        let t = (&raw mut DAT_00aa7b82).read_unaligned();
        (&raw mut DAT_00acc55d)
            .byte_add(off)
            .cast::<i32>()
            .write_unaligned(t);
        let t = (&raw mut DAT_00aa7b86).read_unaligned();
        (&raw mut DAT_00acc565)
            .byte_add(off)
            .cast::<i32>()
            .write_unaligned(t);
        let t = (&raw mut DAT_00aa7b8a).read_unaligned();
        (&raw mut DAT_00acc561)
            .byte_add(off)
            .cast::<i32>()
            .write_unaligned(t);
        let t = (&raw mut DAT_00aa7b8e).read_unaligned();
        (&raw mut DAT_00acc569)
            .byte_add(off)
            .cast::<i32>()
            .write_unaligned(t);

        if param_2.is_null() {
            (&raw mut DAT_00acc139)
                .byte_add(off)
                .cast::<i32>()
                .write_unaligned(0);
            (&raw mut DAT_00acc13d)
                .byte_add(off)
                .cast::<i32>()
                .write_unaligned(0);
            (&raw mut DAT_00acc149)
                .byte_add(off)
                .cast::<i32>()
                .write_unaligned(0);
            (&raw mut DAT_00acc145)
                .byte_add(off)
                .cast::<i32>()
                .write_unaligned(0);
            FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x27f, 0x1df);
            param_2 = local_10.as_mut_ptr();
        } else {
            if param_2.read_unaligned() < 0 {
                param_2.write_unaligned(0);
            }
            if param_2.add(1).read_unaligned() < 0 {
                param_2.add(1).write_unaligned(0);
            }
            if 0x27f < param_2.add(2).read_unaligned() {
                param_2.add(2).write_unaligned(0x27f);
            }
            if 0x1df < param_2.add(3).read_unaligned() {
                param_2.add(3).write_unaligned(0x1df);
            }
            if param_2.add(2).read_unaligned() < param_2.read_unaligned()
                || param_2.add(3).read_unaligned() < param_2.add(1).read_unaligned()
            {
                return 0;
            }
            FUN_004d6480((&raw mut DAT_00acc54d).byte_add(off).cast::<u32>(), param_2);
            let u_var9: u32 = (param_2.add(2).read_unaligned())
                .wrapping_sub(param_2.read_unaligned())
                .wrapping_add(1) as u32;
            let i_var4: i32 = (param_2.add(3).read_unaligned())
                .wrapping_sub(param_2.add(1).read_unaligned())
                .wrapping_add(1);
            let mut local_1c: i32 = i_var4;

            let fmt = (&raw mut DAT_00aa7b1f).read_unaligned();
            if fmt == 8 {
                local_20 = malloc((i_var4 as u32).wrapping_mul(u_var9)).cast::<u32>();
                let c_var1 = DAT_00aa7b0d;
                if local_20.is_null() {
                    return 0;
                }
                if DAT_00aa7b24 == 0 {
                    DAT_00aa6cfc = FUN_004d48e0();
                } else {
                    FUN_004d4e20();
                }
                let stride = (&raw mut DAT_00aa7b13).read_unaligned();
                let mut src: u32 = (param_2.add(1).read_unaligned() as u32)
                    .wrapping_mul(stride as u32)
                    .wrapping_add(param_2.read_unaligned() as u32)
                    .wrapping_add(DAT_00aa6cfc as u32);
                let mut dst: u32 = local_20 as u32;
                if 0 < i_var4 {
                    loop {
                        let mut s = src;
                        let mut d = dst;
                        let mut n = u_var9 >> 2;
                        while n != 0 {
                            (d as *mut u32).write_unaligned((s as *const u32).read_unaligned());
                            s = s.wrapping_add(4);
                            d = d.wrapping_add(4);
                            n -= 1;
                        }
                        let mut n = u_var9 & 3;
                        while n != 0 {
                            (d as *mut u8).write((s as *const u8).read());
                            s = s.wrapping_add(1);
                            d = d.wrapping_add(1);
                            n -= 1;
                        }
                        let stride = (&raw mut DAT_00aa7b13).read_unaligned();
                        src = src.wrapping_add(stride as u32);
                        local_1c = local_1c.wrapping_sub(1);
                        dst = dst.wrapping_add(u_var9);
                        if local_1c == 0 {
                            break;
                        }
                    }
                }
                if DAT_00aa7b24 != 0 {
                    FUN_004d4ec0(1);
                }
                if c_var1 == 0 {
                    FUN_004d4970();
                }
            } else if fmt == 0x10 {
                local_20 =
                    malloc((i_var4 as u32).wrapping_mul(u_var9).wrapping_mul(2)).cast::<u32>();
                let c_var1 = DAT_00aa7b0d;
                if local_20.is_null() {
                    return 0;
                }
                if DAT_00aa7b24 == 0 {
                    DAT_00aa6cfc = FUN_004d48e0();
                } else {
                    FUN_004d4e20();
                }
                let stride = (&raw mut DAT_00aa7b13).read_unaligned();
                let mut src: u32 = (param_2.add(1).read_unaligned() as u32)
                    .wrapping_mul(stride as u32)
                    .wrapping_add((param_2.read_unaligned() as u32).wrapping_mul(2))
                    .wrapping_add(DAT_00aa6cfc as u32);
                let mut local_14: u32 = local_20 as u32;
                if 0 < i_var4 {
                    loop {
                        let mut u_var6 = u_var9;
                        let mut d = local_14;
                        let mut s = src;
                        if 0 < (u_var9 as i32) {
                            loop {
                                (d as *mut u16).write_unaligned((s as *const u16).read_unaligned());
                                u_var6 = u_var6.wrapping_sub(1);
                                d = d.wrapping_add(2);
                                s = s.wrapping_add(2);
                                if u_var6 == 0 {
                                    break;
                                }
                            }
                        }
                        let stride = (&raw mut DAT_00aa7b13).read_unaligned();
                        src = src.wrapping_add((stride as u32) & 0xffff_fffe);
                        local_1c = local_1c.wrapping_sub(1);
                        local_14 = local_14.wrapping_add(u_var9.wrapping_mul(2));
                        if local_1c == 0 {
                            break;
                        }
                    }
                }
                if DAT_00aa7b24 != 0 {
                    FUN_004d4ec0(1);
                }
                if c_var1 == 0 {
                    FUN_004d4970();
                }
            }

            (&raw mut DAT_00acc139)
                .byte_add(off)
                .cast::<u32>()
                .write_unaligned(u_var9);
            (&raw mut DAT_00acc13d)
                .byte_add(off)
                .cast::<i32>()
                .write_unaligned(i_var4);
            (&raw mut DAT_00acc149)
                .byte_add(off)
                .cast::<u32>()
                .write_unaligned(local_20 as u32);

            let fmt = (&raw mut DAT_00aa7b1f).read_unaligned();
            if fmt == 8 {
                let utmp = (i_var4 as u32).wrapping_mul(u_var9);
                (&raw mut DAT_00acc145)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(utmp);
                (&raw mut DAT_00acc141)
                    .byte_add(off)
                    .cast::<i32>()
                    .write_unaligned(0);
                FUN_004b75b0((&raw mut DAT_00acc139).byte_add(off).cast::<i32>());
            } else {
                let fmt = (&raw mut DAT_00aa7b1f).read_unaligned();
                if fmt == 0x10 {
                    let utmp = (i_var4 as u32).wrapping_mul(u_var9).wrapping_mul(2);
                    (&raw mut DAT_00acc145)
                        .byte_add(off)
                        .cast::<u32>()
                        .write_unaligned(utmp);
                    (&raw mut DAT_00acc141)
                        .byte_add(off)
                        .cast::<i32>()
                        .write_unaligned(0);
                }
            }
            if DAT_00aa7b24 != 0 {
                FUN_004dc9b0();
            }
        }

        FUN_004d4d20(param_2);
        let tmp = (&raw mut DAT_00acc131).read_unaligned().wrapping_add(1);
        (&raw mut DAT_00acc131).write_unaligned(tmp);
        (&raw mut DAT_00acc56d)
            .byte_add((tmp as u32).wrapping_mul(0x440) as usize)
            .cast::<u32>()
            .write_unaligned(param_1);
        (&raw mut DAT_00acebbd).write_unaligned(-1);
        1
    }
}
