// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a81c: i16;
    static mut PTR_DAT_0051a818: *mut i8;
    static mut DAT_00a0a1d4: u16;
    static mut DAT_00a0a1d2: u16;
    static mut DAT_005233ec: u32;
    static mut DAT_005233f4: u32;
    static mut DAT_005233cc: u32;
    static mut DAT_0054f9c0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e190(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00410ff0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004105e0() {
    // SAFETY: all globals, callees and the pointers derived from them are the
    // game's own memory, accessed in the same order as the original.
    unsafe {
        let mut u_var7: u32 = 0;
        let mut u_var2: u16 = 0;
        let mut s_var1: i16 = DAT_0051a81c;
        while s_var1 != 0 {
            let pc_var5: *mut i8 = PTR_DAT_0051a818.byte_add(u_var7 as usize);
            let b1 = pc_var5.byte_add(1).read() as i32 as u32;
            let i_var3 = FUN_0040e190(b1.wrapping_add(DAT_00a0a1d4 as u32) as i32);
            let b0 = pc_var5.read() as i32;
            let mut u_var4: u32 =
                (i_var3.wrapping_add(b0) as u32).wrapping_add(DAT_00a0a1d2 as u32);
            if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                let divisor = DAT_005233cc;
                let quot: u32;
                let rem: u32;
                core::arch::asm!(
                    "div {d}",
                    d = in(reg) divisor,
                    inout("eax") u_var4 => quot,
                    inout("edx") 0u32 => rem,
                    options(nomem, nostack)
                );
                let _ = quot;
                u_var4 = rem;
                FUN_00410ff0();
                FUN_00410ff0();
            }
            u_var7 = u_var7.wrapping_add(2);
            let puvar6 = u_var4.wrapping_add(DAT_005233f4) as *mut i8;
            let dst = (&raw mut DAT_0054f9c0).cast::<i8>().add(u_var2 as usize);
            dst.write(puvar6.read());
            puvar6.write(0xceu8 as i8);
            u_var2 = u_var2.wrapping_add(1);
            s_var1 = s_var1.wrapping_sub(1);
        }
    }
}
