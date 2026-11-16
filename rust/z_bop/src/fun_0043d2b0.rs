// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08374: u16;
    static mut DAT_009a806c: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_009e9697: [u8; 1];
    static mut DAT_009e9698: [u8; 1];
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00a0a216: [u16; 1];
    static mut DAT_00a08de4: [u8; 1];
    static mut DAT_00a08de6: [u8; 1];
    static mut DAT_00a08de8: [u8; 1];
    static mut DAT_005233c8: u8;
    static mut DAT_009ed232: u8;
    static mut DAT_009a20ae: u8;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_009ec464: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32);
    fn FUN_004a9640(a: i8);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a95c0(a: u32);
    fn FUN_004a9590(a: u32);
    fn FUN_004269e0(a: u32) -> u32;
    fn FUN_004277f0(a: u32, b: u32, c: u32);
    fn FUN_00440760(a: u32, b: u32);
}

unsafe fn rd32(a: usize) -> u32 {
    // SAFETY: address inside the game's own data structures.
    unsafe { (a as *const u32).read_unaligned() }
}

unsafe fn cur_slot() -> u32 {
    // SAFETY: the game's own global table.
    unsafe {
        let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
        rd32((&raw mut DAT_009e9670).cast::<u8>().add(off) as usize)
    }
}

unsafe fn cur_pb() -> *const u8 {
    // SAFETY: the game's own global table and records.
    unsafe {
        let slot = cur_slot();
        let base = DAT_009a1fe8 as usize;
        let tmp = rd32(base.wrapping_add(slot.wrapping_mul(0x23).wrapping_add(0x1f) as usize));
        rd32((tmp as usize).wrapping_add(0xa9)) as usize as *const u8
    }
}

