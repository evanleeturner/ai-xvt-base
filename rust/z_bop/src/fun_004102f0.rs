// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233ec: i32;
    static mut DAT_005233f4: i32;
    static mut DAT_005233cc: u32;
    static mut DAT_009ed238: i16;
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
pub unsafe extern "C" fn FUN_004102f0(
    mut param_1: *mut u8,
    param_2: i32,
    mut param_3: i32,
    param_4: i16,
    mut param_5: i32,
) {
    // SAFETY: the globals and callees are the game's own; param_1 and the
    // destination address computed from the globals are the caller's memory,
    // read and written exactly as the original does.
    unsafe {
        while param_5 != 0 {
            let i_var3 = FUN_0040e190(param_3);
            let mut u_var4: u32 = param_2.wrapping_add(i_var3) as u32;
            if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                let divisor: u32 = DAT_005233cc;
                let rem: u32;
                core::arch::asm!(
                    "xor edx, edx",
                    "div {d:e}",
                    d = in(reg) divisor,
                    inout("eax") u_var4 => _,
                    out("edx") rem,
                );
                u_var4 = rem;
                FUN_00410ff0();
            }
            let mut puVar2 = u_var4.wrapping_add(DAT_005233f4 as u32) as usize as *mut u8;
            DAT_009ed238 = param_4;
            while DAT_009ed238 != 0 {
                let u_var1 = param_1.read();
                param_1 = param_1.add(1);
                puVar2.write(u_var1);
                puVar2 = puVar2.add(1);
                DAT_009ed238 = DAT_009ed238.wrapping_sub(1);
            }
            param_3 = param_3.wrapping_add(1);
            DAT_009ed238 = 0;
            param_5 = param_5.wrapping_sub(1);
        }
    }
}
