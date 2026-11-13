// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071e522: i16;
    static mut _DAT_0074110c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004062e0(param_1: *mut i32, param_2: i32, param_3: i32) {
    let mut param_1 = param_1;
    // SAFETY: all pointers are the game's own memory, accessed at the same
    // offsets and in the same order as the original.
    unsafe {
        loop {
            let i_var1 = param_1.byte_add(4).read_unaligned();
            let i_var2 = param_1.read_unaligned();
            let mut i_var3 = param_1.byte_add(8).read_unaligned();
            param_1 = param_1.byte_add(12);
            let mut pi_var4 = ((DAT_0071e522 as i32)
                .wrapping_add(i_var2)
                .wrapping_mul(4)
                .wrapping_add(param_3)) as *const i32;
            loop {
                let v = pi_var4.read_unaligned();
                pi_var4 = pi_var4.byte_add(4);
                ((param_2.wrapping_add(v)) as *mut u8).write(i_var1 as u8);
                i_var3 = i_var3.wrapping_sub(1);
                if i_var3 == 0 {
                    break;
                }
            }
            _DAT_0074110c = _DAT_0074110c.wrapping_sub(1);
            if _DAT_0074110c == 0 {
                break;
            }
        }
    }
}
