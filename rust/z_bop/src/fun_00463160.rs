// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009fd394: i32;
    static mut DAT_005233ec: i32;
    static mut DAT_00b6bc32: [u8; 4];
    static mut DAT_00bb73a9: [u8; 0x118];
    static mut DAT_009d7800: [i32; 1];
    static mut DAT_009e9bf6: [u8; 1];
    static mut DAT_009e9678: [u8; 1];
    static mut DAT_00557360: u32;
    static mut DAT_00557364: u32;
    static mut DAT_00557368: [u32; 0x46];
    static mut DAT_00a60a28: *mut c_void;
    static mut DAT_00a60a2c: *mut i8;
    static mut DAT_00a60a30: *mut i8;
    static mut DAT_00a60a34: *mut i8;
    static mut DAT_00a08138: i32;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046f700() -> i32;
    fn FUN_0046e040() -> i32;
    fn FUN_0046fa30() -> i32;
    fn FUN_00448da0();
    fn FUN_00448f90(param_1: i32, param_2: *mut c_void, param_3: i32);
    fn FUN_00448ed0();
    fn FUN_0046f5e0(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> *mut i32;
    fn FUN_0046f660(param_1: i32) -> i32;
    fn FUN_0046f720(param_1: *mut c_void) -> *mut i8;
    fn FUN_0046d300(param_1: *mut u32, param_2: i32);
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

unsafe fn copy_cstr(mut dst: *mut u8, mut src: *const u8) {
    // SAFETY: the pointers refer to the game's NUL-terminated strings and
    // the caller's buffer, copied byte by byte as the original does.
    unsafe {
        loop {
            let b = src.read();
            dst.write(b);
            dst = dst.add(1);
            src = src.add(1);
            if b == 0 {
                break;
            }
        }
    }
}

unsafe fn chat(buf: *mut u8, id: i32, flag: *mut i32, b3: *mut i32) {
    // SAFETY: callees and globals are the game's own; buf is the caller's
    // 256-byte buffer; flag and b3 point at the caller's locals.
    unsafe {
        let p = FUN_0046f720(FUN_0046f660(id) as *mut c_void);
        if p.is_null() {
            copy_cstr(buf, DAT_00a60a2c as *const u8);
        } else {
            copy_cstr(buf, p as *const u8);
            let nv = (flag.read() == 0) as i32;
            b3.write(nv);
            let src2 = if nv != 0 { DAT_00a60a30 } else { DAT_00a60a34 };
            flag.write(nv);
            let mut end = buf;
            while end.read() != 0 {
                end = end.add(1);
            }
            copy_cstr(end, src2 as *const u8);
        }
        FUN_00448f90(3, buf as *mut c_void, 0x30);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00463160() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used with the strides and offsets of the original.
    unsafe {
        let mut b4: i32 = 1;
        let mut b3: i32 = 1;
        let mut b5: i32 = 1;
        let mut local_110: u32 = 0;
        let mut local_114: i32 = 0;
        let mut local_108: i32 = 0;
        let mut local_100 = [0u8; 256];
        let buf = local_100.as_mut_ptr();
        let pb3 = &raw mut b3;
        let pb4 = &raw mut b4;
        let pb5 = &raw mut b5;

        let bf6 = (&raw mut DAT_009e9bf6).cast::<u8>();
        let b678 = (&raw mut DAT_009e9678).cast::<u8>();
        let src_blk = (&raw mut DAT_00bb73a9).cast::<u8>();
        let d7800 = (&raw mut DAT_009d7800).cast::<i32>();
        let a360 = &raw mut DAT_00557360;
        let a368 = (&raw mut DAT_00557368).cast::<u32>();
        let bc32 = (&raw mut DAT_00b6bc32).cast::<u8>();

        let iv7 = FUN_0046f700();
        if DAT_009fd394 < 2 {
            let u6 = bc32.cast::<u32>().read_unaligned();
            bf6.cast::<u16>().write_unaligned(DAT_005233ec as u16);
            b678.cast::<u16>().write_unaligned(u6 as u16);
            for k in 0..0x46usize {
                let v = src_blk.add(k * 4).cast::<u32>().read_unaligned();
                d7800.add(k).cast::<u32>().write_unaligned(v);
            }
        } else {
            let iv8 = FUN_0046e040();
            if iv8 == 0 {
                DAT_00557364 = DAT_005233ec as u32;
                a368.write(bc32.cast::<u32>().read_unaligned());
                DAT_00557360 = 0x11;
                FUN_0046d350(iv7, a360, 0xc);
                FUN_00448da0();
                FUN_00448f90(1, DAT_00a60a28, 0x30);
                let mut p;
                loop {
                    p = FUN_0046f5e0(&raw mut local_114, &raw mut local_108, 0x3c);
                    if p.is_null() {
                        return 0;
                    }
                    if p.read() == 0x1d {
                        let t = timeGetTime();
                        if 200 < t.wrapping_sub(local_110) as i32 {
                            local_110 = t;
                            chat(buf, local_114, pb4, pb3);
                        }
                    }
                    if p.read() == 0x12 {
                        break;
                    }
                }
                FUN_00448ed0();
                DAT_00a08138 = p.add(1).read();
                if 0 < DAT_009fd394 {
                    let mut n = DAT_009fd394;
                    let mut pb = bf6;
                    loop {
                        pb.cast::<u16>().write_unaligned(p.add(2).read() as u16);
                        n = n.wrapping_sub(1);
                        pb.wrapping_sub(0x57e)
                            .cast::<u16>()
                            .write_unaligned(p.add(3).read() as u16);
                        pb = pb.wrapping_add(0x5bd);
                        p = p.add(2);
                        if n == 0 {
                            break;
                        }
                    }
                }
            } else {
                if 0 < DAT_009fd394 {
                    let mut pb = bf6;
                    let mut n = DAT_009fd394;
                    loop {
                        pb.cast::<u16>().write_unaligned(0x13);
                        pb.wrapping_sub(0x57e).cast::<u16>().write_unaligned(0);
                        n = n.wrapping_sub(1);
                        pb = pb.wrapping_add(0x5bd);
                        if n == 0 {
                            break;
                        }
                    }
                }
                FUN_0046fa30();
                let u6 = bc32.cast::<u32>().read_unaligned();
                let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
                bf6.wrapping_offset(off)
                    .cast::<u16>()
                    .write_unaligned(DAT_005233ec as u16);
                b678.wrapping_offset(off)
                    .cast::<u16>()
                    .write_unaligned(u6 as u16);
                FUN_00448da0();
                FUN_00448f90(1, DAT_00a60a28, 0x30);
                let mut local_10c: i32 = 0;
                let mut local_104: u32 = timeGetTime();
                loop {
                    let n = FUN_0046fa30();
                    if !(local_10c < n.wrapping_sub(1)) {
                        break;
                    }
                    let p = FUN_0046f5e0(&raw mut local_114, &raw mut local_108, 0x3c);
                    let mut t = timeGetTime();
                    if p.is_null() {
                        if 60000 < t.wrapping_sub(local_104) {
                            return 0;
                        }
                    } else {
                        local_104 = t;
                        if p.read() == 0x1d {
                            t = timeGetTime();
                            if 200 < t.wrapping_sub(local_110) as i32 {
                                local_110 = t;
                                chat(buf, local_114, pb5, pb3);
                            }
                        }
                        if p.read() == 0x11 {
                            let r = FUN_0046f660(local_114);
                            let off = (r as u32).wrapping_mul(0x5bd) as i32 as isize;
                            bf6.wrapping_offset(off)
                                .cast::<u16>()
                                .write_unaligned(p.add(1).read() as u16);
                            b678.wrapping_offset(off)
                                .cast::<u16>()
                                .write_unaligned(p.add(2).read() as u16);
                            local_10c = local_10c.wrapping_add(1);
                        }
                    }
                }
                FUN_00448ed0();
                DAT_00557364 = DAT_00a08138 as u32;
                DAT_00557360 = 0x12;
                if 0 < DAT_009fd394 {
                    let mut n = DAT_009fd394;
                    let mut pu = a368;
                    let mut pb = bf6;
                    loop {
                        pu.write(pb.cast::<u16>().read_unaligned() as u32);
                        n = n.wrapping_sub(1);
                        pu.add(1)
                            .write(pb.wrapping_sub(0x57e).cast::<u16>().read_unaligned() as u32);
                        pu = pu.add(2);
                        pb = pb.wrapping_add(0x5bd);
                        if n == 0 {
                            break;
                        }
                    }
                }
                FUN_0046d300(
                    a360,
                    (DAT_009fd394 as u32).wrapping_mul(8).wrapping_add(8) as i32,
                );
                loop {
                    let p = FUN_0046f5e0(&raw mut local_114, &raw mut local_108, 0x3c);
                    if p.is_null() {
                        return 0;
                    }
                    if p.read() == 0x12 {
                        break;
                    }
                }
            }
            DAT_00557364 = DAT_009ecc34 as u32;
            DAT_00557360 = 0x20;
            for k in 0..0x46usize {
                let v = src_blk.add(k * 4).cast::<u32>().read_unaligned();
                a368.add(k).write(v);
            }
            let mut iv7: i32 = 0;
            FUN_0046d350(0, a360, 0x120);
            FUN_0046fa30();
            loop {
                let n = FUN_0046fa30();
                if !(iv7 < n) {
                    break;
                }
                let p = FUN_0046f5e0(&raw mut local_114, &raw mut local_108, 0x1e);
                if p.is_null() {
                    break;
                }
                if p.read() == 0x1d {
                    let t = timeGetTime();
                    if 200 < t.wrapping_sub(local_110) as i32 {
                        local_110 = t;
                        chat(buf, local_114, pb3, pb3);
                    }
                } else if p.read() == 0x20 {
                    iv7 = iv7.wrapping_add(1);
                    let src = p.add(2);
                    let idx = (p.add(1).read() as u32).wrapping_mul(0x46) as i32 as isize;
                    let dst = d7800.wrapping_offset(idx);
                    for k in 0..0x46usize {
                        dst.add(k).write(src.add(k).read());
                    }
                }
            }
            FUN_00448ed0();
        }
        1
    }
}
