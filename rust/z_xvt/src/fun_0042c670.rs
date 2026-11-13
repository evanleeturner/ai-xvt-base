// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081f7de: i32;
    static mut DAT_0071bcb8: i32;
    static mut DAT_0081fa9e: i32;
    static mut DAT_0081f812: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042c670(param_1: i32) {
    // SAFETY: the globals are the game's own memory; all reads and writes
    // happen at the same addresses and in the same order as the original.
    unsafe {
        let mut p1: *const u8 = (&raw mut DAT_0081f7de).cast::<u8>();
        let end: *const u8 = (&raw mut DAT_0081fa9e).cast::<u8>();
        let mut i_var2: i32 = 0;
        'found: {
            loop {
                let local_val = p1.cast::<i32>().read_unaligned();
                let target_val = (&raw mut DAT_0071bcb8)
                    .cast::<u8>()
                    .byte_offset(param_1.wrapping_mul(0x5bd) as isize)
                    .cast::<i32>()
                    .read_unaligned();
                if local_val == target_val {
                    break 'found;
                }
                p1 = p1.byte_add(0x58);
                i_var2 = i_var2.wrapping_add(1);
                if !((p1 as usize) < (end as usize)) {
                    break;
                }
            }
            i_var2 = 0;
        }
        (&raw mut DAT_0081f812)
            .cast::<u8>()
            .byte_offset(i_var2.wrapping_mul(0x58) as isize)
            .cast::<i32>()
            .write_unaligned(1);
    }
}
