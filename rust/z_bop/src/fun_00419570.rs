// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00550880: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00419570(param_1: *mut u32, param_2: i32, param_3: u32) {
    // SAFETY: the global is the game's own; the source and destination
    // pointers are the caller's buffers, copied in the same order as the original.
    unsafe {
        let mut src = param_1 as *const u8;
        let mut dst = DAT_00550880.wrapping_add(param_2 as u32) as *mut u8;
        let mut n = param_3 >> 2;
        while n != 0 {
            let v = src.cast::<u32>().read_unaligned();
            dst.cast::<u32>().write_unaligned(v);
            src = src.byte_add(4);
            dst = dst.byte_add(4);
            n -= 1;
        }
        let mut n = param_3 & 3;
        while n != 0 {
            dst.write(src.read());
            src = src.byte_add(1);
            dst = dst.byte_add(1);
            n -= 1;
        }
    }
}
