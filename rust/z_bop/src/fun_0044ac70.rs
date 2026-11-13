// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233ec: i32;
    static mut DAT_005233f4: i32;
    static mut DAT_005233cc: i32;
    static mut DAT_00a00588: i16;
    static mut DAT_00a00530: i16;
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
pub unsafe extern "C" fn FUN_0044ac70(param_1: *mut u16, param_2: i16) {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's array of 3-word records, read and written as the original does.
    unsafe {
        while param_2 != 0 {
            let u_var1 = param_1.read_unaligned();
            let i_var2 = FUN_0040e190(param_1.add(1).read_unaligned() as i32);
            let mut u_var3: u32 = (i_var2 as u32).wrapping_add((u_var1 as u32).wrapping_mul(2));
            if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                let divisor = DAT_005233cc as u32;
                let rem: u32;
                let _q: u32;
                core::arch::asm!(
                    "div {d:e}",
                    d = in(reg) divisor,
                    inout("eax") u_var3 => _q,
                    inout("edx") 0u32 => rem,
                    options(nomem, nostack)
                );
                u_var3 = rem;
                FUN_00410ff0();
                FUN_00410ff0();
            }
            let addr = u_var3.wrapping_add((DAT_005233f4 as u32) & 0xffff_fffe);
            let psv_var4 = addr as *mut i16;
            if psv_var4.read_unaligned() == DAT_00a00588 {
                let idx = (param_1.add(2).read_unaligned() as u8) as usize;
                let src = (&raw mut DAT_00a00530).add(idx * 2);
                psv_var4.write_unaligned(src.read_unaligned());
            } else {
                param_1.add(2).write_unaligned(0);
            }
            param_1 = param_1.add(3);
            param_2 = param_2.wrapping_sub(1);
        }
    }
}
