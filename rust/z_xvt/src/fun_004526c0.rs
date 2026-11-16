// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525e14: i32;
    static mut DAT_00525e10: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bb1b: [u8; 0];
    static mut DAT_00709ffc: i32;
    static mut DAT_005254e8: u32;
    static mut DAT_005254f0: u32;
    static mut DAT_005254f8: i32;
    static mut DAT_005254fc: i32;
    static mut DAT_00525504: i32;
    static mut DAT_00525500: i32;
    static mut DAT_005254f4: i32;
    static mut DAT_005254ec: i32;
    static mut DAT_00525518: i32;
    static mut DAT_00525e0c: *mut i32;
    static mut DAT_006da850: u16;
    static mut DAT_00525e28: [i8; 0];
    static mut DAT_0052356c: [i8; 0];
    static mut DAT_00523570: [i8; 0];
    static mut DAT_00732948: *mut c_void;
    static mut DAT_005f0928: [i8; 0];
    static mut DAT_00525e20: f64;
    static mut DAT_005f0e18: f64;
    static mut DAT_00520148: f64;
    static mut DAT_0071e536: i32;
    static mut DAT_005f0e50: [u32; 0];
    static mut DAT_005f09d8: [u32; 0];
    static mut DAT_005f0e23: u8;
    static mut DAT_005f0e24: i32;
    static mut DAT_005f0e32: u16;
    static mut DAT_005f0e28: i32;
    static mut DAT_005f0e30: u16;
    static mut DAT_005f0e2c: i32;
    static mut DAT_005f0e34: u16;
    static mut DAT_00525e08: u16;
    static mut DAT_005f0ef7: [u8; 0];
    static mut DAT_005f0e3f: [u8; 0];
    static mut DAT_00703c6c: i32;
    static mut DAT_00703c70: i32;
    static mut DAT_00703c84: i32;

    static DAT_00523564: [i8; 0];
    static DAT_00523558: [i8; 0];
    static DAT_00523550: [i8; 0];
    static DAT_00523548: [i8; 0];
    static DAT_00523540: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00465d20();
    fn FUN_0047ed40(p1: *mut i8, p2: *mut i8, p3: i32, p4: i32);
    fn _fscanf(file: *mut c_void, fmt: *const i8, ...) -> i32;
    fn __strnicmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn FUN_0048f4b0(p1: u32);
    fn FUN_00402a80(file: *mut c_void) -> u16;
    fn FUN_00402a70() -> u16;
    fn _fclose(file: *mut c_void) -> i32;
    fn FUN_0048be40(p1: u32) -> u16;
    fn FUN_0048aa90(p1: *mut i8, p2: u32);
    fn FUN_00488490(p1: *mut i8) -> u16;
    fn FUN_0048a570(p1: u32) -> u32;
    fn FUN_0048f4f0(p1: u32) -> *mut i32;
    fn FUN_00485fa0(p1: *mut i32);
    fn FUN_004537e0(p1: i32, p2: i32) -> f64;
    fn FUN_004532d0(p1: *mut u32, p2: i32, p3: u32, p4: u32);
    fn __ftol() -> i64;
}

