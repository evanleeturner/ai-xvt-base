// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0adf0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042de40() {
    // SAFETY: DAT_00a0adf0 is the start of the game's own array of 0x13a4
    // dwords; each one is zeroed in order.
    unsafe {
        let mut p = &raw mut DAT_00a0adf0;
        let mut i: i32 = 0x13a4;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
    }
}
