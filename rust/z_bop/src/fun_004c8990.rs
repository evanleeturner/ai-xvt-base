// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a2e0: i8;
    static mut DAT_00b6bc9e: u8;
    static mut DAT_00b69d20: u8;
    static mut DAT_00b6d962: u8;
    static mut DAT_00b6d9ca: u8;
    static mut DAT_00b6d9b2: u8;
    static mut DAT_00b6d9fa: u8;
    static mut DAT_00b6d9e2: u8;
    static mut DAT_00528118: u8;
    static mut DAT_0052b460: u8;
    static mut DAT_0052b48c: u8;
    static mut DAT_0052b49c: u8;
    static mut DAT_0052b4a8: u8;
    static mut DAT_0052b4b4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004d6460(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004db640(a: i32, s: *mut i8, p: *mut i32, c: u32);
    fn FUN_004b5570(s: *mut i8, a: i32, b: i32);
    fn FUN_004db4e0(a: i32, s: *mut i8, b: i32, c: i32, d: i32);
    fn FUN_004d64a0(p: *mut i32, a: i32, b: i32);
    fn FUN_004daa20(p: *mut i32, a: *mut i8, b: *mut i8, c: i32);
}

unsafe fn rd_val(base: *const u8, i: i32) -> i32 {
    // SAFETY: the globals are the game's own memory; read where the original reads.
    unsafe {
        let d = (&raw mut DAT_00b6d962 as *const u8)
            .cast::<i32>()
            .read_unaligned();
        let idx = (i as u32)
            .wrapping_add((d as u32).wrapping_mul(0x4278))
            .wrapping_mul(4);
        base.byte_add(idx as usize).cast::<i32>().read_unaligned()
    }
}

unsafe fn first_loop(local: *mut i32, base: *const u8, mut y: i32, x_label: i32, x_val: i32) {
    // SAFETY: the callees and globals are the game's own.
    unsafe {
        let mut i: i32 = 0;
        loop {
            if rd_val(base, i) != 0 {
                sprintf(
                    &raw mut DAT_00b69d20 as *mut i8,
                    &raw mut DAT_0052b49c as *const i8,
                    i.wrapping_add(1),
                );
                FUN_004b5570(&raw mut DAT_00b69d20 as *mut i8, x_label, y);
                let v = rd_val(base, i);
                sprintf(
                    &raw mut DAT_00b69d20 as *mut i8,
                    &raw mut DAT_0052b460 as *const i8,
                    v,
                );
                FUN_004db4e0(
                    10,
                    &raw mut DAT_00b69d20 as *mut i8,
                    x_val,
                    y.wrapping_add(1),
                    0xffff,
                );
            }
            y = y.wrapping_add(0x16);
            i = i.wrapping_add(1);
            FUN_004d64a0(local, 0, 0x16);
            if i >= 6 {
                break;
            }
        }
    }
}

