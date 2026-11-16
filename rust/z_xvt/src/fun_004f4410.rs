// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007438d2: [u8; 0];
    static mut DAT_007438d7: [u8; 0];
    static mut DAT_00650434: i32;
    static mut DAT_00743d02: [i16; 0];
    static mut DAT_00743d2e: [i16; 0];
    static mut DAT_00533800: [i32; 0];
    static mut DAT_005339a8: [i8; 0];
    static mut DAT_007577b8: [i32; 0];
    static mut DAT_007577d0: [u32; 0];
    static mut DAT_007577d8: [i32; 0];
    static mut DAT_007577e8: [i32; 0];
    static mut DAT_00533ea0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f33c0(a: *mut i32, b: i32, c: i32, d: *mut i16, e: *mut i16);
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004dd070(a: *mut i32, b: i32, c: i32);
    fn FUN_004dd3e0(a: *mut i32, b: i32, c: i32, d: i16, e: i32);
    fn FUN_004dd570(a: *mut i32, b: i32, c: i32, d: i16);
    fn FUN_004d0b60(a: *mut i8, b: *mut i32, c: i32, d: i32, e: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f4410(param_1: *mut i32, param_2: u32, param_3: i16, param_4: i16) {
    let _ = param_2;
    let mut s_stack_18: i16;
    let mut s_stack_16: i16;
    let mut s_stack_14: i16 = 0;
    let mut s_stack_12: i16 = 0;
    let mut ai_stack_10: [i32; 4] = [0; 4];
    let s_var5: i16;
    let s_var6: i16;
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers handed to callees point at live locals.
    unsafe {
        let i_var4 = param_3 as i32;
        let b_var2 = (&raw mut DAT_007438d2)
            .cast::<u8>()
            .offset(i_var4.wrapping_mul(0x562) as isize)
            .read();
        let mut i_var3 = i_var4.wrapping_mul(0x2b1).wrapping_add(DAT_00650434);
        let sw = (&raw mut DAT_007438d7)
            .cast::<u8>()
            .offset(i_var4.wrapping_mul(0x562) as isize)
            .read();
        s_var6 = match sw {
            0 => 0,
            1 | 4 => 8,
            2 => 0x18,
            3 => 0x10,
            5 => 0x20,
            _ => 0,
        };
        let v1 = (&raw mut DAT_00743d02)
            .cast::<i16>()
            .offset(i_var3 as isize)
            .read();
        let v2 = (&raw mut DAT_00743d2e)
            .cast::<i16>()
            .offset(i_var3 as isize)
            .read();
        FUN_004f33c0(
            param_1,
            v1 as i32,
            v2 as i32,
            &raw mut s_stack_12,
            &raw mut s_stack_14,
        );
        let i_var7 = (&raw mut DAT_00533800)
            .cast::<i32>()
            .offset(b_var2 as isize)
            .read()
            .wrapping_mul(0x10);
        let base = (&raw mut DAT_005339a8).cast::<i8>();
        let rd = |off: i32| -> i32 {
            base.offset(i_var7.wrapping_add(off) as isize)
                .cast::<i32>()
                .read_unaligned()
        };
        i_var3 = rd(8).wrapping_sub(rd(0)).wrapping_add(1);
        let i_var4 = rd(0xc).wrapping_sub(rd(4)).wrapping_add(1);
        s_stack_12 = s_stack_12.wrapping_sub((i_var3 >> 1) as i16);
        s_stack_14 = s_stack_14.wrapping_sub((i_var4 >> 1) as i16);
        let p10 = ai_stack_10.as_mut_ptr();
        FUN_004dcfd0(
            p10,
            s_stack_12 as i32,
            s_stack_14 as i32,
            i_var3.wrapping_add(s_stack_12 as i32).wrapping_sub(1),
            i_var4.wrapping_add(s_stack_14 as i32).wrapping_sub(1),
        );
        let p4 = param_4 as i32;
        let v6 = s_var6 as i32;
        if param_4 < 0xc {
            if param_4 < 4 {
                s_var5 = (p4 + 1) as i16;
                s_stack_16 = 0x10;
                s_stack_18 = v6.wrapping_add(p4.wrapping_mul(-2)).wrapping_add(7) as i16;
            } else {
                s_stack_18 = (v6 + 1) as i16;
                if param_4 < 8 {
                    s_stack_16 = ((0xb - p4) * 2) as i16;
                    s_var5 = 4;
                } else {
                    s_stack_16 = ((0xb - p4) * 2) as i16;
                    s_var5 = (0xc - p4) as i16;
                }
            }
            if 7 < param_4 {
                let i3 = 0xb - p4;
                FUN_004dd070(p10, i3, i3);
                let a = (&raw mut DAT_007577d8)
                    .cast::<i32>()
                    .offset(v6 as isize)
                    .read();
                FUN_004dd3e0(p10, 0, 0, a as i16, 1);
                let b = (&raw mut DAT_007577b8)
                    .cast::<i32>()
                    .offset((v6 + p4) as isize)
                    .read();
                FUN_004dd570(p10, 0, 0, b as i16);
            }
            let mut s5 = s_var5;
            if 0 < s5 {
                let name = (&raw mut DAT_00533ea0).cast::<i8>();
                let icon = base.offset(i_var7 as isize).cast::<i32>();
                loop {
                    let pu = (&raw mut DAT_007577d0)
                        .cast::<u32>()
                        .offset(s_stack_18 as isize);
                    let i3 = s_stack_16 as i32;
                    let x = s_stack_12 as i32;
                    let y = s_stack_14 as i32;
                    FUN_004d0b60(
                        name,
                        icon,
                        x.wrapping_sub(i3),
                        y.wrapping_sub(i3),
                        pu.read(),
                    );
                    FUN_004d0b60(
                        name,
                        icon,
                        x.wrapping_add(i3),
                        y.wrapping_sub(i3),
                        pu.read(),
                    );
                    FUN_004d0b60(
                        name,
                        icon,
                        x.wrapping_sub(i3),
                        y.wrapping_add(i3),
                        pu.read(),
                    );
                    FUN_004d0b60(
                        name,
                        icon,
                        x.wrapping_add(i3),
                        y.wrapping_add(i3),
                        pu.read(),
                    );
                    s_stack_18 = s_stack_18.wrapping_add(2);
                    s_stack_16 = s_stack_16.wrapping_add(-2);
                    s5 = s5.wrapping_sub(1);
                    if s5 == 0 {
                        break;
                    }
                }
            }
        } else {
            FUN_004dd070(p10, -2, -2);
            let a = (&raw mut DAT_007577d8)
                .cast::<i32>()
                .offset(v6 as isize)
                .read();
            FUN_004dd3e0(p10, 0, 0, a as i16, 1);
            let b = (&raw mut DAT_007577e8)
                .cast::<i32>()
                .offset(v6 as isize)
                .read();
            FUN_004dd570(p10, 0, 0, b as i16);
        }
    }
}
