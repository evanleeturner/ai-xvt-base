// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060269c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00489dd0(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    if param_3 < 0 {
        return -1;
    }
    // SAFETY: the caller supplies addresses of the game's own structures;
    // the global is the game's memory, read and written where the original does.
    unsafe {
        let pf1 = param_2.wrapping_add(param_3.wrapping_mul(0xc)) as *const f32;
        DAT_0060269c = DAT_0060269c.wrapping_sub(param_3 >> 1);
        if DAT_0060269c < 0
            || ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() < DAT_0060269c
        {
            DAT_0060269c = 0;
        }
        let base = ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned();
        let mut pf2 = base.wrapping_add(DAT_0060269c.wrapping_mul(0xc)) as *const f32;
        if DAT_0060269c < ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
            loop {
                if pf2.read_unaligned() == pf1.read_unaligned()
                    && pf1.add(1).read_unaligned() == pf2.add(1).read_unaligned()
                    && pf1.add(2).read_unaligned() == pf2.add(2).read_unaligned()
                {
                    return DAT_0060269c;
                }
                pf2 = pf2.add(3);
                DAT_0060269c = DAT_0060269c.wrapping_add(1);
                if DAT_0060269c >= ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        DAT_0060269c = 0;
        pf2 = ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned() as *const f32;
        if 0 < ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
            loop {
                if pf2.read_unaligned() == pf1.read_unaligned()
                    && pf1.add(1).read_unaligned() == pf2.add(1).read_unaligned()
                    && pf1.add(2).read_unaligned() == pf2.add(2).read_unaligned()
                {
                    return DAT_0060269c;
                }
                pf2 = pf2.add(3);
                DAT_0060269c = DAT_0060269c.wrapping_add(1);
                if DAT_0060269c >= ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        0
    }
}
