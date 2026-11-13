// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645778: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049bf90(param_1: *mut u32, param_2: i32, param_3: u32) {
    // SAFETY: the global is the game's own; param_1 and the destination
    // (param_2 + DAT_00645778) point at the caller's buffers, copied
    // forward dword-wise then byte-wise exactly as the original does.
    unsafe {
        let mut src = param_1;
        let mut dst = param_2.wrapping_add(DAT_00645778) as u32 as *mut u32;
        let mut count = param_3 >> 2;
        while count != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            count -= 1;
        }
        let mut src_b = src as *const u8;
        let mut dst_b = dst as *mut u8;
        let mut count = param_3 & 3;
        while count != 0 {
            dst_b.write(src_b.read());
            src_b = src_b.add(1);
            dst_b = dst_b.add(1);
            count -= 1;
        }
    }
}