unsafe fn copy_str(dst: *mut i8, src: *const i8) -> *mut i8 {
    // SAFETY: both pointers address NUL-terminated / large-enough buffers
    // exactly as the original string copy assumes.
    unsafe {
        let mut n: usize = 0;
        loop {
            let c = *src.add(n);
            n += 1;
            if c == 0 {
                break;
            }
        }
        let mut i: usize = 0;
        while i < n {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
        dst.add(n)
    }
}

unsafe fn cat_str(dst: *mut i8, src: *const i8) -> *mut i8 {
    // SAFETY: dst is a NUL-terminated buffer with room for the appended text,
    // as in the original.
    unsafe {
        let mut d = dst;
        while *d != 0 {
            d = d.add(1);
        }
        copy_str(d, src)
    }
}

unsafe fn init_slot() {
    // SAFETY: the globals are the game's own memory; the writes mirror the original.
    unsafe {
        let idx = DAT_0071ecf4 as u32;
        let base = (&raw mut DAT_0071bb1b)
            .cast::<u8>()
            .wrapping_add(idx.wrapping_mul(0x5bd) as usize);
        base.cast::<u32>().write_unaligned(0);
        DAT_00709ffc = 0;
        base.wrapping_add(4)
            .cast::<u32>()
            .write_unaligned(0xfffffb00);
        base.wrapping_add(8).cast::<u32>().write_unaligned(0);
        base.wrapping_add(0x14).cast::<u16>().write_unaligned(0);
        base.wrapping_add(0x10)
            .cast::<u16>()
            .write_unaligned(0x4000);
        base.wrapping_add(0x12).cast::<u16>().write_unaligned(0);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004526c0(param_1: *mut i8) -> u32 {
    let mut ac100: [i8; 256] = [0; 256];
    let mut ac200: [i8; 256] = [0; 256];
    // SAFETY: globals and callees are the game's own; buffers are local and
    // pointers are used as the original uses them.
    unsafe {
        let p100 = ac100.as_mut_ptr();
        let p200 = ac200.as_mut_ptr();

        if DAT_00525e14 != 0 {
            FUN_00465d20();
            DAT_00525e14 = 0;
        }
        let i11_mul5: u32 = (DAT_0071ecf4 as u32).wrapping_mul(5);
        DAT_00525e10 = 0;
        init_slot();
        DAT_005254e8 = 0x3f800000;
        DAT_005254f0 = 0x3f800000;
        DAT_005254f8 = 1;
        DAT_005254fc = 1;
        DAT_00525504 = 1;
        DAT_00525500 = 1;
        DAT_005254f4 = 1;
        DAT_005254ec = 1;
        if DAT_00525e0c.is_null() {
            DAT_006da850 = 0;
        }

        copy_str(p100, param_1);
        let mut p = p100;
        while *p100 != 46 {
            p = p.add(1);
            *p100 = *p;
        }
        *p = 0;
        copy_str(p200, p100);
        cat_str(p200, (&raw mut DAT_00525e28).cast::<i8>());
        FUN_0047ed40(p200, (&raw mut DAT_00523570).cast::<i8>(), 0, 0);

        if DAT_00732948.is_null() {
            copy_str(p200, p100);
            let end13 = cat_str(p200, (&raw mut DAT_0052356c).cast::<i8>());
            FUN_0047ed40(p200, (&raw mut DAT_00523570).cast::<i8>(), 0, 0);
            let file = DAT_00732948;
            if file.is_null() {
                return 0;
            }
            if _fscanf(file, (&raw const DAT_00523564).cast::<i8>(), p200) != 1 {
                return 0;
            }
            if __strnicmp(p200, (&raw const DAT_00523558).cast::<i8>(), 9) != 0 {
                return 0;
            }
            if _fscanf(file, (&raw const DAT_00523550).cast::<i8>(), p200) != 1 {
                return 0;
            }
            if _fscanf(file, (&raw const DAT_00523550).cast::<i8>(), p200) != 1 {
                return 0;
            }
            let u5: u16;
            if __strnicmp(p200, (&raw const DAT_00523548).cast::<i8>(), 5) == 0 {
                if DAT_006da850 != 0 {
                    FUN_0048f4b0(DAT_006da850 as u32);
                }
                u5 = FUN_00402a80(file);
            } else {
                if __strnicmp(p200, (&raw const DAT_00523540).cast::<i8>(), 6) != 0 {
                    _fclose(file);
                    return 0;
                }
                if DAT_006da850 != 0 {
                    FUN_0048f4b0(DAT_006da850 as u32);
                }
                u5 = FUN_00402a70();
            }
            _fclose(file);
            let u6 = FUN_0048be40((((end13 as usize as u32) >> 16) << 16) | (u5 as u32));
            copy_str(p200, p100);
            cat_str(p200, (&raw mut DAT_00525e28).cast::<i8>());
            FUN_0048aa90(p200, ((i11_mul5 >> 16) << 16) | (u6 as u32));
            DAT_006da850 = u6;
        } else {
            _fclose(DAT_00732948);
            if DAT_006da850 != 0 {
                FUN_0048f4b0(DAT_006da850 as u32);
            }
            DAT_006da850 = FUN_00488490(p200);
        }

        let dst = (&raw mut DAT_005f0928).cast::<i8>();
        copy_str(dst, p100);
        cat_str(dst, (&raw mut DAT_00525e28).cast::<i8>());

        let mut u5: u16 = 0;
        if DAT_005254ec != 0 {
            DAT_00525518 = 2;
            let ret = FUN_0048a570((((DAT_005254ec as u32) >> 16) << 16) | (DAT_006da850 as u32));
            DAT_006da850 = ret as u16;
            DAT_00525518 = 2;
            u5 = (ret >> 16) as u16;
        }
        DAT_00525e0c = FUN_0048f4f0(((u5 as u32) << 16) | (DAT_006da850 as u32));
        if (DAT_00525e0c.read() as usize as *mut i32) != DAT_00525e0c {
            FUN_00485fa0(DAT_00525e0c);
        }
        let mut i11: i32 = 0;
        let f15 = FUN_004537e0(DAT_00525e0c as usize as i32, 0);
        let pi4 = DAT_00525e0c;
        DAT_00525e20 = f15;
        let mut i10: i32 = 0;
        let pi1 = (pi4 as usize).wrapping_add(6) as *const i32;
        DAT_005f0e18 = DAT_00520148 / f15;
        let d2: f64 = DAT_00520148 / f15;
        let mut cnt: i32 = pi1.read_unaligned();
        if 0 < cnt {
            loop {
                let bits = d2.to_bits();
                let hi = (bits >> 32) as u32;
                let lo = bits as u32;
                i11 = i11.wrapping_add(4);
                i10 = i10.wrapping_add(1);
                let mut arrp: u32 =
                    ((pi4 as usize).wrapping_add(10) as *const u32).read_unaligned();
                arrp = ((arrp.wrapping_add(i11 as u32).wrapping_sub(4)) as usize as *const u32)
                    .read_unaligned();
                FUN_004532d0(arrp as usize as *mut u32, pi4 as usize as i32, lo, hi);
                cnt = pi1.read_unaligned();
                if !(i10 < cnt) {
                    break;
                }
            }
        }
        DAT_0071e536 = 1;
        if DAT_00525e10 == 0 {
            let mut p14 = (&raw mut DAT_005f0e50).cast::<u32>();
            let mut c: i32 = 0x2b;
            while c != 0 {
                *p14 = 0;
                p14 = p14.add(1);
                c -= 1;
            }
            p14.cast::<u16>().write_unaligned(0);
            p14.cast::<u8>().add(2).write(0);
            let mut p14 = (&raw mut DAT_005f09d8).cast::<u32>();
            let mut c: i32 = 0x10d;
            while c != 0 {
                *p14 = 0;
                p14 = p14.add(1);
                c -= 1;
            }
            p14.cast::<u16>().write_unaligned(0);
            DAT_005f0e23 = 0;
            DAT_005f0e24 = 0;
            DAT_005f0e32 = 0;
            DAT_005f0e28 = 0;
            DAT_005f0e30 = 0;
            DAT_005f0e2c = 0;
            DAT_005f0e34 = 0;
            DAT_00525e08 = 0;
            let a = (&raw mut DAT_005f09d8) as usize as u32;
            (&raw mut DAT_005f0ef7).cast::<u32>().write_unaligned(a);
            let b = (&raw mut DAT_005f0e50) as usize as u32;
            (&raw mut DAT_005f0e3f).cast::<u32>().write_unaligned(b);
            init_slot();
            DAT_005254f8 = 1;
            DAT_005254fc = 1;
            DAT_00525504 = 1;
            DAT_00525500 = 1;
            DAT_005254f4 = 1;
            DAT_005254e8 = 0x3f800000;
            DAT_005254f0 = 0x3f800000;
            let l16 = __ftol();
            DAT_00703c6c = (l16 as i16) as i32;
            let l17 = __ftol();
            DAT_00703c70 = (l17 as i16) as i32;
            DAT_00703c84 = (l16 as i16) as i32;
        }
        1
    }
}
