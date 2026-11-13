// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00522038: [i32; 0];
    static mut DAT_006d9984: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004026c0(param_1: i32, param_2: u32, param_3: u32) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut b_var1: i32 = 0;

    while param_2 != 0 {
        // SAFETY: the table, the callee and the callback are the game's own.
        unsafe {
            let divisor: i32 = (&raw mut DAT_00522038)
                .cast::<i32>()
                .add(param_2 as usize)
                .read_unaligned();
            if divisor == 0 || (param_1 == i32::MIN && divisor == -1) {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            let quotient: i32;
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) divisor,
                inout("eax") param_1 => quotient,
                out("edx") _,
                options(nostack)
            );
            let mut u_var2: u32 = quotient as u32;
            param_1 = param_1.wrapping_sub(((u_var2 & 0xFFFF) as i32).wrapping_mul(divisor));
            let u_var4: u16 = (u_var2 >> 16) as u16;

            let u_var3: u32;
            if b_var1 != 0 || param_3 < param_2 || (u_var2 as u16) != 0 {
                b_var1 = 1;
                if 9 < (u_var2 as u16) {
                    u_var2 = ((u_var4 as u32) << 16) | 9;
                }
                u_var3 = (((((u_var2 >> 16) as i16) as i32) as u32) << 16)
                    | ((u_var2 as i16).wrapping_add(0x30) as u16 as u32);
            } else {
                u_var3 = ((u_var4 as u32) << 16) | 0x20;
                b_var1 = 0;
            }
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(DAT_006d9984);
            f(u_var3);
        }
        param_2 -= 1;
    }
}
