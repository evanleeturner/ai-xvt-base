// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73b6: i32;
    static mut DAT_009a8070: i16;
    static mut DAT_00a08104: i32;
    static mut DAT_00a080f0: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046a8d0(param_1: u32, param_2: u32, param_3: u32);
    fn FUN_00404620(param_1: i32) -> u32;
    fn FUN_004a4cb0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a44d0(param_1: i16, param_2: i32) {
    // SAFETY: the globals and callees are the game's own; the pointers
    // derived from them address the game's records, accessed where the
    // original does.
    unsafe {
        let i_var2 = DAT_009a73b0
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32) as i32;

        let val_1f = ((DAT_009a73b6.wrapping_add(0x1f)) as *const u32).read_unaligned();
        let val_23 = ((DAT_009a73b6.wrapping_add(0x23)) as *const u32).read_unaligned();
        let val_27 = ((DAT_009a73b6.wrapping_add(0x27)) as *const u32).read_unaligned();

        FUN_0046a8d0(
            val_1f.wrapping_sub(((i_var2.wrapping_add(4)) as *const u32).read_unaligned()),
            val_23.wrapping_sub(((i_var2.wrapping_add(8)) as *const u32).read_unaligned()),
            val_27.wrapping_sub(((i_var2.wrapping_add(0xc)) as *const u32).read_unaligned()),
        );

        ((DAT_009a73b6.wrapping_add(0x32)) as *mut i16)
            .write_unaligned(DAT_009a8070.wrapping_add(param_1));
        let u_var1 = FUN_00404620(DAT_00a08104);
        FUN_004a4cb0((((u_var1 >> 1) as u16 & 0x7fff) as u32).wrapping_add(0x4000));

        if param_2 != 0 {
            ((DAT_009a73b6.wrapping_add(0x2e)) as *mut u16).write_unaligned(DAT_00a080f0 as u16);
            ((DAT_00a08104.wrapping_add(0x9e)) as *mut u16).write_unaligned(0xffff);
            ((DAT_00a08104.wrapping_add(0x9d)) as *mut u8).write(0);

            let val_0d = ((DAT_00a08104.wrapping_add(0xd)) as *const u16).read_unaligned();
            let cmp = ((DAT_009a73b6.wrapping_add(0x2e)) as *const u16).read_unaligned();
            if val_0d < cmp {
                ((DAT_00a08104.wrapping_add(0x9c)) as *mut u8).write(2);
            } else {
                ((DAT_00a08104.wrapping_add(0x9c)) as *mut u8).write(1);
            }
            ((DAT_00a08104.wrapping_add(0x96)) as *mut u8).write(0);
            ((DAT_00a08104.wrapping_add(0x97)) as *mut u8).write(0);
        }
    }
}
