// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052361c: i32;
    static mut DAT_009fd398: i32;
    static mut DAT_009a8e34: i32;
    static mut DAT_00521f24: [u16; 1];
    static mut DAT_00527e90: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> i32;
    fn FUN_0040bbf0(param_1: u16, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_00421850(param_1: u16);
    fn FUN_00421930(param_1: i32);
    fn FUN_00421560(param_1: i16, param_2: i16, param_3: i32, param_4: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00420110(param_1: i32, param_2: i32, param_3: i32, param_4: u16) {
    // SAFETY: globals and callees are the game's own; the pointer reads
    // follow the original's addressing of the table and the returned object.
    unsafe {
        DAT_0052361c = 1;
        DAT_009fd398 = 0x100000;
        DAT_009a8e34 = 0x7fffffff;
        let idx = param_1.wrapping_mul(0xd) as isize;
        let base = (&raw mut DAT_00521f24).cast::<u16>();
        let lo = base.wrapping_offset(idx).read_unaligned() as u32;
        let hi = (((param_1 as u32) >> 16) as u16 as u32) << 16;
        let mut i_var1 = FUN_004ac720((hi | lo) as i32);
        FUN_004ac740();
        let off = ((i_var1 as u32).wrapping_add(0x10) as usize as *const i32).read_unaligned();
        let v = ((off as u32).wrapping_add(i_var1 as u32) as usize as *const i32).read_unaligned();
        i_var1 = i_var1.wrapping_add(v);
        if DAT_00527e90 != 0 {
            FUN_0040bbf0(param_4, param_2, param_3, 0x100, i_var1);
            return;
        }
        FUN_00421850(param_4);
        FUN_00421930(i_var1);
        FUN_00421560(param_2 as i16, param_3 as i16, 0x100, i_var1);
    }
}
