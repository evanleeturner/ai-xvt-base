// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d77f8: u8;
    static mut DAT_009a7044: [u32; 1];
    static mut DAT_009a6ff0: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00450c30() {
    // SAFETY: the globals are the game's own memory; the copies read and write
    // the same dwords in the same order as the original, overlap included.
    unsafe {
        if DAT_009d77f8 != 0 {
            let mut u_var2: u16 = 0;
            loop {
                let u_var3: u16 = u_var2.wrapping_add(1);
                let mut p_var4 = (&raw mut DAT_009a7044)
                    .cast::<u32>()
                    .add(u_var2 as usize * 0x15);
                let mut p_var5 = (&raw mut DAT_009a6ff0)
                    .cast::<u32>()
                    .add(u_var2 as usize * 0x15);
                let mut i_var1: i32 = 0x15;
                while i_var1 != 0 {
                    p_var5.write_unaligned(p_var4.read_unaligned());
                    p_var4 = p_var4.add(1);
                    p_var5 = p_var5.add(1);
                    i_var1 -= 1;
                }
                u_var2 = u_var3;
                if !(u_var3 < DAT_009d77f8 as u16) {
                    break;
                }
            }
        }
        DAT_009d77f8 = DAT_009d77f8.wrapping_sub(1);
    }
}
