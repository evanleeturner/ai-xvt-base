// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a6fe4: unsafe extern "C" fn(i32);
    static mut DAT_009ec464: u8;
    static mut DAT_009a807a: u8;
    static mut DAT_00520eb0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9590(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004277f0(param_1: u16, param_2: u32, param_3: u32) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: the globals, the table and the callees are the game's own;
    // the table is read where the original reads it, and the divide faults
    // on a zero entry as the machine does.
    unsafe {
        let u_var3 = DAT_009ec464;
        let u_var2 = DAT_009a807a;
        if param_1 != 0xffff {
            let mut b_var1: i32 = 0;
            while param_2 != 0 {
                let entry = (&raw mut DAT_00520eb0)
                    .cast::<u8>()
                    .byte_add(param_2.wrapping_mul(2) as usize)
                    .cast::<u16>()
                    .read_unaligned();
                let q: u32;
                asm!(
                    "div {d:e}",
                    d = in(reg) entry as u32,
                    inout("eax") param_1 as u32 => q,
                    inout("edx") 0u32 => _,
                );
                let mut u_var4 = q as u16;
                param_1 = param_1.wrapping_sub(entry.wrapping_mul(u_var4));
                let s_var5: i16;
                if b_var1 != 0 || param_2 <= param_3 || u_var4 != 0 {
                    b_var1 = 1;
                    if 9 < u_var4 {
                        u_var4 = 9;
                    }
                    s_var5 = u_var4.wrapping_add(0x30) as i16;
                } else {
                    s_var5 = 0x20;
                    b_var1 = 0;
                }
                (DAT_009a6fe4)(s_var5 as i32);
                param_2 = param_2.wrapping_sub(1);
            }
            return;
        }
        DAT_009ec464 = 0;
        FUN_004a9590(0x40);
        while param_2 != 0 {
            (DAT_009a6fe4)(0x30);
            param_2 = param_2.wrapping_sub(1);
        }
        DAT_009ec464 = u_var3;
        DAT_009a807a = u_var2;
    }
}
