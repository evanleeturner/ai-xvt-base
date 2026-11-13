// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f1e4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00475b70(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    if param_3 < 0 {
        return -1;
    }
    // SAFETY: the global and the memory behind the integer addresses are the
    // game's own; reads happen at the offsets and in the order the original uses.
    unsafe {
        let pf1 = param_2.wrapping_add(param_3.wrapping_mul(0xc)) as *const f32;
        DAT_0060f1e4 = DAT_0060f1e4.wrapping_sub(param_3 >> 1);
        if DAT_0060f1e4 < 0
            || ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() < DAT_0060f1e4
        {
            DAT_0060f1e4 = 0;
        }
        let mut pf2 = ((param_1.wrapping_add(0x14)) as *const i32)
            .read_unaligned()
            .wrapping_add(DAT_0060f1e4.wrapping_mul(0xc)) as *const f32;
        if DAT_0060f1e4 < ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
            loop {
                if pf2.read_unaligned() == pf1.read_unaligned()
                    && pf2.add(1).read_unaligned() == pf1.add(1).read_unaligned()
                    && pf1.add(2).read_unaligned() == pf2.add(2).read_unaligned()
                {
                    return DAT_0060f1e4;
                }
                pf2 = pf2.add(3);
                DAT_0060f1e4 = DAT_0060f1e4.wrapping_add(1);
                if DAT_0060f1e4 >= ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        DAT_0060f1e4 = 0;
        pf2 = ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned() as *const f32;
        if 0 < ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
            loop {
                if pf2.read_unaligned() == pf1.read_unaligned()
                    && pf2.add(1).read_unaligned() == pf1.add(1).read_unaligned()
                    && pf1.add(2).read_unaligned() == pf2.add(2).read_unaligned()
                {
                    return DAT_0060f1e4;
                }
                pf2 = pf2.add(3);
                DAT_0060f1e4 = DAT_0060f1e4.wrapping_add(1);
                if DAT_0060f1e4 >= ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        0
    }
}
