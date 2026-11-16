// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757e78: [u8; 0];
    static mut DAT_0064f650: i32;
    static mut DAT_00758824: i32;
    static mut DAT_00859c2c: i32;
    static mut DAT_00532a34: i32;
    static mut DAT_00532a48: [u8; 0];
    static mut DAT_0081bcd0: u16;
    static mut DAT_0081b854: u16;
    static mut DAT_0081bce8: u16;
    static mut DAT_0081b6fc: u16;
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_005321b8: [i8; 0];
    static mut DAT_0081bcb0: i32;
    static mut DAT_0085e7c5: i8;
    static mut DAT_00531990: [i8; 0];
    static mut DAT_00532a3c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004dcfd0(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004daac0(a: i32, s: *mut i8, r: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004d9010(a: *mut i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32;
    fn FUN_004dd3e0(a: *mut i32, b: i32, c: i32, d: i16, e: i32);
    fn FUN_004d0e60(a: *mut core::ffi::c_void, b: i32, c: i32);
    fn FUN_004dd000(a: *mut i32, b: *mut i32);
    fn FUN_004c9300(a: i32);
    fn FUN_004dd7d0(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004dd570(a: *mut i32, b: i32, c: i32, d: i16);
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004de190(a: *mut i8, b: i32, c: i32, d: i32);
    fn FUN_004dd030(a: *mut i32, b: i32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6320(param_1: i32) -> i32 {
    // SAFETY: all globals and callees are the game's own; the row pointer
    // walks the game's table of 0x3b-byte entries exactly as the original.
    unsafe {
        let mut ai_rect_30: [i32; 4] = [0; 4];
        let mut i_stack_20: i32;
        let mut i_stack_18: i32 = 0;
        let mut ai_stack_14: [i32; 5] = [0; 5];

        if param_1 == 0 {
            DAT_0064f650 = 0;
        }
        let d_stack_1c = GetTickCount();
        let rect = ai_rect_30.as_mut_ptr();
        let a14 = ai_stack_14.as_mut_ptr();
        FUN_004dcfd0(rect, 0x58, 0x5e, 0x1a0, 0x6d);
        let pc_var2 = FUN_004dcfa0(0x10);
        FUN_004daac0(0xc, pc_var2, rect, 0, 1, 0xffff);
        FUN_004ddc70(a14, &raw mut i_stack_18);
        if 6 < DAT_00758824 {
            FUN_004dcfd0(rect, 0x1a5, 0x72, 0x1ae, 0xcc);
            DAT_0064f650 = FUN_004d9010(rect, DAT_0064f650, DAT_00758824, 0, 5, DAT_00859c2c, 5);
        }
        FUN_004dcfd0(rect, 0x58, 0x72, 0x1a3, 0x80);
        i_stack_20 = -1;
        if DAT_0064f650 < DAT_0064f650.wrapping_add(6) {
            let mut pbvar4: *mut u8 = (&raw mut DAT_00757e78)
                .cast::<u8>()
                .byte_add((DAT_0064f650 as u32).wrapping_mul(0x3b) as usize);
            let mut i_var7 = DAT_0064f650;
            loop {
                if DAT_00758824 <= i_var7 {
                    return i_stack_20;
                }
                if i_var7 == DAT_00532a34 {
                    FUN_004dd3e0(rect, 0, 0, DAT_00859c2c as i16, 1);
                }
                let u_var9: u32;
                if *pbvar4 == 2 {
                    let u_tmp = pbvar4.wrapping_sub(8).cast::<u32>().read_unaligned();
                    if u_tmp == 0 {
                        if *pbvar4.add(2) == 0 {
                            u_var9 = DAT_0081bcd0 as u32;
                        } else {
                            u_var9 = DAT_0081b854 as u32;
                        }
                    } else {
                        let mut v = 0xffffu32;
                        let u_tmp2 = pbvar4.wrapping_sub(8).cast::<u32>().read_unaligned();
                        if u_tmp2 < 9 {
                            v = DAT_0081bce8 as u32;
                        }
                        u_var9 = v;
                    }
                } else {
                    u_var9 = DAT_0081b6fc as u32;
                }
                if *pbvar4.add(1) != 0 {
                    FUN_004d0e60(
                        (&raw mut DAT_00532a48).cast::<core::ffi::c_void>(),
                        (ai_rect_30[0] as u32).wrapping_sub(6) as i32,
                        (ai_rect_30[1] as u32).wrapping_add(1) as i32,
                    );
                }
                let pbvar5: *mut u8;
                if *pbvar4 == 2 {
                    pbvar5 = pbvar4.wrapping_sub(0x38);
                } else {
                    let i_var8 = (*pbvar4 as i32) - 1;
                    _sprintf(
                        (&raw mut DAT_0081b740).cast::<i8>(),
                        (&raw const DAT_00532a3c).cast::<i8>(),
                        pbvar4.wrapping_sub(0x38),
                        i_var8 / 100 + 1,
                        i_var8 % 100,
                    );
                    pbvar5 = (&raw mut DAT_0081b740).cast::<u8>();
                }
                FUN_004daac0(0xc, pbvar5.cast::<i8>(), rect, 0, 1, u_var9);
                let u_tmp = pbvar4.wrapping_sub(8).cast::<u32>().read_unaligned();
                if u_tmp < 9 {
                    FUN_004dd000(a14.add(1), rect);
                    ai_stack_14[1] = 0x11d;
                    let u_tmp = pbvar4.wrapping_sub(8).cast::<u32>().read_unaligned();
                    _sprintf(
                        (&raw mut DAT_0081b740).cast::<i8>(),
                        (&raw const DAT_005321b8).cast::<i8>(),
                        u_tmp,
                    );
                    FUN_004daac0(
                        0xc,
                        (&raw mut DAT_0081b740).cast::<i8>(),
                        a14.add(1),
                        0,
                        1,
                        u_var9,
                    );
                    ai_stack_14[1] = 0x176;
                    let u_tmp = pbvar4.wrapping_sub(4).cast::<u32>().read_unaligned();
                    FUN_004c9300((d_stack_1c.wrapping_sub(u_tmp) / 1000) as i32);
                    FUN_004daac0(
                        0xc,
                        (&raw mut DAT_0081b740).cast::<i8>(),
                        a14.add(1),
                        0,
                        1,
                        u_var9,
                    );
                }
                let i_var3 = FUN_004dd7d0(rect, ai_stack_14[0], i_stack_18);
                let mut i_var8 = i_stack_20;
                if i_var3 != 0 {
                    FUN_004dd570(rect, 0, 0, DAT_0081bcb0 as i16);
                    let mut u_var1 = FUN_004db570();
                    if u_var1 != 0 || {
                        u_var1 = FUN_004db590();
                        u_var1 != 0
                    } {
                        i_var8 = i_var7;
                        if DAT_0085e7c5 != 0 {
                            FUN_004de190((&raw mut DAT_00531990).cast::<i8>(), 1, 0, 0xff);
                        }
                    }
                }
                i_stack_20 = i_var8;
                pbvar4 = pbvar4.wrapping_add(0x3b);
                i_var7 = i_var7.wrapping_add(1);
                FUN_004dd030(rect, 0, 0xf);
                if !(i_var7 < DAT_0064f650.wrapping_add(6)) {
                    break;
                }
            }
        }
        i_stack_20
    }
}
