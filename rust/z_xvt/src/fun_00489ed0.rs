// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006026b4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00489ed0(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    if param_3 < 0 {
        return -1;
    }
    // SAFETY: the global and the memory behind the integer addresses are the
    // game's own; reads happen in the same order as the original.
    unsafe {
        let pf1 = param_2.wrapping_add(param_3.wrapping_mul(8)) as *const f32;
        DAT_006026b4 = DAT_006026b4.wrapping_sub(param_3 >> 1);
        let count = |p: i32| ((p.wrapping_add(0x10)) as *const i32).read_unaligned();
        if DAT_006026b4 < 0 || count(param_1) < DAT_006026b4 {
            DAT_006026b4 = 0;
        }
        let base = ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned();
        let mut pf2 = base.wrapping_add(DAT_006026b4.wrapping_mul(8)) as *const f32;
        if DAT_006026b4 < count(param_1) {
            loop {
                if pf2.read_unaligned() == pf1.read_unaligned()
                    && pf2.add(1).read_unaligned() == pf1.add(1).read_unaligned()
                {
                    return DAT_006026b4;
                }
                pf2 = pf2.add(2);
                DAT_006026b4 = DAT_006026b4.wrapping_add(1);
                if !(DAT_006026b4 < count(param_1)) {
                    break;
                }
            }
        }
        DAT_006026b4 = 0;
        pf2 = ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned() as *const f32;
        if 0 < count(param_1) {
            loop {
                if pf2.read_unaligned() == pf1.read_unaligned()
                    && pf2.add(1).read_unaligned() == pf1.add(1).read_unaligned()
                {
                    return DAT_006026b4;
                }
                pf2 = pf2.add(2);
                DAT_006026b4 = DAT_006026b4.wrapping_add(1);
                if !(DAT_006026b4 < count(param_1)) {
                    break;
                }
            }
        }
        0
    }
}
