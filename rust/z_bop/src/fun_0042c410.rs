// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a8078: u16;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ae080(param_1: i32) -> i64;
}

#[inline(always)]
unsafe fn rd32(a: u32) -> i32 {
    // SAFETY: the address is inside the game's own memory, read as the original does.
    unsafe { (a as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(a: u32) -> i16 {
    // SAFETY: the address is inside the game's own memory, read as the original does.
    unsafe { (a as *const i16).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042c410(param_1: u16) -> u32 {
    // SAFETY: all globals, callee and pointers belong to the game; reads
    // happen in the original's order.
    unsafe {
        let base = DAT_009a1fe8 as u32;
        let p1 = param_1 as u32;
        let mut s_var3: i16;
        if *((base.wrapping_add(3).wrapping_add(p1.wrapping_mul(0x23))) as *const u8) == 0x62 {
            let l = FUN_004ae080(0x62);
            s_var3 = 0u32.wrapping_sub((l as i16) as u16 as u32) as i16;
        } else {
            s_var3 = 0;
        }
        if param_1 == DAT_009a8078 {
            s_var3 = (s_var3 as i32).wrapping_sub(0x400) as i16;
        } else {
            s_var3 = (s_var3 as i32).wrapping_add(0x20) as i16;
        }
        let i_var9 = base.wrapping_add(p1.wrapping_mul(0x23));
        let k = s_var3 as i32;
        let p_obj = rd32(i_var9.wrapping_add(0x1f)) as u32;
        let s93 = rd16(p_obj.wrapping_add(0x93));
        let s95 = rd16(p_obj.wrapping_add(0x95));
        let s97 = rd16(p_obj.wrapping_add(0x97));
        let l12 = (k as i64).wrapping_mul(s93 as i64);
        let l1 = (k as i64).wrapping_mul(s95 as i64);
        let l2 = (k as i64).wrapping_mul(s97 as i64);
        let t4 = rd32(i_var9.wrapping_add(4));
        let t8 = rd32(i_var9.wrapping_add(8));
        let tc = rd32(i_var9.wrapping_add(0xc));
        let mut i_var5 =
            (t4 as u32).wrapping_add(((l12 >> 15) as i32 as u32).wrapping_mul(2)) as i32;
        let mut i_var10 =
            (t8 as u32).wrapping_add(((l1 >> 15) as i32 as u32).wrapping_mul(2)) as i32;
        let mut i_var11 =
            (tc as u32).wrapping_add(((l2 >> 15) as i32 as u32).wrapping_mul(2)) as i32;
        let arr = (&raw mut DAT_009e9670) as *mut u8 as u32;
        let idx = rd32(arr.wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)));
        let p_tgt = (idx as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32);
        let t4 = rd32(p_tgt.wrapping_add(4));
        let t8 = rd32(p_tgt.wrapping_add(8));
        let tc = rd32(p_tgt.wrapping_add(0xc));
        let i_var6 = (t4 as u32).wrapping_sub(i_var5 as u32) as i32;
        let i_var7 = (t8 as u32).wrapping_sub(i_var10 as u32) as i32;
        let i_var8 = (tc as u32).wrapping_sub(i_var11 as u32) as i32;
        if 0x4000 < i_var6 || i_var6 < -0x4000 {
            return 0;
        }
        if 0x4000 < i_var7 || i_var7 < -0x4000 {
            return 0;
        }
        if 0x4000 < i_var8 || i_var8 < -0x4000 {
            return 0;
        }
        let p_tgt_obj = rd32(p_tgt.wrapping_add(0x1f)) as u32;
        let tmp_i = rd32(p_tgt_obj.wrapping_add(6));
        i_var5 = (tmp_i as u32).wrapping_sub(i_var5 as u32) as i32;
        let tmp_i = rd32(p_tgt_obj.wrapping_add(10));
        i_var10 = (tmp_i as u32).wrapping_sub(i_var10 as u32) as i32;
        let tmp_j = rd32(p_tgt_obj.wrapping_add(0xe));
        i_var11 = (tmp_j as u32).wrapping_sub(i_var11 as u32) as i32;
        if i_var5 >= 0x4001 || i_var5 < -0x4000 {
            return 0;
        }
        if 0x4000 < i_var10 || i_var10 < -0x4000 {
            return 0;
        }
        if i_var11 >= 0x4001 || i_var11 < -0x4000 {
            return 0;
        }
        let mut i_var4 = ((i_var6 as i16 as i32).wrapping_mul(s93 as i32))
            .wrapping_add((i_var7 as i16 as i32).wrapping_mul(s95 as i32))
            .wrapping_add((i_var8 as i16 as i32).wrapping_mul(s97 as i32));
        if 0x3fffffff < i_var4 {
            i_var4 = 0x3fffffff;
        }
        if i_var4 < -0x3fffffff {
            i_var4 = -0x3fff0000;
        }
        let local_14 = (i_var4 >> 15) as i16;
        let local_10 = i_var10 as i16;
        let local_c = i_var5 as i16;
        let mut i_var5b = ((local_c as i32).wrapping_mul(s93 as i32))
            .wrapping_add((local_10 as i32).wrapping_mul(s95 as i32))
            .wrapping_add((i_var11 as i16 as i32).wrapping_mul(s97 as i32));
        if 0x3fffffff < i_var5b {
            i_var5b = 0x3fffffff;
        }
        if i_var5b < -0x3fffffff {
            i_var5b = -0x3fff0000;
        }
        let local_8 = (i_var5b >> 15) as i16;
        if (local_14 < 0 || 0 < local_8) && (0 < local_14 || local_8 < 0) {
            return 0;
        }
        1
    }
}
