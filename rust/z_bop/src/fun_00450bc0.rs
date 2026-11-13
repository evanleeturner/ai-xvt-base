// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d77f8: u8;
    static mut DAT_009a6ffd: u8;
    static mut DAT_009a6ffc: u8;
    static mut DAT_009a6f9c: u8;
    static mut DAT_009a6ff0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00450bc0() {
    // SAFETY: the globals are the game's own memory; the copy reads and
    // writes the same dword ranges, in the same order, as the original.
    unsafe {
        let mut b_var1: u8 = DAT_009d77f8;
        if DAT_009a6ffd < 2 && DAT_009a6ffc == 0 {
            let mut u_var4: u16 = (DAT_009d77f8 as u32).wrapping_add(1) as u16;
            while u_var4 != 0 {
                let u_var2: u32 = u_var4 as u32;
                u_var4 = u_var4.wrapping_sub(1);
                let mut p5 = (&raw mut DAT_009a6f9c)
                    .cast::<u8>()
                    .byte_add(u_var2.wrapping_mul(0x54) as usize)
                    .cast::<u32>();
                let mut p6 = (&raw mut DAT_009a6ff0)
                    .cast::<u8>()
                    .byte_add(u_var2.wrapping_mul(0x15) as usize)
                    .cast::<u32>();
                let mut i_var3: i32 = 0x15;
                while i_var3 != 0 {
                    p6.write_unaligned(p5.read_unaligned());
                    p5 = p5.add(1);
                    p6 = p6.add(1);
                    i_var3 -= 1;
                }
            }
            b_var1 = DAT_009d77f8.wrapping_add(1);
            if 9 < b_var1 {
                b_var1 = DAT_009d77f8;
            }
        }
        DAT_009d77f8 = b_var1;
    }
}
