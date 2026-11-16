// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006559b0: [i16; 8];
    static mut DAT_00655184: u16;
    static mut DAT_00656d6c: [u8; 1];
    static mut DAT_00656d68: [u8; 1];
    static mut DAT_00656d4a: [u8; 1];
    static mut DAT_00656d6a: [u8; 1];
    static mut DAT_00656d66: [u8; 1];
    static mut DAT_00656d64: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00457700();
    fn FUN_0044ff80();
    fn FUN_00436840();
    fn FUN_004825d0() -> u32;
    fn FUN_00459fa0();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004588c0(param_1: i32);
}

#[inline(always)]
unsafe fn put(base: *mut u8, v: i16) {
    // SAFETY: base is the game's own table; the index is read from the game's global.
    unsafe {
        let idx = DAT_00655184 as usize;
        base.byte_add(idx * 2).cast::<i16>().write_unaligned(v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b2880() {
    // SAFETY: all globals and callees are the game's own memory and code.
    unsafe {
        let arr = (&raw mut DAT_006559b0).cast::<i16>();
        let p6c = (&raw mut DAT_00656d6c).cast::<u8>();
        let p68 = (&raw mut DAT_00656d68).cast::<u8>();
        let p4a = (&raw mut DAT_00656d4a).cast::<u8>();
        let p6a = (&raw mut DAT_00656d6a).cast::<u8>();
        let p66 = (&raw mut DAT_00656d66).cast::<u8>();
        let p64 = (&raw mut DAT_00656d64).cast::<u8>();

        let mut i: u32 = 0;
        loop {
            let s = arr.add(i as usize).read();
            if s == -2 {
                match i {
                    0 => {
                        FUN_00457700();
                        put(p6c, -2);
                    }
                    1 => {
                        FUN_0044ff80();
                        put(p68, -2);
                    }
                    2 => {
                        FUN_00436840();
                        put(p4a, -2);
                    }
                    3 => {
                        FUN_004825d0();
                        put(p6a, -2);
                    }
                    4 => {
                        FUN_004588c0(1);
                        put(p66, s);
                    }
                    5 => {
                        FUN_004588c0(0);
                        put(p64, -2);
                    }
                    6 => {
                        FUN_00459fa0();
                        put(p66, s);
                    }
                    _ => {}
                }
                arr.add(i as usize).write(0);
            }
            i = (i as u16).wrapping_add(1) as u32;
            if i >= 8 {
                break;
            }
        }

        let mut j: u16 = 0;
        loop {
            let s = arr.add(j as usize).read();
            if s != 0 {
                match j {
                    0 => {
                        FUN_00457700();
                        put(p6c, s);
                    }
                    1 => {
                        FUN_0044ff80();
                        put(p68, s);
                    }
                    2 => {
                        FUN_00436840();
                        put(p4a, s);
                    }
                    3 => {
                        let r = FUN_004825d0();
                        if (r as i16) != 0 {
                            put(p6a, s);
                        }
                    }
                    4 => {
                        FUN_004588c0(1);
                        put(p66, s);
                    }
                    5 => {
                        FUN_004588c0(0);
                        put(p64, s);
                    }
                    6 => {
                        FUN_00459fa0();
                        put(p66, s);
                    }
                    _ => {}
                }
            }
            j = j.wrapping_add(1);
            if j > 7 {
                break;
            }
        }
    }
}
