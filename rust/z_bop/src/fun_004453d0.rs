// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523600: [u8; 256];
    static mut DAT_00523610: [u8; 256];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004438c0() -> *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_00443a50(param_1: i32);
    fn FUN_004441f0(param_1: i32);
    fn FUN_00443c30(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004453d0(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
) {
    // SAFETY: all pointers are the game's own memory; reads and writes follow
    // the original order and offsets.
    unsafe {
        let p3 = FUN_004438c0();
        if p3.is_null() {
            return;
        }
        p3.add(0x40a).write(0);
        p3.add(0x40c).write(param_3);
        p3.add(0x40d).write(param_4 as u32);
        let local_108e: u32 = p3.add(0x40c).read().wrapping_mul(param_4 as u32);
        p3.byte_add(0x108e)
            .cast::<u32>()
            .write_unaligned(local_108e);

        let size = p3
            .add(0x40c)
            .read()
            .wrapping_mul(param_4 as u32)
            .wrapping_mul(6);
        let mut p4 = _malloc(size) as *mut u8;
        let mut p9 = (param_1 as u32).wrapping_add(0x10) as usize as *const u8;
        let mut i10: i32 = 0;
        p3.byte_add(0x1086).cast::<*mut u8>().write_unaligned(p4);

        let mut b6: u8 = p9.read();
        while b6 != 0xff {
            while b6 != 0xfe {
                let mut local_4: u8 = p9.read();
                if local_4 == 0xfb {
                    let b2 = p9.add(2).read();
                    let b1 = p9.add(1).read();
                    p9 = p9.add(3);
                    i10 = (b2 as i32).wrapping_mul(0x100).wrapping_add(b1 as i32);
                } else if local_4 == 0xfc {
                    let b = p9.add(1).read();
                    p9 = p9.add(2);
                    let mut p5 = p4;
                    let mut c = b;
                    loop {
                        p5.write(0x80);
                        p4 = p5.add(6);
                        p5.add(1).write(0x80);
                        p5.add(2).write(0x80);
                        p5.add(4).cast::<u16>().write_unaligned(0);
                        p5.add(3).write(0);
                        let nz = c != 0;
                        p5 = p4;
                        c = c.wrapping_sub(1);
                        if !nz {
                            break;
                        }
                    }
                } else {
                    let u7: u32;
                    if local_4 == 0xfd {
                        local_4 = p9.add(1).read();
                        u7 = p9.add(2).read() as u32;
                        p9 = p9.add(3);
                    } else {
                        p9 = p9.add(1);
                        let sh = ((&raw const DAT_00523610)
                            .cast::<u8>()
                            .offset(param_5 as isize)
                            .read()
                            & 0x1f) as u32;
                        u7 = ((local_4 as u32) >> sh).wrapping_add(i10 as u32);
                        local_4 = (&raw const DAT_00523600)
                            .cast::<u8>()
                            .offset(param_5 as isize)
                            .read()
                            & local_4;
                    }
                    let i8 = u7.wrapping_mul(4);
                    let base = param_2 as u32;
                    let mut p5 = p4;
                    loop {
                        p5.write((base.wrapping_add(i8) as usize as *const u8).read());
                        p4 = p5.add(6);
                        p5.add(1).write(
                            (base.wrapping_add(1).wrapping_add(i8) as usize as *const u8).read(),
                        );
                        p5.add(2).write(
                            (base.wrapping_add(2).wrapping_add(i8) as usize as *const u8).read(),
                        );
                        p5.add(4).cast::<u16>().write_unaligned(0);
                        p5.add(3).write(0);
                        let nz = local_4 != 0;
                        p5 = p4;
                        local_4 = local_4.wrapping_sub(1);
                        if !nz {
                            break;
                        }
                    }
                }
                b6 = p9.read();
            }
            p9 = p9.add(1);
        }
        FUN_00443a50(p3 as usize as i32);
        FUN_004441f0(p3 as usize as i32);
        FUN_00443c30(p3 as *mut core::ffi::c_void);
    }
}