unsafe fn second_loop(local: *mut i32, base: *const u8, str_base: i32) {
    // SAFETY: the callees and globals are the game's own.
    unsafe {
        let mut i: i32 = 0;
        loop {
            if rd_val(base, i) != 0 {
                let p = FUN_004ddc10(i.wrapping_add(str_base));
                FUN_004daa20(local, core::ptr::null_mut(), p, 0xc);
            }
            i = i.wrapping_add(1);
            FUN_004d64a0(local, 0, 0x16);
            if i >= 6 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c8990() -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    let l = local_10.as_mut_ptr();
    // SAFETY: the callees and globals are the game's own memory; reads and
    // writes happen in the original's order.
    unsafe {
        FUN_004d6460(l, 0x54, 0x5a, 0x194, 0x6a);
        if DAT_00b6a2e0 == 0 {
            let p = FUN_004ddc10(9);
            sprintf(
                &raw mut DAT_00b69d20 as *mut i8,
                &raw mut DAT_00528118 as *const i8,
                p,
            );
        } else {
            let p = FUN_004ddc10(9);
            sprintf(
                &raw mut DAT_00b69d20 as *mut i8,
                &raw mut DAT_0052b48c as *const i8,
                p,
                6i32,
                &raw mut DAT_00b6bc9e as *mut i8,
                4i32,
                &raw mut DAT_00b6a2e0 as *mut i8,
            );
        }
        FUN_004db640(0xf, &raw mut DAT_00b69d20 as *mut i8, l, 0xffff);
        if DAT_00b6a2e0 == 0 {
            return 0;
        }

        FUN_004d6460(l, 0x10c, 0x76, 0x1a8, 0x84);
        let p = FUN_004ddc10(0x17b);
        FUN_004db640(0xc, p, l, 0xffff);
        FUN_004d6460(l, 0x161, 0x85, 0x183, 0x92);
        first_loop(l, &raw mut DAT_00b6d9ca as *const u8, 0x85, 0x161, 0x193);

        FUN_004d6460(l, 0x61, 0x76, 0xfd, 0x84);
        let p = FUN_004ddc10(0x17a);
        FUN_004db640(0xc, p, l, 0xffff);
        FUN_004d6460(l, 0xb6, 0x85, 0xd8, 0x92);
        first_loop(l, &raw mut DAT_00b6d9b2 as *const u8, 0x85, 0xb6, 0xe8);

        FUN_004d6460(l, 0x10c, 0x10d, 0x1a8, 0x11b);
        let p = FUN_004ddc10(0x17d);
        FUN_004db640(0xc, p, l, 0xffff);
        FUN_004d6460(l, 0x161, 0x11c, 0x183, 0x129);
        first_loop(l, &raw mut DAT_00b6d9fa as *const u8, 0x11c, 0x161, 0x193);

        FUN_004d6460(l, 0x61, 0x10d, 0xfd, 0x11b);
        let p = FUN_004ddc10(0x17c);
        let mut y: i32 = 0x11c;
        FUN_004db640(0xc, p, l, 0xffff);
        let mut i: i32 = 0;
        FUN_004d6460(l, 0xb6, 0x11c, 0xd8, 0x129);
        let base = &raw mut DAT_00b6d9e2 as *const u8;
        loop {
            if rd_val(base, i) != 0 {
                let d = (&raw mut DAT_00b6d962 as *const u8)
                    .cast::<i32>()
                    .read_unaligned();
                let fmt = if d == 0 {
                    &raw mut DAT_0052b4b4 as *const i8
                } else {
                    &raw mut DAT_0052b4a8 as *const i8
                };
                sprintf(&raw mut DAT_00b69d20 as *mut i8, fmt, i.wrapping_add(1));
                FUN_004b5570(&raw mut DAT_00b69d20 as *mut i8, 0xb6, y);
                let v = rd_val(base, i);
                sprintf(
                    &raw mut DAT_00b69d20 as *mut i8,
                    &raw mut DAT_0052b460 as *const i8,
                    v,
                );
                FUN_004db4e0(
                    10,
                    &raw mut DAT_00b69d20 as *mut i8,
                    0xe8,
                    y.wrapping_add(1),
                    0xffff,
                );
            }
            y = y.wrapping_add(0x16);
            i = i.wrapping_add(1);
            FUN_004d64a0(l, 0, 0x16);
            if i >= 6 {
                break;
            }
        }

        FUN_004d6460(l, 0x161, 0x85, 0x183, 0x92);
        second_loop(l, &raw mut DAT_00b6d9ca as *const u8, 0x17e);

        FUN_004d6460(l, 0xb6, 0x85, 0xd8, 0x92);
        second_loop(l, &raw mut DAT_00b6d9b2 as *const u8, 0x17e);

        FUN_004d6460(l, 0x161, 0x11c, 0x183, 0x129);
        second_loop(l, &raw mut DAT_00b6d9fa as *const u8, 0x17e);

        FUN_004d6460(l, 0xb6, 0x11c, 0xd8, 0x129);
        second_loop(l, &raw mut DAT_00b6d9e2 as *const u8, 0x294);
    }
    1
}
