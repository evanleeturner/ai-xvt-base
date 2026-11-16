// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8e25: [i8; 0];
    static mut DAT_009d8e29: [i8; 0];
    static mut DAT_009d8c88: [i8; 0];
    static mut DAT_009fd458: [i8; 0];
    static mut DAT_009d6954: i8;
    static mut DAT_009e967c: [i8; 0];
    static mut DAT_009e96dd: [i8; 0];
    static mut DAT_009e96e1: [i8; 0];
    static mut DAT_009e967a: [i8; 0];
    static mut DAT_009e97c5: [i8; 0];
    static mut DAT_009e98d3: [i8; 0];
    static mut DAT_009e9889: [i8; 0];
    static mut DAT_009e9765: [i8; 0];
    static mut DAT_009e98c7: [i8; 0];
    static mut DAT_009e98ef: [i8; 0];
    static mut DAT_009e9857: [i8; 0];
    static mut DAT_009e9919: [i8; 0];
    static mut DAT_009e9705: [i8; 0];
    static mut DAT_009e98bb: [i8; 0];
    static mut DAT_009e98df: [i8; 0];
    static mut DAT_009e9825: [i8; 0];
    static mut DAT_009e9909: [i8; 0];
    static mut DAT_009e98ff: [i8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_00a08292: i16;
    static mut DAT_00a07be0: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00435a40(param_1: u32) -> i32;
    fn FUN_00435640(a: i32, b: i32, c: i32, d: i32, e: i32, f: u32) -> i32;
    fn FUN_0042fb70(a: i32, b: u32, c: i32, d: i32, e: u32, f: u32);
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_00430200(a: i32, b: i32, c: u32, d: i32);
}

#[inline(always)]
fn at(base: *mut i8, off: u32) -> *mut u8 {
    (base as usize).wrapping_add(off as usize) as *mut u8
}

