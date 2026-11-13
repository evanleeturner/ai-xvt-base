// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00520ef0: [i32; 1];
    static mut DAT_009a6fe4: *const core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042cbd0(mut param_1: i32, mut param_2: u32, param_3: u32) {
    let mut b_var1: bool = false;
    while param_2 != 0 {
        // SAFETY: the table and the function pointer are the game's own;
        // the division is the machine's idiv, faulting as the original does.
        unsafe {
            let divisor: i32 = (&raw mut DAT_00520ef0)
                .cast::<i32>()
                .add(param_2 as usize)
                .read_unaligned();
            let q: i32;
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) divisor,
                inout("eax") param_1 => q,
                out("edx") _,
            );
            let mut u_var2: u32 = q as u32;
            param_1 = param_1.wrapping_sub((u_var2 & 0xffff).wrapping_mul(divisor as u32) as i32);
            let u_var4: u32 = u_var2 >> 16;
            let u_var3: u32;
            if b_var1 || param_2 > param_3 || (u_var2 as u16) != 0 {
                b_var1 = true;
                if (u_var2 as u16) > 9 {
                    u_var2 = (u_var2 & 0xffff0000) | 9;
                }
                u_var3 = (((u_var2 >> 16) as u16 as u32) << 16)
                    | ((u_var2 as u16).wrapping_add(0x30) as u32);
            } else {
                u_var3 = (u_var4 << 16) | 0x20;
                b_var1 = false;
            }
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(DAT_009a6fe4);
            f(u_var3);
        }
        param_2 = param_2.wrapping_sub(1);
    }
}
