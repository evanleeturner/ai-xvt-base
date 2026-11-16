// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acec49: i32;
    static mut DAT_00acf149: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d1400() -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; the scan reads 4 bytes
    // at each 10-byte step from DAT_00acec49 up to DAT_00acf149, as the original does.
    unsafe {
        let mut p: *const u8 = (&raw mut DAT_00acec49).cast::<u8>().cast_const();
        let end: *const u8 = (&raw mut DAT_00acf149).cast::<u8>().cast_const();
        loop {
            let local_val = p.cast::<i32>().read_unaligned();
            if local_val == 1 {
                i_var1 = i_var1.wrapping_add(1);
            }
            p = p.byte_add(10);
            if !(p < end) {
                break;
            }
        }
    }
    i_var1
}