#[inline(always)]
unsafe fn inc16(p: *mut u8) {
    // SAFETY: p points into the game's own table.
    unsafe {
        let v = p.cast::<u16>().read_unaligned();
        p.cast::<u16>().write_unaligned(v.wrapping_add(1));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00434bc0(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: u32,
    param_8: i32,
) {
    let _ = (param_7, param_8);
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed and accessed exactly as the original does.
    unsafe {
        let mut u_stack_e: u16 = 0;
        let mut local_8: u16 = 1;
        let p1 = param_1 & 0xffff;
        let a = (p1.wrapping_mul(8).wrapping_sub(p1))
            .wrapping_mul(5)
            .wrapping_add(0x16)
            .wrapping_add(DAT_009a1fe8 as u32);
        let u_var4: u16 = (a as usize as *const u8).read() as u16;
        let u_var3: u32 = u_var4 as u32;
        let mut i_var5: i32 = u_var3.wrapping_mul(0x566) as i32;
        let mut pc6: *mut u8 = at((&raw mut DAT_009d8e25).cast::<i8>(), i_var5 as u32);
        let mut i_var7: i32 = 8;
        let i_var2: u32 = (param_4 as u32).wrapping_mul(0x5bd);
        let idx0 = at((&raw mut DAT_009e967c).cast::<i8>(), i_var2)
            .cast::<u16>()
            .read_unaligned();
        let mut pc8: *mut u8 = at(
            (&raw mut DAT_009d8e29).cast::<i8>(),
            (idx0 as u32).wrapping_add(i_var5 as u32),
        );
        loop {
            if pc6.read() == 0 && pc8.read() == 1 {
                let c = pc6.add(1).read();
                if c == 0 {
                    u_stack_e = 0x6000;
                } else if c == 2 {
                    u_stack_e = 0xf000;
                } else if c == 10 {
                    u_stack_e = 0x6000;
                }
            }
            pc6 = (pc6 as usize).wrapping_add(0x4e) as *mut u8;
            pc8 = (pc8 as usize).wrapping_add(0x4e) as *mut u8;
            i_var7 -= 1;
            if i_var7 != 0 {
                continue;
            }
            let idx = at((&raw mut DAT_009e967c).cast::<i8>(), i_var2)
                .cast::<u16>()
                .read_unaligned();
            let tb = *at((&raw mut DAT_009d8c88).cast::<i8>(), i_var5 as u32);
            let t = *at(
                (&raw mut DAT_009fd458).cast::<i8>(),
                (idx as u32).wrapping_mul(0x1e5).wrapping_add(tb as u32),
            );
            if t == 0 {
                i_var5 = FUN_00435a40(p1);
                let o3 = u_var3.wrapping_mul(2).wrapping_add(i_var2);
                let o6 = (param_6 as u32).wrapping_mul(2).wrapping_add(i_var2);
                let o5 = (param_5 as u32).wrapping_mul(2).wrapping_add(i_var2);
                let o45 = (param_4 as u32)
                    .wrapping_mul(2)
                    .wrapping_add((param_5 as u32).wrapping_mul(0x5bd));
                if param_3 == 1 {
                    inc16(at((&raw mut DAT_009e97c5).cast::<i8>(), o3));
                    if param_5 == -1 {
                        inc16(at((&raw mut DAT_009e98d3).cast::<i8>(), o6));
                    } else {
                        inc16(at((&raw mut DAT_009e9889).cast::<i8>(), o6));
                    }
                    i_var5 /= 10;
                    local_8 = 10;
                } else if param_3 == 2 {
                    inc16(at((&raw mut DAT_009e9765).cast::<i8>(), o3));
                    if param_5 == -1 {
                        inc16(at((&raw mut DAT_009e98c7).cast::<i8>(), o6));
                    } else {
                        inc16(at((&raw mut DAT_009e98ef).cast::<i8>(), o5));
                        inc16(at((&raw mut DAT_009e9857).cast::<i8>(), o6));
                        inc16(at((&raw mut DAT_009e9919).cast::<i8>(), o45));
                    }
                    local_8 = 6;
                    i_var5 /= 2;
                } else if param_3 == 3 {
                    inc16(at((&raw mut DAT_009e9705).cast::<i8>(), o3));
                    if param_5 == -1 {
                        inc16(at((&raw mut DAT_009e98bb).cast::<i8>(), o6));
                    } else {
                        inc16(at((&raw mut DAT_009e98df).cast::<i8>(), o5));
                        inc16(at((&raw mut DAT_009e9825).cast::<i8>(), o6));
                        inc16(at((&raw mut DAT_009e9909).cast::<i8>(), o45));
                    }
                    local_8 = 1;
                }
                if DAT_009d6954 == 2 {
                    i_var5 = i_var5.wrapping_mul(2);
                } else if DAT_009d6954 == 0 {
                    i_var5 /= 2;
                }
                let idx = at((&raw mut DAT_009e967c).cast::<i8>(), i_var2)
                    .cast::<u16>()
                    .read_unaligned();
                let i_var7b = FUN_00435640(
                    2,
                    (((u_stack_e as u32) << 16) | u_var4 as u32) as i32,
                    param_4,
                    local_8 as i32,
                    param_2,
                    idx as u32,
                );
                if -1 < i_var7b {
                    let p = at((&raw mut DAT_009e96dd).cast::<i8>(), i_var2).cast::<i32>();
                    let v = p.read_unaligned();
                    p.write_unaligned(v.wrapping_add(i_var5));
                }
                if param_3 == 3 && DAT_009ecc34 == param_4 {
                    FUN_0042fb70(
                        DAT_009ecc34,
                        0xffffffff,
                        0x15,
                        -1,
                        0xffffffff,
                        u_stack_e as u32,
                    );
                    return;
                }
            } else if param_3 == 2 || param_3 == 3 {
                i_var5 = FUN_00435a40(p1);
                let p = at((&raw mut DAT_009e96dd).cast::<i8>(), i_var2).cast::<i32>();
                let v = p.read_unaligned();
                p.write_unaligned(v.wrapping_sub(i_var5));
                inc16(at((&raw mut DAT_009e98ff).cast::<i8>(), i_var2));
                let p = at((&raw mut DAT_009e96e1).cast::<i8>(), i_var2).cast::<i32>();
                let v = p.read_unaligned();
                p.write_unaligned(v.wrapping_add(-500));
                DAT_00a08292 = at((&raw mut DAT_009e967a).cast::<i8>(), i_var2)
                    .cast::<i16>()
                    .read_unaligned();
                FUN_00450650(0x119, param_4);
                if i_var5 != 0 {
                    DAT_00a07be0 = i_var5 as i16;
                    FUN_00450650(200, param_4);
                }
                if DAT_009ecc34 == param_4 {
                    FUN_00430200(2, 0x39, p1, 0xffff);
                }
            }
            return;
        }
    }
}
