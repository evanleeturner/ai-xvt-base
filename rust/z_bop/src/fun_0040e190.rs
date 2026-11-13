// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005505e0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e190(param_1: i32) -> u32 {
    // SAFETY: the global is the game's own; the computed address points at
    // the game's table, read where the original does.
    unsafe {
        let addr = DAT_005505e0.wrapping_add(param_1.wrapping_mul(4));
        (addr as usize as *const u32).read_unaligned()
    }
}
