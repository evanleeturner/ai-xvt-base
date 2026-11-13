// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_00a08292: u16;
    static mut DAT_0051c560: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8c30: i32;
    static mut DAT_00a07be4: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00451bf0(param_1: i32, param_2: i32);
    fn FUN_0043c740(param_1: i32, param_2: *mut u32) -> u32;
    fn FUN_00450650(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00452880(param_1: i32) {
    // SAFETY: all globals, callees and the pointers derived from them are the
    // game's own memory, read in the same order as the original.
    unsafe {
        let i_var1 = ((((&raw mut DAT_009e9670) as usize as i32)
            .wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd))) as *const i32)
            .read_unaligned()
            .wrapping_mul(0x23);

        let q =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var1) as *const i32).read_unaligned();
        DAT_00a08292 = u16::from((q.wrapping_add(0x88) as *const u8).read());

        let q =
            (DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var1) as *const i32).read_unaligned();
        let r = (q.wrapping_add(0xa9) as *const i32).read_unaligned();
        let b = (r.wrapping_add(4) as *const u8).read();
        FUN_00451bf0(
            0,
            ((&raw mut DAT_0051c560) as usize as i32)
                .wrapping_add((u32::from(b) as i32).wrapping_mul(0x101)),
        );

        let b = (DAT_009a1fe8.wrapping_add(0x16).wrapping_add(i_var1) as *const u8).read();
        FUN_00451bf0(
            1,
            ((&raw mut DAT_009d8c30) as usize as i32)
                .wrapping_add((u32::from(b) as i32).wrapping_mul(0x566)),
        );

        let b = (DAT_009a1fe8.wrapping_add(i_var1).wrapping_add(0x16) as *const u8).read();
        let q =
            (DAT_009a1fe8.wrapping_add(i_var1).wrapping_add(0x1f) as *const i32).read_unaligned();
        let p = (q.wrapping_add(0xa9) as *const u32).read_unaligned();
        let u_var2 = FUN_0043c740(i32::from(b), p as usize as *mut u32);
        DAT_00a07be4 = u_var2 as u16;

        FUN_00450650(param_1, DAT_009ecc34);
    }
}
