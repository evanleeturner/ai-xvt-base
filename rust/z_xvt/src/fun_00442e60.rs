// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_0071e534: u16;
    static mut DAT_00732550: u32;
    static mut DAT_00732540: u32;
    static mut DAT_00732544: i32;
    static mut DAT_00732580: i32;
    static mut DAT_00732560: i32;
    static mut DAT_00732564: i32;
    static mut DAT_007329c4: i32;
    static mut DAT_007329c8: i32;
    static mut DAT_00732a14: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00483540(a: i16, b: i16, c: i32);
    fn FUN_00483cf0(a: i32, b: i32, c: i32, d: i16);
    fn FUN_004184f0(a: i16) -> i16;
    fn FUN_00419100(a: i32, b: i32) -> i32;
    fn FUN_004187b0(a: i16) -> i32;
    fn FUN_00418450(a: i16) -> i32;
    fn FUN_00413ca0(a: i32, b: i32) -> i32;
    fn FUN_00413d00(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00442e60(param_1: i16, param_2: i16, param_3: u32, param_4: i32) {
    // SAFETY: all globals, callees and pointers belong to the game; the
    // record table and the inner record are read and written at the same
    // offsets as the original.
    unsafe {
        let i_var2: u32 = (param_3 & 0xffff).wrapping_mul(0x23);
        let i_var11: *mut u8 = DAT_006d4988.byte_add(i_var2 as usize);
        let mut p_inner: *mut u8 = i_var11.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
        if p_inner.byte_add(0x90).read() != 0 {
            let s1: i16 = i_var11.byte_add(0x12).cast::<i16>().read_unaligned();
            let s0: i16 = i_var11.byte_add(0x10).cast::<i16>().read_unaligned();
            FUN_00483410(s1, s0, i_var11 as usize as i32);
            let base = DAT_006d4988.byte_add(i_var2 as usize);
            let s2: i16 = base.byte_add(0x14).cast::<i16>().read_unaligned();
            FUN_00483540(s2, 0, base as usize as i32);
        }

        p_inner = DAT_006d4988
            .byte_add(0x1f)
            .byte_add(i_var2 as usize)
            .cast::<*mut u8>()
            .read_unaligned();

        let mut s0: i16 = p_inner.byte_add(0x91).cast::<i16>().read_unaligned();
        DAT_007329c4 = (s0 as i32).wrapping_neg();
        s0 = p_inner.byte_add(0x93).cast::<i16>().read_unaligned();
        DAT_007329c8 = (s0 as i32).wrapping_neg();
        s0 = p_inner.byte_add(0x95).cast::<i16>().read_unaligned();
        DAT_00732a14 = (s0 as i32).wrapping_neg();
        s0 = p_inner.byte_add(0x9d).cast::<i16>().read_unaligned();
        DAT_00732580 = s0 as i32;
        s0 = p_inner.byte_add(0x9f).cast::<i16>().read_unaligned();
        DAT_00732560 = s0 as i32;
        s0 = p_inner.byte_add(0xa1).cast::<i16>().read_unaligned();
        DAT_00732564 = s0 as i32;
        s0 = p_inner.byte_add(0x97).cast::<i16>().read_unaligned();
        DAT_00732550 = (s0 as i32) as u32;
        s0 = p_inner.byte_add(0x99).cast::<i16>().read_unaligned();
        DAT_00732540 = (s0 as i32) as u32;
        s0 = p_inner.byte_add(0x9b).cast::<i16>().read_unaligned();
        DAT_00732544 = s0 as i32;

        FUN_00483cf0(
            DAT_00732550 as i32,
            DAT_00732540 as i32,
            DAT_00732544,
            param_1,
        );
        if ((DAT_0071e534 as u8) & 0xe) != 2 {
            FUN_00483cf0(DAT_00732580, DAT_00732560, DAT_00732564, param_2);
        }

        let mut s_var3: i16 = FUN_004184f0(((DAT_00732a14 as i16) as i32).wrapping_neg() as i16);
        (param_4 as u32 as usize as *mut u8)
            .byte_add(0xd)
            .cast::<i16>()
            .write_unaligned(s_var3);

        let u_var6 = FUN_00419100(DAT_007329c4, 0u32.wrapping_sub(DAT_007329c8 as u32) as i32);
        let s_var4: i16 = ((u_var6 as i16) as i32).wrapping_neg() as i16;

        let l_var18 = FUN_004187b0(s_var4);
        let l_var19 = FUN_00418450(s_var4);
        let s_var5: i16 = l_var19 as i16;
        let l_var19 = FUN_004187b0(s_var3);
        let l_var20 = FUN_00418450(s_var3);
        s_var3 = l_var20 as i16;

        let i_var_a: i32 = (l_var19 as i16) as i32;
        let u_var21 = FUN_00413ca0(s_var5 as i32, i_var_a);
        let u_var22 = FUN_00413ca0(s_var5 as i32, s_var3 as i32);
        let i_var14: i32 = (l_var18 as i16) as i32;
        let u_var23 = FUN_00413ca0(i_var14, i_var_a);
        let u_var24 = FUN_00413ca0(i_var14, s_var3 as i32);

        let i_var7: i32 = (s_var5 as i32).wrapping_neg();
        let mut i_var8 = FUN_00413d00(
            DAT_00732550 as i32,
            DAT_00732540 as i32,
            DAT_00732544,
            i_var14,
            i_var7,
            0,
        );
        let i_var12: i32 = (s_var3 as i32).wrapping_neg();
        let i_var13: i32 = (u_var23 as i16) as i32;
        let i_var16: i32 = (u_var21 as i16) as i32;
        let mut i_var9 = FUN_00413d00(
            DAT_00732550 as i32,
            DAT_00732540 as i32,
            DAT_00732544,
            i_var16,
            i_var13,
            i_var12,
        );
        let i_var15: i32 = (u_var24 as i16) as i32;
        let i_var17: i32 = (u_var22 as i16) as i32;
        let mut i_var10 = FUN_00413d00(
            DAT_00732550 as i32,
            DAT_00732540 as i32,
            DAT_00732544,
            i_var17,
            i_var15,
            i_var_a,
        );
        DAT_00732550 = ((i_var8 as i16) as i32) as u32;
        DAT_00732540 = ((i_var9 as i16) as i32) as u32;
        DAT_00732544 = (i_var10 as i16) as i32;

        i_var8 = FUN_00413d00(DAT_00732580, DAT_00732560, DAT_00732564, i_var14, i_var7, 0);
        i_var9 = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            i_var16,
            i_var13,
            i_var12,
        );
        i_var10 = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            i_var17,
            i_var15,
            i_var_a,
        );
        DAT_00732580 = (i_var8 as i16) as i32;
        DAT_00732560 = (i_var9 as i16) as i32;
        DAT_00732564 = (i_var10 as i16) as i32;

        let r7 = FUN_00413d00(DAT_007329c4, DAT_007329c8, DAT_00732a14, i_var14, i_var7, 0);
        let r8 = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            i_var16,
            i_var13,
            i_var12,
        );
        let ra = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            i_var17,
            i_var15,
            i_var_a,
        );
        DAT_007329c4 = (r7 as i16) as i32;
        DAT_007329c8 = (r8 as i16) as i32;
        DAT_00732a14 = (ra as i16) as i32;

        let u_var6 = FUN_00419100(DAT_00732540 as i32, DAT_00732550 as i32);
        DAT_006d4988
            .byte_add(0x10)
            .byte_add(i_var2 as usize)
            .cast::<i16>()
            .write_unaligned(s_var4);
        let s_out: i16 = ((u_var6 as i16) as i32).wrapping_neg() as i16;
        DAT_006d4988
            .byte_add(0x14)
            .byte_add(i_var2 as usize)
            .cast::<i16>()
            .write_unaligned(s_out);
    }
}
