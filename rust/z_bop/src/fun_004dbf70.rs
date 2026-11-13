// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aabd11: [u32; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dbf70(param_1: i32) -> u8 {
    if param_1 >= 0 && param_1 < 0x100 {
        // SAFETY: the table is the game's own memory; the index is checked
        // to be within 0..0x100, and the entry is read unaligned as the
        // table sits at an odd address.
        unsafe {
            let ptr = (&raw mut DAT_00aabd11)
                .cast::<u32>()
                .add(param_1 as usize)
                .read_unaligned();
            if ptr != 0 {
                return ((ptr.wrapping_add(0x404)) as *const u8).read();
            }
        }
    }
    0
}
