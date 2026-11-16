// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: *mut u8;
    static mut DAT_006d9d60: u16;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_0070ad85: [u8; 1];
    static mut DAT_0070ad40: [u8; 1];
    static mut DAT_0052b0b4: [u8; 1];
    static mut DAT_0052b0b6: [u8; 1];
    static mut DAT_006d4a94: [u8; 1];
    static mut DAT_006d4a96: [u8; 1];
    static mut DAT_006d4ab7: [u8; 1];
    static mut DAT_006d4ab8: [u8; 1];
    static mut DAT_0070b252: [u8; 1];
    static mut DAT_0070acf0: [u8; 1];
    static mut DAT_00732fe2: u16;
    static mut _DAT_00732952: u16;
    static mut _DAT_00732954: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478320(param_1: u32, param_2: u16) -> u32;
    fn FUN_00446fa0(param_1: u32) -> i16;
    fn FUN_0040c070(param_1: i32, param_2: i32, param_3: i32, param_4: u16) -> u32;
    fn FUN_004a3490(param_1: i32, param_2: i32);
    fn FUN_004ac6f0(param_1: u32, param_2: *mut core::ffi::c_void) -> u32;
    fn FUN_004a1d70(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00454f60() -> u8 {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are read and written at the offsets the original uses.
    unsafe {
        let p = DAT_00732e54;
        if p.byte_add(0x92).cast::<i16>().read_unaligned() == 0 {
            return 0;
        }
        let mut b13: u8 = 0;
        let d60 = DAT_006d9d60 as u32;
        let d50 = DAT_006d9d50;
        let u10 = ((d60.wrapping_mul(0x2b3)) >> 16) as u16;
        let u11 = ((p as u32) >> 16) as u16;
        let mut u_stack4: u16 = 0;
        let sel = (&raw mut DAT_0070ad85)
            .cast::<u8>()
            .add(d60.wrapping_mul(0x566) as usize)
            .read();
        match sel {
            1 => {
                b13 = ((p.byte_add(0x1df).read() & 1) == 0) as u8;
                let c2 = DAT_006d4988
                    .add((d50 as u32).wrapping_mul(0x23).wrapping_add(2) as usize)
                    .cast::<i8>()
                    .read();
                let s20b = p.byte_add(0x20b).cast::<i16>().read_unaligned();
                let s20d = p.byte_add(0x20d).cast::<i16>().read_unaligned();
                if (c2 == 3 || c2 == 4) && (s20b as i32 + s20d as i32) == 0 {
                    b13 = 1;
                }
                u_stack4 = 0x17c;
            }
            2 => {
                b13 = ((p.byte_add(0x1df).read() & 0x10) == 0) as u8;
                u_stack4 = 0x180;
            }
            3 => {
                b13 = 0;
                let mut local6: u16 = 0;
                let b1 = p.byte_add(0x228).read();
                let mut b3: u8 = 0;
                if b1 != 0 {
                    loop {
                        b13 = b3;
                        let mut u7 = local6 as u32;
                        if p.byte_add(u7 as usize + 0x229).read() != 0 {
                            let i6 = (p.byte_add(4).read() as u32)
                                .wrapping_mul(0xef)
                                .wrapping_add(u7) as usize;
                            u7 = (&raw mut DAT_0052b0b4).cast::<u8>().add(i6).read() as u32;
                            let mut u4 = u7 as u16;
                            while u4 as u32
                                <= (&raw mut DAT_0052b0b6).cast::<u8>().add(i6).read() as u32
                            {
                                if p.byte_add((u7.wrapping_mul(4) as usize) + 0x346).read() != 0 {
                                    b13 = 1;
                                }
                                u4 = (u7 as u16 as i16).wrapping_add(1) as u16;
                                u7 = u4 as u32;
                            }
                        }
                        local6 = local6.wrapping_add(1);
                        b3 = b13;
                        if !(local6 < b1 as u16) {
                            break;
                        }
                    }
                }
                b13 = (b13 == 0 || (p.byte_add(0x1df).read() & 8) == 0) as u8;
                u_stack4 = 0x180;
            }
            4 => {
                let c7 = p.byte_add(199).cast::<u16>().read_unaligned();
                let c3 = p.byte_add(0xc3).cast::<u16>().read_unaligned();
                let u7 = FUN_00478320(((u11 as u32) << 16) | c7 as u32, 0x8000);
                b13 = ((u7 as u16) <= c3) as u8;
                u_stack4 = 0x17e;
            }
            5 => {
                let mut u4: u16 = 0;
                loop {
                    if p.byte_add(u4 as usize + 0x1e6).read() != 0 {
                        b13 = 1;
                    }
                    u4 = u4.wrapping_add(1);
                    if !(u4 < 10) {
                        break;
                    }
                }
                u_stack4 = 0x181;
            }
            6 => {
                let s5 = FUN_00446fa0(((u10 as u32) << 16) | d50 as u32);
                let u7 = FUN_00478320(s5 as u16 as u32, 0x8000);
                let s20b = p.byte_add(0x20b).cast::<i16>().read_unaligned();
                let s20d = p.byte_add(0x20d).cast::<i16>().read_unaligned();
                b13 = ((s20b as i32 + s20d as i32) <= (u7 & 0xffff) as i32) as u8;
                u_stack4 = 0x17a;
            }
            7 => {
                let s5 = FUN_00446fa0(((u10 as u32) << 16) | d50 as u32);
                let u7 = FUN_00478320(s5 as u16 as u32, 0x4000);
                let s20b = p.byte_add(0x20b).cast::<i16>().read_unaligned();
                let s20d = p.byte_add(0x20d).cast::<i16>().read_unaligned();
                b13 = ((s20b as i32 + s20d as i32) <= (u7 & 0xffff) as i32) as u8;
                u_stack4 = 0x17b;
            }
            8 => {
                let c7 = p.byte_add(199).cast::<u16>().read_unaligned();
                let c3 = p.byte_add(0xc3).cast::<u16>().read_unaligned();
                let u7 = FUN_00478320(((u11 as u32) << 16) | c7 as u32, 0x4000);
                b13 = ((u7 as u16) <= c3) as u8;
                u_stack4 = 0x17d;
            }
            9 => {
                let c7 = p.byte_add(199).cast::<u16>().read_unaligned();
                let c3 = p.byte_add(0xc3).cast::<u16>().read_unaligned();
                let u7 = FUN_00478320(((u11 as u32) << 16) | c7 as u32, 0xc000);
                b13 = ((u7 as u16) <= c3) as u8;
                u_stack4 = 0x17f;
            }
            _ => {}
        }
        if b13 != 0 {
            if p.byte_add(0x73).read() == 0 {
                let u7 = DAT_006d9d60 as u32;
                let off = u7.wrapping_mul(0x126) as usize;
                let a94 = (&raw mut DAT_006d4a94).cast::<u8>().add(off).cast::<i16>();
                a94.write_unaligned(a94.read_unaligned().wrapping_add(1));
                let p12 = p.byte_add(0xb0);
                let c2 = (&raw mut DAT_0070ad40)
                    .cast::<u8>()
                    .add(u7.wrapping_mul(0x566) as usize)
                    .read() as i8;
                if p12.read() as i8 == c2 {
                    (&raw mut DAT_006d4ab7).cast::<u8>().add(off).write(1);
                }
                let p8 = p.byte_add(0x74);
                if p8.read() != 0 {
                    let a96 = (&raw mut DAT_006d4a96).cast::<u8>().add(off).cast::<i16>();
                    a96.write_unaligned(a96.read_unaligned().wrapping_add(-1));
                    if p12.read() as i8 == c2 {
                        (&raw mut DAT_006d4ab8).cast::<u8>().add(off).write(0);
                    }
                    p8.write(0);
                }
                FUN_0040c070(1, 0xf, DAT_006d9d50 as i32, 0xffff);
                let moff = (DAT_006d9d60 as u32).wrapping_mul(0x566) as usize;
                let mut i252 = (&raw mut DAT_0070b252)
                    .cast::<u8>()
                    .add(moff)
                    .cast::<i32>()
                    .read_unaligned();
                if i252 != -1 {
                    let p1f = DAT_006d4988
                        .add((DAT_006d9d50 as u32).wrapping_mul(0x23).wrapping_add(0x1f) as usize)
                        .cast::<*const u8>()
                        .read_unaligned();
                    DAT_00732fe2 = p1f.add(0x86).read() as u16;
                    let moff = (DAT_006d9d60 as u32).wrapping_mul(0x566) as usize;
                    FUN_004a3490(
                        0,
                        (&raw mut DAT_0070acf0).cast::<u8>().add(moff) as usize as i32,
                    );
                    let u9 = FUN_004ac6f0(
                        DAT_006d9d60 as u32,
                        DAT_00732e54.cast::<core::ffi::c_void>(),
                    );
                    _DAT_00732952 = u9 as u16;
                    _DAT_00732954 = u_stack4;
                    i252 = (&raw mut DAT_0070b252)
                        .cast::<u8>()
                        .add(moff)
                        .cast::<i32>()
                        .read_unaligned();
                    FUN_004a1d70(0x179, i252);
                }
            }
            DAT_00732e54.byte_add(0x73).write(1);
        }
        b13
    }
}
