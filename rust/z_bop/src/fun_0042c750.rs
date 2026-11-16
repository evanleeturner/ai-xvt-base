// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233ec: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_009d8b68: i8;
    static mut DAT_009a20ae: u16;
    static mut DAT_009d6940: [u8; 4];
    static mut DAT_009d694e: u16;
    static mut DAT_009d694a: u16;
    static mut DAT_009d6950: u16;
    static mut DAT_009d6944: i32;
    static mut DAT_00a606f0: i32;
    static mut DAT_00a606f4: i32;
    static mut DAT_00a606f8: i32;
    static mut DAT_00a606fc: i32;
    static mut DAT_00a60700: i32;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_009a6fe4: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9640(a: i8);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9630(a: i32);
    fn FUN_004a95c0(a: i32);
    fn FUN_004a9590(a: i32);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9960(a: i32);
    fn FUN_004277f0(a: u16, b: i32, c: i32);
    fn FUN_0042cbd0(a: i32, b: i32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042c750(param_1: i16, param_2: i16) {
    let mut param_1 = param_1;
    let s_var1: i16;
    let s_stack_14: i16;
    let s_stack_10: i16;
    let s_stack_18: i16;
    let s_stack_c: i16;
    let mut s_stack_8: i16;
    let s_stack_4: i16;
    let s_var3: i16;

    // SAFETY: the globals and callees are the game's own; the pointer chain
    // is read from the game's own tables exactly as the original does.
    unsafe {
        if DAT_005233ec == 0x13 {
            s_var1 = 8;
            s_stack_14 = 0x1a;
            s_stack_10 = 0x35;
            s_stack_18 = 0x5a;
            s_stack_c = 0x14;
            s_stack_8 = 0x4b;
            s_stack_4 = 0x2d;
            s_var3 = param_2.wrapping_add(-6);
        } else if DAT_005233ec == 0x101 {
            s_var1 = 0x10;
            s_stack_14 = 0x34;
            s_stack_10 = 0x6a;
            s_stack_18 = 0xb4;
            s_stack_c = 0x28;
            s_stack_8 = 0x96;
            s_stack_4 = 0x5a;
            param_1 = param_1.wrapping_add(10);
            s_var3 = param_2.wrapping_add(-10);
        } else {
            s_var1 = 8;
            s_stack_14 = 0x1a;
            s_stack_10 = 0x35;
            s_stack_18 = 0x5a;
            s_stack_c = 0x14;
            s_stack_8 = 0x4b;
            s_stack_4 = 0x2d;
            s_var3 = param_2.wrapping_add(-6);
        }

        let base = (&raw mut DAT_009e9670) as usize as u32;
        let addr_a = base.wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd));
        let i_var_a = (addr_a as usize as *const u32).read_unaligned();
        let addr_b = i_var_a
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8 as u32);
        let i_var_b = (addr_b as usize as *const u32).read_unaligned();
        let addr_c = i_var_b.wrapping_add(0xa9);
        let i_var_c = (addr_c as usize as *const u32).read_unaligned();
        let b_var_d = ((i_var_c.wrapping_add(4)) as usize as *const u8).read();

        let s_var2: i16 = match b_var_d {
            7 | 8 | 0xb | 0xf => s_var1,
            _ => s_var1.wrapping_neg(),
        };
        let s_var2: i16 = param_1.wrapping_add(s_var2);
        let v3 = i32::from(s_var3);
        let v2 = i32::from(s_var2);
        let w = || i32::from(DAT_009a20ae);

        if DAT_009d8b68 != 0 {
            FUN_004a9640(1);
            FUN_004a9520(
                s_var1.wrapping_add(s_var2),
                s_var3,
                (i32::from(s_var1).wrapping_mul(10).wrapping_add(v2)) as i16,
                w().wrapping_add(v3) as i16,
            );
            FUN_004a9630(1);
            FUN_004a95c0(0x30);
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            FUN_004a9590(0x49);
            FUN_004a9500(s_stack_14.wrapping_add(s_var2), s_var3);
            FUN_004a9960(DAT_00a606f0);
            FUN_004a9590(0x4a);
            FUN_004a9500(s_stack_10.wrapping_add(s_var2), s_var3);
            FUN_004277f0(u16::from(DAT_009d6940[3]), 2, 2);
            FUN_004a9520(
                s_var2,
                w().wrapping_add(v3).wrapping_add(1) as i16,
                s_stack_18.wrapping_add(s_var2),
                w().wrapping_mul(5).wrapping_add(v3).wrapping_add(1) as i16,
            );
            FUN_004a9590(0x45);
            FUN_004a9500(s_var2, w().wrapping_add(v3).wrapping_add(1) as i16);
            FUN_004a9960(DAT_00a606f4);
            FUN_004a9500(
                s_var2,
                v3.wrapping_add(w().wrapping_mul(2)).wrapping_add(1) as i16,
            );
            FUN_004a9960(DAT_00a606f8);
            FUN_004a9590(0x4d);
            FUN_004a9500(
                s_var2,
                w().wrapping_mul(3).wrapping_add(v3).wrapping_add(1) as i16,
            );
            FUN_004a9960(DAT_00a606fc);
            FUN_004a9590(0x51);
            FUN_004a9500(
                s_var2.wrapping_add(s_stack_c),
                v3.wrapping_add(w().wrapping_mul(4)).wrapping_add(1) as i16,
            );
            FUN_004a9960(DAT_00a60700);
        }

        FUN_004a9640(1);
        FUN_004a9520(
            s_var2,
            w().wrapping_add(v3).wrapping_add(1) as i16,
            s_stack_18.wrapping_add(s_var2),
            w().wrapping_mul(5).wrapping_add(v3).wrapping_add(1) as i16,
        );
        FUN_004a9630(1);
        FUN_004a95c0(0x30);
        FUN_004a9590(0x46);
        s_stack_8 = s_stack_8.wrapping_add(s_var2);
        FUN_004a9500(s_stack_8, w().wrapping_add(v3).wrapping_add(1) as i16);
        FUN_004277f0(DAT_009d694e, 3, 1);
        let g: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
        g(0x20);
        FUN_004a9500(
            s_stack_8,
            v3.wrapping_add(w().wrapping_mul(2)).wrapping_add(1) as i16,
        );
        FUN_004277f0(DAT_009d694a, 3, 1);
        let g: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
        g(0x20);
        FUN_004a9590(0x4e);
        FUN_004a9500(
            s_stack_8,
            w().wrapping_mul(3).wrapping_add(v3).wrapping_add(1) as i16,
        );
        FUN_004277f0(DAT_009d6950, 3, 1);
        let g: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
        g(0x20);
        FUN_004a9590(0x52);
        FUN_004a9500(
            s_var2.wrapping_add(s_stack_4),
            v3.wrapping_add(w().wrapping_mul(4)).wrapping_add(1) as i16,
        );
        FUN_0042cbd0(DAT_009d6944, 6, 1);
        let g: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
        g(0x20);
        FUN_004a9640(2);
    }
}
