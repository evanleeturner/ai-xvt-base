// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bcf0: [i8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_0081e082: [i8; 0];
    static mut DAT_0081fb96: [u8; 0];
    static mut DAT_0081fbfa: [u8; 0];
    static mut DAT_0081fbe2: [u8; 0];
    static mut DAT_0081fc2a: [u8; 0];
    static mut DAT_0081fc12: [u8; 0];
    static mut DAT_005321f4: [i8; 0];
    static mut DAT_005321b8: [i8; 0];
    static mut DAT_005321e4: [i8; 0];
    static mut DAT_005321f8: [i8; 0];
    static mut DAT_00532204: [i8; 0];
    static mut DAT_00532210: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: i32) -> *mut i8;
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004da7c0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d9d20(param_1: *mut i32, param_2: *mut i8, param_3: *mut i8, param_4: i32);
}

/// Reads the index word, then the table value for slot `i`.
unsafe fn table_val(base: *mut u8, i: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; reads mirror the original.
    unsafe {
        let idx = (&raw mut DAT_0081fb96)
            .cast::<u8>()
            .cast::<i32>()
            .read_unaligned();
        let off = (idx as u32)
            .wrapping_mul(0x3a09)
            .wrapping_add(i as u32)
            .wrapping_mul(4);
        base.byte_add(off as usize).cast::<i32>().read_unaligned()
    }
}

unsafe fn draw_list(local_10: *mut i32, base: *mut u8, y0: i32, x: i32, vx: i32, city: bool) {
    // SAFETY: calls into the game's own routines and globals.
    unsafe {
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        let mut y = y0;
        let mut i = 0;
        loop {
            if table_val(base, i) != 0 {
                let fmt: *const i8 = if city {
                    let idx = (&raw mut DAT_0081fb96)
                        .cast::<u8>()
                        .cast::<i32>()
                        .read_unaligned();
                    if idx == 0 {
                        (&raw mut DAT_00532210).cast::<i8>()
                    } else {
                        (&raw mut DAT_00532204).cast::<i8>()
                    }
                } else {
                    (&raw mut DAT_005321f8).cast::<i8>()
                };
                _sprintf(buf, fmt, i.wrapping_add(1));
                FUN_004d0e60(buf, x, y);
                let v = table_val(base, i);
                _sprintf(buf, (&raw mut DAT_005321b8).cast::<i8>(), v);
                FUN_004da7c0(10, buf, vx, y.wrapping_add(1), 0xffff);
            }
            y = y.wrapping_add(0x16);
            i = i.wrapping_add(1);
            FUN_004dd030(local_10, 0, 0x16);
            if i >= 6 {
                break;
            }
        }
    }
}

unsafe fn draw_names(local_10: *mut i32, base: *mut u8, first: i32) {
    // SAFETY: calls into the game's own routines and globals.
    unsafe {
        let mut i = 0;
        loop {
            if table_val(base, i) != 0 {
                let p = FUN_004dcfa0(i.wrapping_add(first));
                FUN_004d9d20(local_10, core::ptr::null_mut(), p, 0xc);
            }
            i = i.wrapping_add(1);
            FUN_004dd030(local_10, 0, 0x16);
            if i >= 6 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c7c10() -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    let r = local_10.as_mut_ptr();
    // SAFETY: the callees and globals are the game's own memory.
    unsafe {
        let buf = (&raw mut DAT_0081b740).cast::<i8>();
        let name = (&raw mut DAT_0081bcf0).cast::<i8>();
        let t_a = (&raw mut DAT_0081fbfa).cast::<u8>();
        let t_b = (&raw mut DAT_0081fbe2).cast::<u8>();
        let t_c = (&raw mut DAT_0081fc2a).cast::<u8>();
        let t_d = (&raw mut DAT_0081fc12).cast::<u8>();

        FUN_004dcfd0(r, 0x54, 0x5a, 0x194, 0x6a);
        if *name == 0 {
            let p = FUN_004dcfa0(9);
            _sprintf(buf, (&raw mut DAT_005321f4).cast::<i8>(), p);
        } else {
            let p = FUN_004dcfa0(9);
            _sprintf(
                buf,
                (&raw mut DAT_005321e4).cast::<i8>(),
                p,
                6i32,
                (&raw mut DAT_0081e082).cast::<i8>(),
                4i32,
                name,
            );
        }
        FUN_004da920(0xf, buf, r, 0xffff);
        if *name == 0 {
            return 0;
        }

        FUN_004dcfd0(r, 0x10c, 0x76, 0x1a8, 0x84);
        let p = FUN_004dcfa0(0x16e);
        FUN_004da920(0xc, p, r, 0xffff);
        FUN_004dcfd0(r, 0x161, 0x85, 0x183, 0x92);
        draw_list(r, t_a, 0x85, 0x161, 0x193, false);

        FUN_004dcfd0(r, 0x61, 0x76, 0xfd, 0x84);
        let p = FUN_004dcfa0(0x16d);
        FUN_004da920(0xc, p, r, 0xffff);
        FUN_004dcfd0(r, 0xb6, 0x85, 0xd8, 0x92);
        draw_list(r, t_b, 0x85, 0xb6, 0xe8, false);

        FUN_004dcfd0(r, 0x10c, 0x10d, 0x1a8, 0x11b);
        let p = FUN_004dcfa0(0x170);
        FUN_004da920(0xc, p, r, 0xffff);
        FUN_004dcfd0(r, 0x161, 0x11c, 0x183, 0x129);
        draw_list(r, t_c, 0x11c, 0x161, 0x193, false);

        FUN_004dcfd0(r, 0x61, 0x10d, 0xfd, 0x11b);
        let p = FUN_004dcfa0(0x16f);
        FUN_004da920(0xc, p, r, 0xffff);
        FUN_004dcfd0(r, 0xb6, 0x11c, 0xd8, 0x129);
        draw_list(r, t_d, 0x11c, 0xb6, 0xe8, true);

        FUN_004dcfd0(r, 0x161, 0x85, 0x183, 0x92);
        draw_names(r, t_a, 0x171);
        FUN_004dcfd0(r, 0xb6, 0x85, 0xd8, 0x92);
        draw_names(r, t_b, 0x171);
        FUN_004dcfd0(r, 0x161, 0x11c, 0x183, 0x129);
        draw_names(r, t_c, 0x171);
        FUN_004dcfd0(r, 0xb6, 0x11c, 0xd8, 0x129);
        draw_names(r, t_d, 0x286);
    }
    1
}
