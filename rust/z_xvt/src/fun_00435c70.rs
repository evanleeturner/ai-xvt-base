// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bb1b: u8;
    static mut DAT_0071bb1f: u8;
    static mut DAT_0071bb23: u8;
    static mut DAT_0072f450: i32;
    static mut DAT_0072f44c: i32;
    static mut DAT_0072f448: i32;
    static mut DAT_006db7b4: i32;
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
    static mut DAT_00732568: i32;
    static mut DAT_0073255c: i32;
    static mut DAT_00732558: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732584: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732e8c: i16;
    static mut DAT_006d9980: i16;
    static mut DAT_006fedd4: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040ca60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca00(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040c970(a: u32, b: u32, c: u32);
    fn FUN_004a6a80();
    fn FUN_0040cad0(a: u32, b: u32) -> i32;
    fn FUN_0040ca90(a: u32, b: u32) -> i32;
    fn FUN_004a6f90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00435c70() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the byte offsets index the game's record table as the original does.
    unsafe {
        DAT_00732a24 = 0;
        DAT_00732a28 = 0;
        DAT_00732a2c = 0;
        let mut local_8: i32 = -0x100000;
        let mut local_c: i32 = 0x21;
        loop {
            let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
            let f1f = (&raw mut DAT_0071bb1f)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            DAT_0072f450 = (local_8 as u32).wrapping_sub(f1f as u32) as i32;
            let f1b = (&raw mut DAT_0071bb1b)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            DAT_0072f44c = 0xfff00000u32.wrapping_sub(f1b as u32) as i32;
            let f23 = (&raw mut DAT_0071bb23)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            DAT_0072f448 = 0xffff0000u32.wrapping_sub(f23 as u32) as i32;
            DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            let mut u11 = (DAT_00732568 as u32)
                .wrapping_mul(0x40)
                .wrapping_add(DAT_006db7b4 as u32);
            if 0 < (u11 as i32) || 0 < DAT_006db7b4 {
                DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
                DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
                let u1 = (DAT_0073255c as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(DAT_006db7ac as u32);
                let u2 = (DAT_00732558 as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(DAT_006db7b0 as u32);
                let mut u7 = DAT_006db7b4 as u32;
                let mut u10 = u2;
                let mut u12 = u1;
                if (u11 as i32) < 1 {
                    u7 = u11;
                    u10 = DAT_006db7b0 as u32;
                    u11 = DAT_006db7b4 as u32;
                    u12 = DAT_006db7ac as u32;
                    DAT_006db7ac = u1 as i32;
                    DAT_006db7b0 = u2 as i32;
                }
                DAT_006db7b4 = u7 as i32;
                if (u7 as i32) < 1 {
                    FUN_0040c970(u12, u10, u11);
                }
                FUN_004a6a80();
                let i8_ = DAT_00732e8c as i32;
                let i3 = DAT_006d9980 as i32;
                let u13: u32 = 0x31;
                let mut i4 = FUN_0040cad0(u10, u11);
                i4 = (i4 as u32).wrapping_add(i8_ as u32) as i32;
                let mut i5 = FUN_0040ca90(u12, u11);
                u11 = DAT_006db7b4 as u32;
                i5 = (i5 as u32).wrapping_add(i3 as u32) as i32;
                let mut i6 = FUN_0040cad0(DAT_006db7b0 as u32, u11);
                i6 = (i6 as u32).wrapping_add(i8_ as u32) as i32;
                let i8b = FUN_0040ca90(DAT_006db7ac as u32, u11);
                let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) =
                    core::mem::transmute(DAT_006fedd4);
                f((i8b as u32).wrapping_add(i3 as u32) as i32, i6, i5, i4, u13);
                FUN_004a6f90();
            }
            local_c = local_c.wrapping_sub(1);
            local_8 = (local_8 as u32).wrapping_add(0x10000) as i32;
            if local_c == 0 {
                break;
            }
        }
        local_c = 0x21;
        let i3 = (DAT_00732a28 as u32).wrapping_sub(0x100000) as i32;
        let mut local_4 = (DAT_00732a2c as u32).wrapping_sub(0x100000) as i32;
        loop {
            let idx = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
            let f1f = (&raw mut DAT_0071bb1f)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            DAT_0072f450 = (i3 as u32).wrapping_sub(f1f as u32) as i32;
            let f1b = (&raw mut DAT_0071bb1b)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            DAT_0072f44c = (local_4 as u32).wrapping_sub(f1b as u32) as i32;
            let f23 = (&raw mut DAT_0071bb23)
                .byte_add(idx)
                .cast::<i32>()
                .read_unaligned();
            DAT_0072f448 = 0xffff0000u32.wrapping_sub(f23 as u32) as i32;
            DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            let mut u11 = (DAT_00732530 as u32)
                .wrapping_mul(0x40)
                .wrapping_add(DAT_006db7b4 as u32);
            if 0 < (u11 as i32) || 0 < DAT_006db7b4 {
                DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
                DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
                let u1 = (DAT_00732584 as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(DAT_006db7ac as u32);
                let u2 = (DAT_00732578 as u32)
                    .wrapping_mul(0x40)
                    .wrapping_add(DAT_006db7b0 as u32);
                let mut u7 = DAT_006db7b4 as u32;
                let mut u10 = u2;
                let mut u12 = u1;
                if (u11 as i32) < 1 {
                    u7 = u11;
                    u10 = DAT_006db7b0 as u32;
                    u11 = DAT_006db7b4 as u32;
                    u12 = DAT_006db7ac as u32;
                    DAT_006db7ac = u1 as i32;
                    DAT_006db7b0 = u2 as i32;
                }
                DAT_006db7b4 = u7 as i32;
                if (u7 as i32) < 1 {
                    FUN_0040c970(u12, u10, u11);
                }
                FUN_004a6a80();
                let i9 = DAT_00732e8c as i32;
                let i4 = DAT_006d9980 as i32;
                let u13: u32 = 0x31;
                let mut i5 = FUN_0040cad0(u10, u11);
                i5 = (i5 as u32).wrapping_add(i9 as u32) as i32;
                let mut i6 = FUN_0040ca90(u12, u11);
                u11 = DAT_006db7b4 as u32;
                i6 = (i6 as u32).wrapping_add(i4 as u32) as i32;
                let mut i8_ = FUN_0040cad0(DAT_006db7b0 as u32, u11);
                i8_ = (i8_ as u32).wrapping_add(i9 as u32) as i32;
                let i9b = FUN_0040ca90(DAT_006db7ac as u32, u11);
                let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) =
                    core::mem::transmute(DAT_006fedd4);
                f(
                    (i9b as u32).wrapping_add(i4 as u32) as i32,
                    i8_,
                    i6,
                    i5,
                    u13,
                );
                FUN_004a6f90();
            }
            local_c = local_c.wrapping_sub(1);
            local_4 = (local_4 as u32).wrapping_add(0x10000) as i32;
            if local_c == 0 {
                break;
            }
        }
    }
}
