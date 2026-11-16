// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_0070acf0: [i8; 0];
    static mut DAT_0073b268: [*mut i8; 0];
    static mut DAT_0073b1b4: [*mut i8; 0];
    static mut DAT_0052b070: [i8; 0];
    static mut DAT_0052b07a: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac6f0(a: u32, b: *mut u32) -> u32;
}

unsafe fn str_end(mut p: *mut i8) -> *mut i8 {
    // SAFETY: p points at a nul-terminated string.
    unsafe {
        while *p != 0 {
            p = p.add(1);
        }
    }
    p
}

unsafe fn put(mut dst: *mut i8, mut src: *const i8) -> *mut i8 {
    // SAFETY: src is nul-terminated, dst has room for it.
    unsafe {
        while *src != 0 {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
        }
    }
    dst
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a3e40(param_1: u32, param_2: i16, param_3: *mut i8) {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // where the original accesses them.
    unsafe {
        let mut b7: bool = false;
        *param_3 = 0;
        let i6: u32 = (param_1 & 0xffff).wrapping_mul(0x23);
        let base = DAT_006d4988;
        let i2 = base.add(i6 as usize);
        let mut p10: *mut i8 = base
            .add(0x1f)
            .add(i6 as usize)
            .cast::<*mut i8>()
            .read_unaligned();
        let names = (&raw mut DAT_0070acf0).cast::<i8>();
        if p10.is_null() {
            if param_2 != 1 && param_2 != 0 {
                let src = names.add((*i2.add(0x16) as usize).wrapping_mul(0x566));
                let e = put(param_3, src);
                *e = 0;
                return;
            }
            let src = (&raw mut DAT_0073b268)
                .cast::<*mut i8>()
                .add(*i2.add(3) as usize)
                .read();
            let e = put(param_3, src);
            *e = 0;
            let nm = names.add((*base.add(0x16).add(i6 as usize) as usize).wrapping_mul(0x566));
            if *nm == 0 {
                return;
            }
            let e = str_end(param_3);
            *e = b' ' as i8;
            *e.add(1) = 0;
            let e = str_end(param_3);
            let e = put(e, nm);
            *e = 0;
            return;
        }
        let b3 = *i2.add(3);
        if *p10 != 0 {
            if 0x8e < b3 && b3 < 0x9c {
                let src = (&raw mut DAT_0073b1b4)
                    .cast::<*mut i8>()
                    .add(b3 as usize)
                    .read();
                let e = put(param_3, src);
                *e = 0;
                return;
            }
            if b3 < 0x46 {
                return;
            }
            if 0x54 < b3 {
                return;
            }
            let src = (&raw mut DAT_0073b268)
                .cast::<*mut i8>()
                .add(b3 as usize)
                .read();
            let e = put(param_3, src);
            *e = 0;
            return;
        }
        let p5: *mut u32 = p10.add(0xa7).cast::<*mut u32>().read_unaligned();
        let mut u9: u32 = *i2.add(0x16) as u32;
        let idx = (*(p5 as *const u8).add(4) as usize).wrapping_mul(0xef);
        if param_2 == 1 {
            let p8: *mut i8 = (&raw mut DAT_0052b07a)
                .cast::<i8>()
                .add(idx)
                .cast::<*mut i8>()
                .read_unaligned();
            let e = put(param_3, p8);
            *e = 0;
            b7 = true;
        } else if param_2 == 0 {
            let p8 = (&raw mut DAT_0052b070).cast::<i8>().add(idx);
            let e = put(param_3, p8);
            *e = 0;
            b7 = true;
        }
        p10 = names.add((u9 as usize).wrapping_mul(0x566));
        if *p10 != 0 {
            if b7 {
                let e = str_end(param_3);
                *e = b' ' as i8;
                *e.add(1) = 0;
            }
            let e = str_end(param_3);
            let e = put(e, p10);
            b7 = true;
            *e = 0;
        }
        u9 = FUN_004ac6f0(u9, p5);
        if (u9 as u16) == 0 {
            return;
        }
        if b7 {
            let e = str_end(param_3);
            *e = b' ' as i8;
            *e.add(1) = 0;
        }
        if 9 < (u9 as u16) {
            let e = str_end(param_3);
            *e = ((u9 & 0xffff) / 10) as i8;
            *e = (*e).wrapping_add(b'0' as i8);
            *e.add(1) = 0;
            let e = str_end(param_3);
            *e = ((u9 & 0xffff) % 10) as i8;
            *e = (*e).wrapping_add(b'0' as i8);
            *e.add(1) = 0;
            return;
        }
        let e = str_end(param_3);
        *e = (u9 as i8).wrapping_add(b'0' as i8);
        *e.add(1) = 0;
    }
}
