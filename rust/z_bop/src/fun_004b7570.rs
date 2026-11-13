// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664c88: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6e30(param_1: u8, param_2: u8, param_3: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7570(param_1: *mut u8, param_2: i32) -> u32 {
    // SAFETY: the global table and callee are the game's own; param_1 points
    // at the caller's bytes, read only when the original reads them.
    unsafe {
        let base = (&raw mut DAT_00664c88) as usize as u32;
        let psvar1 = base.wrapping_add(param_2.wrapping_mul(2) as u32) as usize as *mut i16;
        let svar2 = psvar1.read_unaligned();
        let mut uvar3 = svar2 as u16 as u32;
        if 0xff < svar2 {
            if param_2 == 0 {
                let a = param_1.read();
                let b = param_1.byte_add(1).read();
                let c = param_1.byte_add(2).read();
                uvar3 = FUN_004d6e30(a, b, c);
                psvar1.write_unaligned(uvar3 as i16);
            } else {
                return uvar3 & 0xffffff00;
            }
        }
        uvar3
    }
}
