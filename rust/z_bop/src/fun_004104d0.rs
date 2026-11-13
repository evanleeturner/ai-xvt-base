// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233cc: u32;
    static mut DAT_005233ec: u32;
    static mut DAT_005233f4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e190(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00410ff0();
}

#[inline(always)]
unsafe fn umod(n: u32, d: u32) -> u32 {
    let r: u32;
    // SAFETY: the machine's own unsigned divide; a zero divisor faults as in the original.
    unsafe {
        core::arch::asm!(
            "div {d:e}",
            d = in(reg) d,
            inout("eax") n => _,
            inout("edx") 0u32 => r,
            options(nostack)
        );
    }
    r
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004104d0(param_1: *mut u16, param_2: i16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's array of 3-word records, and the written address is the
    // game's own video memory, as in the original.
    unsafe {
        while param_2 != 0 {
            let u_var1 = param_1.read_unaligned();
            let i_var2 = FUN_0040e190(param_1.add(1).read_unaligned() as u32 as i32);
            let mut u_var3 = (u_var1 as u32).wrapping_add(i_var2 as u32);
            if DAT_005233ec != 0x13 && DAT_005233f4 != 0xa0000 {
                u_var3 = umod(u_var3, DAT_005233cc);
                FUN_00410ff0();
            }
            if (param_1.add(2).read_unaligned() & 1) != 0 {
                (u_var3.wrapping_add(DAT_005233f4) as usize as *mut u8).write(0x2c);
            }
            if DAT_005233ec == 0x101 {
                let u_var1 = param_1.read_unaligned();
                let w = param_1.add(1).read_unaligned().wrapping_add(1);
                let i_var2 = FUN_0040e190(w as u32 as i32);
                let mut u_var3 = (u_var1 as u32).wrapping_add(i_var2 as u32);
                if DAT_005233ec != 0x13 && DAT_005233f4 != 0xa0000 {
                    u_var3 = umod(u_var3, DAT_005233cc);
                    FUN_00410ff0();
                }
                if (param_1.add(2).read_unaligned() & 2) != 0 {
                    (u_var3.wrapping_add(DAT_005233f4) as usize as *mut u8).write(0x2c);
                }
            }
            param_1 = param_1.add(3);
            param_2 = param_2.wrapping_sub(1);
        }
    }
}
