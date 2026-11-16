// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00550b8c: u32;
    static mut DAT_00550bd8: u32;
    static mut DAT_00550b88: *mut u32;
    static mut DAT_00550880: *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00419620() {
    // SAFETY: the globals are the game's own memory; the source and
    // destination pointers address the caller's buffers, copied as the
    // original does, dword by dword and then byte by byte.
    unsafe {
        DAT_00550b8c = DAT_00550bd8;
        let mut p_var2 = DAT_00550b88;
        let mut p_var3 = DAT_00550880;
        let mut u_var1 = DAT_00550bd8 >> 2;
        while u_var1 != 0 {
            p_var3.write_unaligned(p_var2.read_unaligned());
            p_var2 = p_var2.wrapping_add(1);
            p_var3 = p_var3.wrapping_add(1);
            u_var1 = u_var1.wrapping_sub(1);
        }
        let mut u_var1 = DAT_00550b8c & 3;
        let mut b2 = p_var2 as *const u8;
        let mut b3 = p_var3 as *mut u8;
        while u_var1 != 0 {
            b3.write(b2.read());
            b2 = b2.wrapping_add(1);
            b3 = b3.wrapping_add(1);
            u_var1 = u_var1.wrapping_sub(1);
        }
    }
}
