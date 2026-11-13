// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bb1b: i32;
    static mut DAT_0071bb1f: i32;
    static mut DAT_0071bb23: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_0073255c: i32;
    static mut DAT_00732584: i32;
    static mut DAT_0073253c: i32;
    static mut DAT_00732568: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732548: i32;
    static mut DAT_00732558: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732538: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421c70(param_1: u32, param_2: i32);
    fn FUN_00413d00(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_00419100(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040a640(param_1: u32, param_2: *mut i32) -> i32 {
    if param_1 == 0xffff_ffff {
        return 0x40;
    }
    // SAFETY: all globals, callees and pointers belong to the game's own
    // memory; reads and writes happen where the original does them.
    unsafe {
        let addr = param_1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32);
        let i_var1 = (addr as usize as *const i32).read_unaligned();
        let off = (DAT_0071ecf4 as usize).wrapping_mul(0x5bd);
        let a1b = ((&raw mut DAT_0071bb1b) as usize).wrapping_add(off) as *const i32;
        let a1f = ((&raw mut DAT_0071bb1f) as usize).wrapping_add(off) as *const i32;
        let a23 = ((&raw mut DAT_0071bb23) as usize).wrapping_add(off) as *const i32;
        let mut s_var4: i16;
        let mut s_var5: i16;
        let mut s_var8: i16;
        if i_var1 == 0 {
            FUN_00421c70(param_1, 0);
            s_var5 = (DAT_00732a28 as i16).wrapping_sub(a1f.read_unaligned() as i16);
            s_var8 = (DAT_00732a2c as i16).wrapping_sub(a1b.read_unaligned() as i16);
            s_var4 = (DAT_00732a24 as i16).wrapping_sub(a23.read_unaligned() as i16);
        } else {
            let b = i_var1 as u32 as usize;
            s_var8 = ((b.wrapping_add(6)) as *const i32).read_unaligned() as i16;
            s_var8 = s_var8.wrapping_sub(a1b.read_unaligned() as i16);
            s_var5 = ((b.wrapping_add(10)) as *const i32).read_unaligned() as i16;
            s_var5 = s_var5.wrapping_sub(a1f.read_unaligned() as i16);
            s_var4 = ((b.wrapping_add(0xe)) as *const i32).read_unaligned() as i16;
            s_var4 = s_var4.wrapping_sub(a23.read_unaligned() as i16);
        }
        let i_var7 = s_var4 as i32;
        let i_var6 = s_var5 as i32;
        let i_var9 = s_var8 as i32;
        let i_var1 = FUN_00413d00(
            i_var9,
            i_var6,
            i_var7,
            DAT_0073255c,
            DAT_00732584,
            DAT_0073253c,
        );
        let i_var2 = FUN_00413d00(
            i_var9,
            i_var6,
            i_var7,
            DAT_00732568,
            DAT_00732530,
            DAT_00732548,
        );
        let u_var3 = FUN_00419100(i_var1 as i16 as i32, i_var2 as i16 as i32);
        s_var4 = u_var3 as i16;
        if 0x3fff < s_var4 || s_var4 < -0x3fff {
            let i_var1 = FUN_00413d00(
                i_var9,
                i_var6,
                i_var7,
                DAT_00732558,
                DAT_00732578,
                DAT_00732538,
            );
            let u_var3 = FUN_00419100(i_var1 as i16 as i32, i_var2 as i16 as i32);
            s_var5 = i16::MIN.wrapping_sub(u_var3 as i16);
            s_var4 = i16::MIN.wrapping_sub(s_var4);
            if s_var5 < 0 {
                s_var5 = s_var5.wrapping_neg();
            }
            s_var8 = s_var4;
            if s_var4 < 0 {
                s_var8 = s_var8.wrapping_neg();
            }
            let x = (0x4000i16.wrapping_sub(s_var5)) >> 8;
            let y = (0x4000i16.wrapping_sub(s_var8)) >> 8;
            let mut i_var1 = (x as i32).wrapping_mul(y as i32) as i16 as i32;
            let t = (i_var1.wrapping_add((i_var1 >> 31) & 0x3f) >> 6) as i16;
            i_var1 = ((param_2.read() as i16 as i32).wrapping_mul(t as i32)) as i16 as i32;
            let r = (i_var1.wrapping_add((i_var1 >> 31) & 0x7f) >> 7) as i16 as i32;
            param_2.write(param_2.read().wrapping_sub(r));
        }
        s_var4 >>= 7;
        if s_var4 < -0x40 {
            s_var4 = -0x40;
        }
        if 0x3f < s_var4 {
            s_var4 = 0x3f;
        }
        s_var4.wrapping_add(0x40) as i32
    }
}
