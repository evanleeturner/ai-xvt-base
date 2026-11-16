// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: i32;
    static mut DAT_009a1fe8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00481d90(param_1: i16, param_2: i32) {
    // SAFETY: the globals and every address derived from them belong to the
    // game's own memory; the loads and stores happen in the original's order.
    unsafe {
        let base = (&raw mut DAT_009e9670) as usize as i32;
        let a1 = base.wrapping_add(param_2.wrapping_mul(0x5bd));
        let v1 = (a1 as usize as *const i32).read_unaligned();
        let a2 = v1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8);
        let v2 = (a2 as usize as *const i32).read_unaligned();
        let a3 = v2.wrapping_add(0xa9);
        let i_var2 = (a3 as usize as *const i32).read_unaligned();
        let p = i_var2.wrapping_add(0xbd) as usize as *mut u16;
        let u_var1 = p.read_unaligned();
        let new_val = (param_1 as i32).wrapping_add(u_var1 as i32) as u16;
        p.write_unaligned(new_val);
        if new_val < u_var1 {
            p.write_unaligned(0xffff);
        }
    }
}
