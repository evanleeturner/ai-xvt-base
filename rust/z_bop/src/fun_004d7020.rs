// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665438: i32;
    static mut DAT_00aa6cf4: i32;
    static mut DAT_00bb2810: i32;
    static mut DAT_0052bf50: i32;
    static mut DAT_00b69ccc: u16;
    static mut DAT_00b6a2c0: u16;
    static mut DAT_00b69e38: u16;
    static mut DAT_00b6a2d8: u16;
    static mut DAT_00b6a2a0: i32;
    static mut DAT_00bb73a7: i8;
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0052b460: [i8; 1];
    static mut DAT_0052bf70: [i8; 1];
    static mut DAT_0052bf64: [i8; 1];
    static mut DAT_0052bf58: [i8; 1];
    static mut DAT_0052ad74: [i8; 1];
    static mut DAT_00aa62e8: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004d6460(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004db7d0(a: i32, s: *mut i8, r: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004d9d70(a: *mut i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32;
    fn FUN_004d67e0(a: *mut i32, b: i32, c: i32, d: i16, e: i32);
    fn FUN_004b5570(s: *mut i8, a: i32, b: i32);
    fn FUN_004d6480(a: *mut i32, b: *mut i32);
    fn FUN_004c9bf0(a: u32);
    fn FUN_004d6b90(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004d6950(a: *mut i32, b: i32, c: i32, d: i16);
    fn FUN_004dc220() -> u8;
    fn FUN_004dc240() -> u8;
    fn FUN_004de5a0(s: *mut i8, a: i32, b: i32, c: i32);
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d7020(param_1: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own; the entry table
    // pointer is walked in bytes exactly as the original does.
    unsafe {
        let mut local_30: [i32; 4] = [0; 4];
        let mut local_14: [i32; 5] = [0; 5];
        let mut local_18: i32 = 0;
        let p30 = local_30.as_mut_ptr();
        let p14 = local_14.as_mut_ptr();

        if param_1 == 0 {
            DAT_00665438 = 0;
        }
        let local_1c = GetTickCount();
        FUN_004d6460(p30, 0x58, 0x5e, 0x1a0, 0x6d);
        let pc = FUN_004ddc10(0x10);
        FUN_004db7d0(0xc, pc, p30, 0, 1, 0xffff);
        FUN_004de090(p14, &raw mut local_18);
        if 6 < DAT_00aa6cf4 {
            FUN_004d6460(p30, 0x1a5, 0x72, 0x1ae, 0xcc);
            DAT_00665438 = FUN_004d9d70(p30, DAT_00665438, DAT_00aa6cf4, 0, 5, DAT_00bb2810, 5);
        }
        FUN_004d6460(p30, 0x58, 0x72, 0x1a3, 0x80);
        let mut local_20: i32 = -1;
        if DAT_00665438 < (DAT_00665438 as u32).wrapping_add(6) as i32 {
            let base = (&raw mut DAT_00aa62e8).cast::<u8>();
            let mut p5 = base.byte_add((DAT_00665438 as u32).wrapping_mul(0x3e) as usize);
            let mut ivar7 = DAT_00665438;
            loop {
                if DAT_00aa6cf4 <= ivar7 {
                    return local_20;
                }
                if ivar7 == DAT_0052bf50 {
                    FUN_004d67e0(p30, 0, 0, DAT_00bb2810 as i16, 1);
                }
                let mut u_var4: u16 = DAT_00b69ccc;
                let v_type = p5.cast::<i32>().read_unaligned();
                let mut v_m8 = p5.byte_sub(8).cast::<i32>().read_unaligned();
                if v_type == 0x65 {
                    if v_m8 == 0 {
                        u_var4 = DAT_00b6a2c0;
                        if p5.byte_add(5).read() != 0 {
                            u_var4 = DAT_00b69e38;
                        }
                    } else {
                        u_var4 = 0xffff;
                        if (v_m8 as u32) < 9 {
                            u_var4 = DAT_00b6a2d8;
                        }
                    }
                }
                if p5.byte_add(4).read() != 0 {
                    FUN_004b5570(
                        (&raw mut DAT_0052bf70).cast::<i8>(),
                        (local_30[0] as u32).wrapping_sub(6) as i32,
                        (local_30[1] as u32).wrapping_add(1) as i32,
                    );
                }
                let ivar8 = p5.cast::<i32>().read_unaligned();
                let name = p5.byte_sub(0x38).cast::<i8>();
                let pc6: *mut i8;
                if ivar8 == 0x65 {
                    pc6 = name;
                } else {
                    if ivar8 == 0 {
                        _sprintf(
                            (&raw mut DAT_00b69d20).cast::<i8>(),
                            (&raw const DAT_0052bf64).cast::<i8>(),
                            name,
                        );
                    } else {
                        let t = (ivar8 as u32).wrapping_sub(1);
                        _sprintf(
                            (&raw mut DAT_00b69d20).cast::<i8>(),
                            (&raw const DAT_0052bf58).cast::<i8>(),
                            name,
                            t / 100 + 1,
                            t % 100,
                        );
                    }
                    pc6 = (&raw mut DAT_00b69d20).cast::<i8>();
                }
                FUN_004db7d0(0xc, pc6, p30, 0, 1, u_var4 as u32);
                v_m8 = p5.byte_sub(8).cast::<i32>().read_unaligned();
                if (v_m8 as u32) < 9 {
                    FUN_004d6480(p14.add(1), p30);
                    local_14[1] = 0x11d;
                    _sprintf(
                        (&raw mut DAT_00b69d20).cast::<i8>(),
                        (&raw const DAT_0052b460).cast::<i8>(),
                        v_m8,
                    );
                    FUN_004db7d0(
                        0xc,
                        (&raw mut DAT_00b69d20).cast::<i8>(),
                        p14.add(1),
                        0,
                        1,
                        u_var4 as u32,
                    );
                    local_14[1] = 0x176;
                    let v_m4 = p5.byte_sub(4).cast::<i32>().read_unaligned();
                    FUN_004c9bf0(local_1c.wrapping_sub(v_m4 as u32) / 1000);
                    FUN_004db7d0(
                        0xc,
                        (&raw mut DAT_00b69d20).cast::<i8>(),
                        p14.add(1),
                        0,
                        1,
                        u_var4 as u32,
                    );
                }
                let i3 = FUN_004d6b90(p30, local_14[0], local_18);
                let mut i8v = local_20;
                if i3 != 0 {
                    FUN_004d6950(p30, 0, 0, DAT_00b6a2a0 as i16);
                    if FUN_004dc220() != 0 || FUN_004dc240() != 0 {
                        i8v = ivar7;
                        if DAT_00bb73a7 != 0 {
                            FUN_004de5a0((&raw mut DAT_0052ad74).cast::<i8>(), 1, 0, 0xff);
                        }
                    }
                }
                local_20 = i8v;
                p5 = p5.byte_add(0x3e);
                ivar7 = ivar7.wrapping_add(1);
                FUN_004d64a0(p30, 0, 0xf);
                if !(ivar7 < (DAT_00665438 as u32).wrapping_add(6) as i32) {
                    break;
                }
            }
        }
        local_20
    }
}
