// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00650424: i16;
    static mut DAT_00650426: i16;
    static mut DAT_0065042c: i16;
    static mut DAT_0065042e: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f33c0(
    param_1: *mut i32,
    param_2: i16,
    param_3: i16,
    param_4: *mut i16,
    param_5: *mut i16,
) {
    // SAFETY: the globals are the game's own; the pointers are the caller's
    // rect and output slots, read and written where the original does.
    unsafe {
        let p0 = param_1.read_unaligned();
        let p1 = param_1.add(1).read_unaligned();
        let p2 = param_1.add(2).read_unaligned();
        let p3 = param_1.add(3).read_unaligned();

        let mut i_var2 = (param_2 as i32)
            .wrapping_sub(DAT_00650424 as i32)
            .wrapping_mul(DAT_0065042c as i32);
        let mut s_var1 = (i_var2.wrapping_add((i_var2 >> 31) & 0xff) >> 8) as i16;
        param_4.write_unaligned(s_var1);
        let half = (p2.wrapping_sub(p0) >> 1) as i16 as u16 as u32;
        let sum = half
            .wrapping_add(s_var1 as u16 as u32)
            .wrapping_add(p0 as i16 as u16 as u32);
        param_4.write_unaligned(sum as i16);

        i_var2 = (param_3 as i32)
            .wrapping_sub(DAT_00650426 as i32)
            .wrapping_mul(DAT_0065042e as i32);
        s_var1 = (i_var2.wrapping_add((i_var2 >> 31) & 0xff) >> 8) as i16;
        param_5.write_unaligned(s_var1);
        let half = (p3.wrapping_sub(p1) >> 1) as i16 as u16 as u32;
        let sum = half
            .wrapping_add(p1 as i16 as u16 as u32)
            .wrapping_add(s_var1 as u16 as u32);
        param_5.write_unaligned(sum as i16);
    }
}
