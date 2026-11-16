// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut i8;
    static mut DAT_009ec474: i16;
    static mut DAT_00a07c68: u32;
    static mut DAT_00a07c6c: i32;
    static mut DAT_00a07cc4: i32;
    static mut DAT_00a004d0: i32;
    static mut DAT_00a004ac: i32;
    static mut DAT_00a004b4: i32;
    static mut DAT_00a0049c: u32;
    static mut DAT_00a0048c: u32;
    static mut DAT_00a00494: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(a: i16, b: i16, p: *mut core::ffi::c_void);
    fn FUN_00427d30(a: i16, b: i16, p: *mut core::ffi::c_void);
    fn FUN_004290b0(a: i32, b: i32, c: i32, d: i16);
    fn FUN_0046a460(a: i16) -> i16;
    fn FUN_0046abf0(a: i32, b: i32) -> u32;
    fn FUN_0046a6d0(a: i16) -> i64;
    fn FUN_0046a3c0(a: i16) -> i64;
}

fn clamp(v: i32) -> i32 {
    let mut v = v;
    if 0x3fffffff < v {
        v = 0x3fffffff;
    }
    if v < -0x3fffffff {
        v = -0x3fff0000;
    }
    v
}

fn mul(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00483a00(param_1: i16, param_2: i16, param_3: u32, param_4: i32) {
    // SAFETY: all globals and callees are the game's own; the record table
    // pointer and the record it points into are read and written at the
    // offsets the original uses.
    unsafe {
        let i_var2 = (param_3 & 0xffff).wrapping_mul(0x23) as i32;
        let i_var10p = DAT_009a1fe8.byte_offset(i_var2 as isize);

        let pdata = i_var10p.byte_add(0x1f).cast::<u32>().read_unaligned() as *const u8;
        if pdata.byte_add(0x92).read() != 0 {
            let sa = i_var10p.byte_add(0x12).cast::<i16>().read_unaligned();
            let sb = i_var10p.byte_add(0x10).cast::<i16>().read_unaligned();
            FUN_00427bd0(sa, sb, i_var10p.cast());
            let base = DAT_009a1fe8.byte_offset(i_var2 as isize);
            let sa = base.byte_add(0x14).cast::<i16>().read_unaligned();
            FUN_00427d30(sa, 0, base.cast());
        }

        let pbase = DAT_009a1fe8.byte_offset(0x1f).byte_offset(i_var2 as isize);
        let rd = |off: usize| -> i16 {
            let p = pbase.cast::<u32>().read_unaligned() as *const u8;
            p.byte_add(off).cast::<i16>().read_unaligned()
        };

        DAT_00a07c68 = 0u32.wrapping_sub(rd(0x93) as i32 as u32);
        DAT_00a07c6c = 0u32.wrapping_sub(rd(0x95) as i32 as u32) as i32;
        DAT_00a07cc4 = 0u32.wrapping_sub(rd(0x97) as i32 as u32) as i32;
        DAT_00a004d0 = rd(0x9f) as i32;
        DAT_00a004ac = rd(0xa1) as i32;
        DAT_00a004b4 = rd(0xa3) as i32;
        DAT_00a0049c = rd(0x99) as i32 as u32;
        DAT_00a0048c = rd(0x9b) as i32 as u32;
        DAT_00a00494 = rd(0x9d) as i32;

        FUN_004290b0(
            DAT_00a0049c as i32,
            DAT_00a0048c as i32,
            DAT_00a00494,
            param_1,
        );

        let flag = DAT_009ec474 as u8;
        if (flag & 0xe) != 2 {
            FUN_004290b0(DAT_00a004d0, DAT_00a004ac, DAT_00a004b4, param_2);
        }

        let s_var3 = FUN_0046a460(0u32.wrapping_sub(DAT_00a07cc4 as u32) as i16);
        (param_4 as usize as *mut u8)
            .byte_add(0xd)
            .cast::<i16>()
            .write_unaligned(s_var3);

        let u_var6 = FUN_0046abf0(
            DAT_00a07c68 as i32,
            0u32.wrapping_sub(DAT_00a07c6c as u32) as i32,
        );
        let s_var4 = 0u32.wrapping_sub(u_var6 as i16 as i32 as u32) as i16;

        let l_var16 = FUN_0046a6d0(s_var4);
        let l_var17 = FUN_0046a3c0(s_var4);
        let s_var5 = l_var17 as i16;
        let l_var17 = FUN_0046a6d0(s_var3);
        let l_var18 = FUN_0046a3c0(s_var3);
        let s_var3 = l_var18 as i16;

        let i_var10 = l_var17 as i16 as i32;
        let local_5c = ((mul(s_var5 as i32, i_var10) as u32) >> 0xf) as i16;
        let i_var14 = l_var16 as i16 as i32;
        let local_58 = ((mul(s_var5 as i32, s_var3 as i32) as u32) >> 0xf) as i16;
        let local_54 = ((mul(i_var14, i_var10) as u32) >> 0xf) as i16;
        let local_50 = ((mul(i_var14, s_var3 as i32) as u32) >> 0xf) as i16;

        let i_var7 = 0i32.wrapping_sub(s_var5 as i32);
        let i_var8 =
            clamp(mul(DAT_00a0049c as i32, i_var14).wrapping_add(mul(DAT_00a0048c as i32, i_var7)));
        let i_var11 = 0i32.wrapping_sub(s_var3 as i32);
        let local_4c = (i_var8 >> 0xf) as i16;

        let i_var8 = local_54 as i32;
        let i_var15 = local_5c as i32;
        let i_var9 = clamp(
            mul(DAT_00a0049c as i32, i_var15)
                .wrapping_add(mul(DAT_00a0048c as i32, i_var8))
                .wrapping_add(mul(DAT_00a00494, i_var11)),
        );
        let local_48 = (i_var9 >> 0xf) as i16;

        let i_var12 = local_50 as i32;
        let i_var13 = local_58 as i32;
        let i_var9 = clamp(
            mul(DAT_00a0049c as i32, i_var13)
                .wrapping_add(mul(DAT_00a0048c as i32, i_var12))
                .wrapping_add(mul(DAT_00a00494, i_var10)),
        );
        DAT_00a0049c = local_4c as i32 as u32;
        DAT_00a0048c = local_48 as i32 as u32;
        let local_44 = (i_var9 >> 0xf) as i16;
        DAT_00a00494 = local_44 as i32;

        let i_var9 = clamp(mul(DAT_00a004d0, i_var14).wrapping_add(mul(DAT_00a004ac, i_var7)));
        let local_40 = (i_var9 >> 0xf) as i16;

        let i_var9 = clamp(
            mul(DAT_00a004d0, i_var15)
                .wrapping_add(mul(DAT_00a004ac, i_var8))
                .wrapping_add(mul(DAT_00a004b4, i_var11)),
        );
        let local_3c = (i_var9 >> 0xf) as i16;

        let i_var9 = clamp(
            mul(DAT_00a004d0, i_var13)
                .wrapping_add(mul(DAT_00a004ac, i_var12))
                .wrapping_add(mul(DAT_00a004b4, i_var10)),
        );
        DAT_00a004d0 = local_40 as i32;
        DAT_00a004ac = local_3c as i32;
        let local_38 = (i_var9 >> 0xf) as i16;
        DAT_00a004b4 = local_38 as i32;

        let i_var7 =
            clamp(mul(DAT_00a07c68 as i32, i_var14).wrapping_add(mul(DAT_00a07c6c, i_var7)));
        let local_30 = (i_var7 >> 0xf) as i16;

        let i_var7 = clamp(
            mul(DAT_00a07c68 as i32, i_var15)
                .wrapping_add(mul(DAT_00a07c6c, i_var8))
                .wrapping_add(mul(DAT_00a07cc4, i_var11)),
        );
        let local_24 = (i_var7 >> 0xf) as i16;

        let i_var10 = clamp(
            mul(DAT_00a07c68 as i32, i_var13)
                .wrapping_add(mul(DAT_00a07c6c, i_var12))
                .wrapping_add(mul(DAT_00a07cc4, i_var10)),
        );
        DAT_00a07c68 = local_30 as i32 as u32;
        DAT_00a07c6c = local_24 as i32;
        let local_1c = (i_var10 >> 0xf) as i16;
        DAT_00a07cc4 = local_1c as i32;

        let u_var6 = FUN_0046abf0(DAT_00a0048c as i32, DAT_00a0049c as i32);
        DAT_009a1fe8
            .byte_add(0x10)
            .byte_offset(i_var2 as isize)
            .cast::<i16>()
            .write_unaligned(s_var4);
        let sa = 0u32.wrapping_sub(u_var6 as i16 as i32 as u32) as i16;
        DAT_009a1fe8
            .byte_add(0x14)
            .byte_offset(i_var2 as isize)
            .cast::<i16>()
            .write_unaligned(sa);
    }
}
