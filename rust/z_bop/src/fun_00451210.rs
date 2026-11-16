// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8b80: [i16; 0];
    static mut DAT_009d8b86: i8;
    static mut DAT_009d8b88: i8;
    static mut DAT_009d8b8a: i8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a6ff0: i16;
    static mut DAT_00556aa4: i32;
    static mut DAT_00a08ba4: i16;
    static mut DAT_009d77f8: i8;
    static mut DAT_005233c8: u8;
    static mut DAT_009a806c: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_005235dc: i32;
    static mut DAT_009d1148: i32;
    static mut DAT_009a7ec0: i32;
    static mut DAT_009a7b44: i32;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_009a6ff9: u8;
    static mut DAT_005569f8: i16;
    static mut DAT_009a7390: i32;
    static mut DAT_009d12f8: i32;
    static mut DAT_009a8d98: i32;
    static mut DAT_009a8d94: i32;
    static mut DAT_00556a50: i16;
    static mut DAT_009ec470: i32;
    static mut DAT_009ecc38: i32;
    static mut DAT_009d12f4: i32;
    static mut DAT_009d6934: i32;
    static mut DAT_009e968a: i8;
    static mut DAT_009e9a77: i8;
    static mut DAT_009d7694: u32;
    static mut DAT_009e9680: i8;
    static mut DAT_009ec468: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00450c30();
    fn FUN_00450c90(a: u16);
    fn FUN_004a95c0(a: u32);
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_00451f50(a: u32, b: i32, c: i32, d: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00451210() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed from their addresses and offsets as the original does.
    unsafe {
        let idx8 = (DAT_009ecc34 as u32).wrapping_mul(8) as i32;
        let w = (&raw mut DAT_009d8b80 as *mut i16)
            .offset(idx8 as isize)
            .read_unaligned();
        if ((w == 0 && DAT_009a6ff0 != -1) || DAT_00556aa4 != 0) && DAT_00a08ba4 == 0 {
            if DAT_009d77f8 == 0 {
                FUN_004a95c0(DAT_005233c8 as u32);
                FUN_0040e0c0(
                    DAT_009a806c,
                    DAT_009a7bac,
                    DAT_009a8c14,
                    (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
                );
                FUN_004a9520(
                    DAT_005235dc as i16,
                    DAT_009d1148 as i16,
                    DAT_009a7ec0 as i16,
                    DAT_009a7b44 as i16,
                );
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
                f();
                FUN_0040e0c0(0, 0x140, 200, 0);
                DAT_009a6ff0 = -1;
            } else {
                FUN_00450c30();
                FUN_00450c90(DAT_009a6ff9 as u16);
            }
        }

        let off16 = (DAT_009ecc34 as u32).wrapping_mul(0x10) as i32 as isize;
        let v = (&raw mut DAT_009d8b86)
            .byte_add(off16 as usize)
            .cast::<i16>()
            .read_unaligned();
        if (v == 0 && DAT_005569f8 != -1) || DAT_00556aa4 != 0 {
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_0040e0c0(
                DAT_009a806c,
                DAT_009a7bac,
                DAT_009a8c14,
                (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
            );
            FUN_004a9520(
                DAT_009a7390 as i16,
                DAT_009d12f8 as i16,
                DAT_009a8d98 as i16,
                DAT_009a8d94 as i16,
            );
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            FUN_0040e0c0(0, 0x140, 200, 0);
            DAT_005569f8 = -1;
        }

        let off16 = (DAT_009ecc34 as u32).wrapping_mul(0x10) as i32 as isize;
        let v = (&raw mut DAT_009d8b88)
            .byte_add(off16 as usize)
            .cast::<i16>()
            .read_unaligned();
        if (v == 0 && DAT_00556a50 != -1) || DAT_00556aa4 != 0 {
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_0040e0c0(
                DAT_009a806c,
                DAT_009a7bac,
                DAT_009a8c14,
                (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
            );
            FUN_004a9520(
                DAT_009ec470 as i16,
                DAT_009ecc38 as i16,
                DAT_009d12f4 as i16,
                DAT_009d6934 as i16,
            );
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            FUN_0040e0c0(0, 0x140, 200, 0);
            DAT_00556a50 = -1;
        }

        let off16 = (DAT_009ecc34 as u32).wrapping_mul(0x10) as i32 as isize;
        let p = (&raw mut DAT_009d8b8a)
            .byte_add(off16 as usize)
            .cast::<i16>();
        if p.read_unaligned() == 0 {
            let i5 = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let s_ptr = (&raw mut DAT_009e968a).byte_add(i5 as usize).cast::<i16>();
            let s = s_ptr.read_unaligned();
            if s != -1 {
                let u = FUN_00451f50(s as u16 as u32, DAT_009ecc34, 0, 0);
                if u != DAT_009d7694 {
                    let s = s_ptr.read_unaligned();
                    let u = FUN_00451f50(s as u16 as u32, DAT_009ecc34, 0, 1);
                    if u != 0 {
                        let c = (&raw mut DAT_009e9a77).byte_add(i5 as usize).read();
                        if c == 0x13 || c == 0 || c == 0x14 {
                            let s = s_ptr.read_unaligned();
                            DAT_009d7694 = FUN_00451f50(s as u16 as u32, DAT_009ecc34, 1, 0);
                            let off16 = (DAT_009ecc34 as u32).wrapping_mul(0x10) as i32 as isize;
                            (&raw mut DAT_009d8b8a)
                                .byte_add(off16 as usize)
                                .cast::<i16>()
                                .write_unaligned(0x49c);
                        }
                    }
                }
            }
        }

        let mut pc: *mut i8 = &raw mut DAT_009e9680;
        loop {
            let i_tmp = pc.byte_add(0x59d).cast::<i32>().read_unaligned();
            if pc.read() != 0 && i_tmp == 0 {
                pc.byte_add(0x36).write(0);
            }
            pc = pc.byte_add(0x5bd);
            if !(pc < &raw mut DAT_009ec468) {
                break;
            }
        }
        if DAT_00556aa4 != 0 {
            DAT_00556aa4 = 0;
        }
    }
}