unsafe fn type_arg() -> u32 {
    // SAFETY: the game's own global table.
    unsafe {
        let slot = cur_slot();
        let b = DAT_009a1fe8 as usize;
        let v = ((b as u32 >> 8) << 8)
            | ((DAT_009a1fe8.add(slot.wrapping_mul(0x23) as usize + 3)).read() as u32);
        v & 0xffff00ff
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043d2b0(param_1: u32, param_2: u16, param_3: u32) {
    // SAFETY: all globals and callees are the game's own; pointers are
    // read at the offsets the original uses.
    unsafe {
        if DAT_00a08374 != 0 {
            FUN_0040e0c0(
                DAT_009a806c,
                DAT_009a7bac,
                DAT_009a8c14,
                (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
            );
            let i_var4 = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
            let mut pb = cur_pb();
            let u_var10: u16 = if pb.add(0x232).read() == 0 {
                0
            } else {
                pb.add(0x372 + (param_1 & 0xffff) as usize * 4).read() as u16
            };
            let idx = (DAT_00a08374 as u32).wrapping_add(param_2 as u32) as usize;
            (&raw mut DAT_00a0a216)
                .cast::<u16>()
                .add(idx)
                .write(u_var10);
            let c_var8: i8;
            if u_var10 == 0 || {
                pb = cur_pb();
                (pb.add(0x1e5).read() & 8) == 0
            } {
                c_var8 = 0;
            } else if (&raw mut DAT_009e9698).cast::<u8>().add(i_var4).read() == 0 {
                c_var8 = 1;
            } else {
                let g = (&raw mut DAT_009e9697).cast::<u8>().add(i_var4).read();
                if (param_3 & 0xffff) == g as u32 {
                    let b_var2 = pb.add(g as usize + 0x235).read();
                    if (b_var2 & 0x7f) == 3 {
                        c_var8 = 2;
                    } else {
                        c_var8 = (((param_2 & 1) == (b_var2 >> 7) as u16) as i32 + 1) as i8;
                    }
                } else {
                    c_var8 = 1;
                }
            }
            if u_var10 == 0 {
                FUN_004a9640(2);
                let s_var9 = ((DAT_009ed232 as i32 + 1).wrapping_mul(param_2 as i32)) as i16 as i32;
                FUN_004a9520(
                    s_var9.wrapping_add(2) as i16,
                    2,
                    s_var9.wrapping_add(DAT_009ed232 as i32).wrapping_add(3) as i16,
                    (DAT_009a20ae as i32 + 3) as i16,
                );
                FUN_004a95c0(DAT_005233c8 as u32);
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
                f();
            } else {
                FUN_004a9640(2);
                FUN_004a9520(0, 0, DAT_009a7bac as i16, DAT_009a8c14 as i16);
                FUN_004a9500(
                    ((DAT_009ed232 as i32 + 1)
                        .wrapping_mul(param_2 as i32)
                        .wrapping_add(2)) as i16,
                    2,
                );
                FUN_004a95c0(DAT_005233c8 as u32);
                let u_var11: u32 = if c_var8 == 2 { 0x52 } else { 0x4a };
                FUN_004a9590(u_var11);
                DAT_009ec464 = 0;
                let off = idx.wrapping_mul(0xc);
                let us = (&raw mut DAT_00a08de8)
                    .cast::<u8>()
                    .add(off)
                    .cast::<u16>()
                    .read_unaligned();
                if us != 0 {
                    FUN_004a9640(0);
                    let a = type_arg();
                    let u_var11 = FUN_004269e0(a);
                    let u_var6 = FUN_004269e0(0xc);
                    if (u_var11 as i16) == (u_var6 as i16) {
                        FUN_004277f0(u_var10 as u32, 2, 1);
                    } else {
                        FUN_004277f0(u_var10 as u32, 1, 1);
                    }
                }
            }
            FUN_0040e0c0(0, 0x140, 200, 0);
            return;
        }

        let mut pb = cur_pb();
        let mut u_var10: u16 = if pb.add(0x232).read() == 0 {
            0
        } else {
            pb.add(0x372 + (param_1 & 0xffff) as usize * 4).read() as u16
        };
        let u_var11 = param_2 as u32;
        let ent = (&raw mut DAT_00a0a216).cast::<u16>().add(u_var11 as usize);
        if ent.read() != u_var10 {
            ent.write(u_var10);
            FUN_004a9640(2);
            FUN_004a9520(0, 0, DAT_009a7bac as i16, DAT_009a8c14 as i16);
            let off = (DAT_00a08374 as u32)
                .wrapping_add(u_var11)
                .wrapping_mul(0xc) as usize;
            let s1 = (&raw mut DAT_00a08de4)
                .cast::<u8>()
                .add(off)
                .cast::<i16>()
                .read_unaligned();
            let s2 = (&raw mut DAT_00a08de6)
                .cast::<u8>()
                .add(off)
                .cast::<i16>()
                .read_unaligned();
            FUN_004a9500(s1, s2);
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9590(0x4a);
            DAT_009ec464 = 0;
            let a = type_arg();
            let u_var6 = FUN_004269e0(a);
            let u_var7 = FUN_004269e0(0xc);
            let r: u32 = if (u_var6 as i16) == (u_var7 as i16) {
                2
            } else {
                1
            };
            FUN_004277f0(u_var10 as u32, r, 1);
        }
        'lab: {
            if u_var10 != 0 {
                pb = cur_pb();
                if (pb.add(0x1e5).read() & 8) != 0 {
                    let i4 = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
                    if (&raw mut DAT_009e9698).cast::<u8>().add(i4).read() == 0 {
                        u_var10 = 1;
                    } else {
                        let g = (&raw mut DAT_009e9697).cast::<u8>().add(i4).read();
                        if (param_3 & 0xffff) == g as u32 {
                            let b_var2 = pb.add(g as usize + 0x235).read();
                            if (b_var2 & 0x7f) == 3 {
                                u_var10 = 2;
                            } else {
                                u_var10 =
                                    (((param_2 & 1) == (b_var2 >> 7) as u16) as i32 + 1) as u16;
                            }
                        } else {
                            u_var10 = 1;
                        }
                    }
                    break 'lab;
                }
            }
            u_var10 = 0;
        }
        let i_var3 = cur_slot();
        let b_var5: bool;
        if i_var3 == 0xffffffff {
            b_var5 = false;
        } else {
            let c = DAT_009a1fe8
                .add(i_var3.wrapping_mul(0x23) as usize + 3)
                .read() as i8;
            b_var5 = c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4;
        }
        if b_var5 && u_var10 == 2 {
            u_var10 = 4;
        }
        FUN_00440760(u_var11.wrapping_add(0x13), u_var10 as u32);
    }
}
