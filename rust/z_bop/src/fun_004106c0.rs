// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a81c: i16;
    static mut PTR_DAT_0051a818: *mut i8;
    static mut DAT_00a08c86: u16;
    static mut DAT_00a08c88: u16;
    static mut DAT_005233ec: i32;
    static mut DAT_005233f4: i32;
    static mut DAT_005233cc: u32;
    static mut DAT_0054f9c0: i8;
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
pub unsafe extern "stdcall" fn FUN_004106c0() {
    // SAFETY: globals and callees are the game's own memory; the pointer
    // table and the destination are read and written where the original does.
    unsafe {
        let mut u_var1: u16 = 0;
        let mut u_var6: u32 = 0;
        let mut s_var4: i16 = DAT_0051a81c;
        while s_var4 != 0 {
            let pc_var5: *mut i8 = PTR_DAT_0051a818.byte_add(u_var6 as usize);
            let b1 = pc_var5.byte_add(1).read() as i32;
            let i_var2 = FUN_0040e190(b1.wrapping_add(DAT_00a08c86 as i32));
            let mut u_var3: u32 = (i_var2 as u32)
                .wrapping_add(pc_var5.read() as i32 as u32)
                .wrapping_add(DAT_00a08c88 as u32);
            if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                let divisor = DAT_005233cc;
                let rem: u32;
                core::arch::asm!(
                    "div {d}",
                    d = in(reg) divisor,
                    inout("eax") u_var3 => _,
                    inout("edx") 0u32 => rem,
                    options(nomem, nostack)
                );
                u_var3 = rem;
                FUN_00410ff0();
            }
            u_var6 = (u_var6 as u16).wrapping_add(2) as u32;
            let src = (&raw mut DAT_0054f9c0).add(u_var1 as usize).read() as u8;
            let dst = u_var3.wrapping_add(DAT_005233f4 as u32) as *mut u8;
            dst.write(src);
            u_var1 = u_var1.wrapping_add(1);
            s_var4 = s_var4.wrapping_sub(1);
        }
    }
}
