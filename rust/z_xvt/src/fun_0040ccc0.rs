// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523b58: [u16; 1];
    static mut DAT_006d9984: *const core::ffi::c_void;
    static mut DAT_0071e524: u8;
    static mut DAT_006da9fa: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ed70(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040ccc0(param_1: u16, param_2: u32, param_3: u32) {
    // SAFETY: the globals, the table and the callbacks are the game's own memory
    // and functions; the table is read at the indices the original uses.
    unsafe {
        let mut param_1 = param_1;
        let mut param_2 = param_2;
        let u_var3 = DAT_0071e524;
        let u_var2 = DAT_006da9fa;
        if param_1 != 0xffff {
            let mut b_var1: i32 = 0;
            while param_2 != 0 {
                let d = (&raw mut DAT_00523b58)
                    .cast::<u16>()
                    .add(param_2 as usize)
                    .read_unaligned() as u32;
                let q: u32;
                core::arch::asm!(
                    "div {d:e}",
                    d = in(reg) d,
                    inout("eax") param_1 as u32 => q,
                    inout("edx") 0u32 => _,
                    options(nomem, nostack)
                );
                let mut u_var4 = q as u16;
                param_1 = (param_1 as u32).wrapping_sub(d.wrapping_mul(u_var4 as u32)) as u16;
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
                let f: unsafe extern "C" fn(i16) = core::mem::transmute(DAT_006d9984);
                f(s_var5);
                param_2 = param_2.wrapping_sub(1);
            }
            return;
        }
        DAT_0071e524 = 0;
        FUN_0048ed70(0x40);
        while param_2 != 0 {
            let f: unsafe extern "C" fn(i16) = core::mem::transmute(DAT_006d9984);
            f(0x30);
            param_2 = param_2.wrapping_sub(1);
        }
        DAT_0071e524 = u_var3;
        DAT_006da9fa = u_var2;
    }
}
