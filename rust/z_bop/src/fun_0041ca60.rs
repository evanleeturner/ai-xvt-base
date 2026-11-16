// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08298: i32;
    static mut DAT_00a081e8: i32;
    static mut DAT_00a07c58: i32;
    static mut DAT_00a07c54: i32;
    static mut DAT_00a07bdc: i32;
    static mut DAT_009ec604: i32;
    static mut DAT_009d8c18: i32;
    static mut DAT_009d80c0: i32;
    static mut DAT_009d77fc: i32;
    static mut DAT_009d6828: i32;
    static mut DAT_009d1150: i32;
    static mut DAT_009cd260: i32;
    static mut DAT_009a1fd8: i32;
    static mut DAT_009a1fd4: i32;
    static mut DAT_009a1fd0: i32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_0051bf58: i32;
    static mut DAT_0051c59e: u8;
    static mut DAT_009e9697: u8;
    static mut DAT_0051c5b2: u8;
    static mut DAT_0051c5b4: u8;
    static mut DAT_0051c5b6: u8;
    static mut s_starshipprotectpln_0051a2fc: u8;
    static mut s_capescortersldr1pln_0051a334: u8;
    static mut s_waitforboardpln_0051a358: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425b70(a: u32, b: u16) -> u32;
    fn FUN_004035d0(p: *mut core::ffi::c_void, a: i16, b: i16, c: i16);
    fn FUN_00427bd0(a: i16, b: i16, p: *mut core::ffi::c_void);
    fn FUN_004ae640(a: u32, b: u32) -> i32;
    fn FUN_004ae6e0(a: u32, b: u32) -> i32;
    fn FUN_004ae5a0(a: u32, b: u32) -> i32;
    fn FUN_004037b0(p: *mut core::ffi::c_void, a: i32, b: i32, c: i32);
    fn FUN_0041c1d0(a: u16, b: u16) -> u32;
    fn FUN_00446700(a: u16, b: u16) -> u32;
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: the address is game memory read where the original reads it.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn r16(a: u32) -> u16 {
    // SAFETY: the address is game memory read where the original reads it.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn r32(a: u32) -> u32 {
    // SAFETY: the address is game memory read where the original reads it.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041ca60(param_1: u16, param_2: u16, param_3: u32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers are the
    // game's records, read and written where the original does.
    unsafe {
        let u_var19 = DAT_00a08298;
        let u_var18 = DAT_00a081e8;
        let u_var17 = DAT_00a07c58;
        let u_var16 = DAT_00a07c54;
        let u_var15 = DAT_00a07bdc;
        let u_var14 = DAT_009ec604;
        let u_var13 = DAT_009d8c18;
        let u_var12 = DAT_009d80c0;
        let u_var11 = DAT_009d77fc;
        let u_var10 = DAT_009d6828;
        let u_var9 = DAT_009d1150;
        let u_var8 = DAT_009cd260;
        let u_var7 = DAT_009a1fd8;
        let u_var6 = DAT_009a1fd4;
        let u_var5 = DAT_009a1fd0;

        let tbl = (&raw mut DAT_009e9697) as usize as u32;
        let tbl_off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd);

        let i_var25: u32 = (param_1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);
        let mut p_a = r32(i_var25.wrapping_add(0x1f));
        let p_b = r32(p_a.wrapping_add(0xa9));
        let mut i_var21: i32 = (rb(p_b.wrapping_add(4)) as u32).wrapping_mul(0x101) as i32;
        let u_var26: u32 = param_3 & 0xffff;
        let t59e = (&raw mut DAT_0051c59e) as usize as u32;
        let idx = (i_var21 as u32).wrapping_add(rb(tbl.wrapping_add(tbl_off)) as u32);
        let mut u_var20: u32 = rb(t59e.wrapping_add(idx)) as u32;
        if (0x3fi8)
            < (rb(p_b
                .wrapping_add(0x371)
                .wrapping_add(u_var26.wrapping_mul(4))) as i8)
        {
            let b = rb(t59e.wrapping_add(idx));
            u_var20 = b.wrapping_add(1) as u16 as u32;
            // (byte + 1 computed as an int, then truncated to 16 bits)
            u_var20 = (b as u32).wrapping_add(1) & 0xffff;
        }
        let u_var2 = r16(((&raw mut s_starshipprotectpln_0051a2fc) as usize as u32)
            .wrapping_add(u_var20.wrapping_mul(2))
            .wrapping_add(10));
        let u_var3 = r16(((&raw mut s_capescortersldr1pln_0051a334) as usize as u32)
            .wrapping_add(u_var20.wrapping_mul(2))
            .wrapping_add(2));
        let w = r16(((&raw mut s_waitforboardpln_0051a358) as usize as u32)
            .wrapping_add(u_var20.wrapping_mul(2))
            .wrapping_add(0xe));
        u_var20 = FUN_00425b70(0xec, w);
        DAT_009ec604 = r32(i_var25.wrapping_add(4)) as i32;
        let mut i_var24: i32 =
            (u_var20 & 0xffff).wrapping_add((u_var3 as u32).wrapping_mul(0xec)) as i32;
        DAT_00a081e8 = r32(i_var25.wrapping_add(8)) as i32;
        DAT_00a08298 = r32(i_var25.wrapping_add(0xc)) as i32;
        let off = u_var26.wrapping_mul(8).wrapping_add(i_var21 as u32);
        FUN_004035d0(
            i_var25 as usize as *mut core::ffi::c_void,
            r16(((&raw mut DAT_0051c5b2) as usize as u32).wrapping_add(off)) as i16,
            r16(((&raw mut DAT_0051c5b4) as usize as u32).wrapping_add(off)) as i16,
            r16(((&raw mut DAT_0051c5b6) as usize as u32).wrapping_add(off)) as i16,
        );
        if rb(i_var25.wrapping_add(3)) as i8 == 0x35 {
            DAT_009d8a58 = DAT_009d8a58.wrapping_mul(2);
            DAT_009d8a54 = DAT_009d8a54.wrapping_mul(2);
            DAT_009d8b60 = DAT_009d8b60.wrapping_mul(2);
        }
        DAT_009ec604 = DAT_009ec604.wrapping_add(DAT_009d8a58);
        DAT_00a081e8 = DAT_00a081e8.wrapping_add(DAT_009d8a54);
        DAT_00a08298 = DAT_00a08298.wrapping_add(DAT_009d8b60);

        p_a = r32(i_var25.wrapping_add(0x1f));
        let us79 = r16(p_a.wrapping_add(0x79));
        let tmp: i32 = (us79 as u32)
            .wrapping_add(u_var2 as u32)
            .wrapping_mul(0x1234)
            .wrapping_add(0x80) as i32;
        i_var21 = ((tmp >> 8) as u32).wrapping_mul(i_var24 as u32) as i32;
        i_var21 /= 0xec;

        p_a = r32(i_var25.wrapping_add(0x1f));
        if rb(p_a.wrapping_add(0x8b)) != 0 {
            FUN_00427bd0(
                r16(i_var25.wrapping_add(0x12)) as i16,
                r16(i_var25.wrapping_add(0x10)) as i16,
                i_var25 as usize as *mut core::ffi::c_void,
            );
        }

        p_a = r32(i_var25.wrapping_add(0x1f));
        let l = (r16(p_a.wrapping_add(0x8c)) as i16 as i64).wrapping_mul(i_var21 as i64);
        DAT_009a1fd4 = ((l >> 15) as u32).wrapping_add(DAT_009ec604 as u32) as i32;
        p_a = r32(i_var25.wrapping_add(0x1f));
        let l = (r16(p_a.wrapping_add(0x8e)) as i16 as i64).wrapping_mul(i_var21 as i64);
        DAT_009a1fd8 = (DAT_00a081e8 as u32).wrapping_add((l >> 15) as u32) as i32;
        p_a = r32(i_var25.wrapping_add(0x1f));
        let l = (r16(p_a.wrapping_add(0x90)) as i16 as i64).wrapping_mul(i_var21 as i64);
        DAT_009a1fd0 = (DAT_00a08298 as u32).wrapping_add((l >> 15) as u32) as i32;

        let i_var25: u32 = (param_2 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);
        p_a = r32(i_var25.wrapping_add(0x1f));
        if p_a == 0 {
            DAT_0051bf58 = 1;
            u_var20 = FUN_00446700(param_1, param_2);
            u_var20 = u_var20 as i16 as i32 as u32;
        } else {
            DAT_009d80c0 = r32(i_var25.wrapping_add(4)) as i32;
            DAT_009d8c18 = r32(i_var25.wrapping_add(8)) as i32;
            DAT_009d1150 = r32(i_var25.wrapping_add(0xc)) as i32;
            let u_var2 = r16(tbl.wrapping_add(tbl_off).wrapping_add(2));
            if u_var2 != 0 {
                let u_var26 = u_var2 as u32;
                let u_var20b = rb(i_var25.wrapping_add(3)) as u32;
                let i_var22 = FUN_004ae640(u_var20b, u_var26);
                let i_var22 = i_var22.wrapping_neg();
                let i_var23 = FUN_004ae6e0(u_var20b, u_var26);
                let xv = FUN_004ae5a0(u_var20b, u_var26);
                FUN_004037b0(
                    i_var25 as usize as *mut core::ffi::c_void,
                    xv,
                    i_var23,
                    i_var22,
                );
                DAT_009d80c0 = DAT_009d80c0.wrapping_add(DAT_009d8a58);
                DAT_009d8c18 = DAT_009d8c18.wrapping_add(DAT_009d8a54);
                DAT_009d1150 = DAT_009d1150.wrapping_add(DAT_009d8b60);
            }
            p_a = r32(i_var25.wrapping_add(0x1f));
            let us79 = r16(p_a.wrapping_add(0x79));
            let tmp: i32 = (us79 as u32).wrapping_mul(0x1234).wrapping_add(0x80) as i32;
            i_var24 = ((tmp >> 8) as u32).wrapping_mul(i_var24 as u32) as i32;
            i_var24 /= 0xec;

            p_a = r32(i_var25.wrapping_add(0x1f));
            if rb(p_a.wrapping_add(0x8b)) != 0 {
                FUN_00427bd0(
                    r16(i_var25.wrapping_add(0x12)) as i16,
                    r16(i_var25.wrapping_add(0x10)) as i16,
                    i_var25 as usize as *mut core::ffi::c_void,
                );
            }

            p_a = r32(i_var25.wrapping_add(0x1f));
            let l = (r16(p_a.wrapping_add(0x8c)) as i16 as i64).wrapping_mul(i_var24 as i64);
            DAT_00a07bdc = (DAT_009d80c0 as u32).wrapping_add((l >> 15) as u32) as i32;
            p_a = r32(i_var25.wrapping_add(0x1f));
            let l = (r16(p_a.wrapping_add(0x8e)) as i16 as i64).wrapping_mul(i_var24 as i64);
            DAT_00a07c54 = ((l >> 15) as u32).wrapping_add(DAT_009d8c18 as u32) as i32;
            p_a = r32(i_var25.wrapping_add(0x1f));
            let l = (r16(p_a.wrapping_add(0x90)) as i16 as i64).wrapping_mul(i_var24 as i64);
            DAT_00a07c58 = ((l >> 15) as u32).wrapping_add(DAT_009d1150 as u32) as i32;
            DAT_0051bf58 = 1;
            u_var20 = FUN_0041c1d0(param_1, param_2);
            u_var20 &= 0xffff;
        }
        DAT_009ec604 = u_var14;
        DAT_00a081e8 = u_var18;
        DAT_00a08298 = u_var19;
        DAT_009a1fd4 = u_var6;
        DAT_009a1fd8 = u_var7;
        DAT_009a1fd0 = u_var5;
        DAT_009d80c0 = u_var12;
        DAT_009d8c18 = u_var13;
        DAT_009d1150 = u_var9;
        DAT_00a07bdc = u_var15;
        DAT_00a07c54 = u_var16;
        DAT_00a07c58 = u_var17;
        DAT_009d77fc = u_var11;
        DAT_009d6828 = u_var10;
        DAT_009cd260 = u_var8;
        DAT_0051bf58 = 0;
        u_var20
    }
}
