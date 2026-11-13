// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73b6: i32;
    static mut DAT_00527760: i32;
    static mut DAT_009a73d0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00404620(param_1: i32) -> u32;
    fn FUN_004a4cb0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a3a50(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; every address
    // computed here points into the game's structures, read and written
    // exactly where the original does.
    unsafe {
        let idx = ((DAT_00a08104.wrapping_add(0x6a)) as *const u16).read_unaligned();
        let s_var1: i16;
        if idx == 0xffff {
            let v = (param_1
                .wrapping_mul(0x23)
                .wrapping_add(0x10)
                .wrapping_add(DAT_009a1fe8) as *const i16)
                .read_unaligned();
            s_var1 = v.wrapping_add(i16::MIN);
        } else {
            s_var1 = ((idx as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x10)
                .wrapping_add(DAT_009a1fe8 as u32) as *const i16)
                .read_unaligned();
        }
        (DAT_009a73b6.wrapping_add(0x32) as *mut i16).write_unaligned(s_var1);
        let u_var2 = FUN_00404620(DAT_00a08104);
        FUN_004a4cb0(((((u_var2 >> 1) as u16) & 0x7fff) as i32).wrapping_add(0x8000));
        let t = (&raw mut DAT_00527760)
            .cast::<u8>()
            .byte_add((DAT_009a73d0 as u32).wrapping_mul(2) as usize)
            .cast::<i16>()
            .read_unaligned();
        (DAT_009a73b6.wrapping_add(0x3a) as *mut i16).write_unaligned(t.wrapping_mul(0xec));
    }
}
