// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1c0: u32;
    static mut DAT_00743270: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045bbe0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6a91(param_1: *mut i32) -> u32 {
    // SAFETY: the globals and callee are the game's own; param_1 and the
    // table entries point at the caller's and the game's int records.
    unsafe {
        let count = DAT_0052f1c0;
        if count == 0 {
            return 0;
        }
        let mut local_8: u32 = 0;
        let mut local_10: u32 = 0;
        let mut local_18: *const i32 = (&raw mut DAT_00743270).cast::<i32>();
        let mut local_c: u32 = 0;
        while local_c < DAT_0052f1c0 {
            let mut local_14: u32 = 0;
            let p1 = param_1.add(1).read_unaligned();
            if p1 == 0 || p1 == local_18.add(1).read_unaligned() {
                local_14 = 1;
                let p2 = param_1.add(2).read_unaligned();
                if p2 == 0 || p2 == local_18.add(2).read_unaligned() {
                    local_14 = 2;
                    if (param_1.add(4).read_unaligned() & local_18.add(4).read_unaligned()) != 0 {
                        local_14 = 3;
                        if param_1.read_unaligned() == local_18.read_unaligned() {
                            FUN_0045bbe0();
                            return local_c;
                        }
                    }
                }
            }
            if local_8 < local_14 {
                local_10 = local_c;
                local_8 = local_14;
            }
            local_18 = local_18.add(0x5e);
            local_c = local_c.wrapping_add(1);
        }
        FUN_0045bbe0();
        local_10
    }
}
