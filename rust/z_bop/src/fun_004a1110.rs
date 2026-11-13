// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73b6: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425cb0() -> u16;
    fn FUN_00404620(param_1: i32) -> u32;
    fn FUN_004a4cb0(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a1110(param_1: i32) {
    // SAFETY: the callees and globals are the game's own; the addresses
    // computed from the globals point at the game's structures, read and
    // written exactly where the original does.
    unsafe {
        let mut u_var1: u16 = FUN_00425cb0();
        u_var1 = (u_var1 & 0x1f).wrapping_add(0x32);
        let u_var2: u16 = FUN_00425cb0();
        let mut s_var3: i16 = ((u_var2 & 0xff).wrapping_add(0x180)) as i16;

        let local_b9: i32 = (DAT_00a08104.wrapping_add(0xb9) as *const i32).read_unaligned();
        if local_b9 >= 0 {
            u_var1 = 0u16.wrapping_sub(u_var1);
            s_var3 = 0i16.wrapping_sub(s_var3);
        }

        let local_store: u32 = u_var1 as u32;
        (DAT_00a08104.wrapping_add(0xb9) as *mut u32).write_unaligned(local_store);

        let addr = param_1
            .wrapping_mul(0x23)
            .wrapping_add(0x10)
            .wrapping_add(DAT_009a1fe8);
        let local_read: i16 = (addr as *const i16).read_unaligned();
        (DAT_009a73b6.wrapping_add(0x32) as *mut i16)
            .write_unaligned(local_read.wrapping_add(s_var3));

        let u_var4: u32 = FUN_00404620(DAT_00a08104);
        FUN_004a4cb0((((u_var4 >> 1) as u16) & 0x7fff).wrapping_add(0x8000));

        (DAT_009a73b6.wrapping_add(0x3a) as *mut u16).write_unaligned(0x76);
    }
}
