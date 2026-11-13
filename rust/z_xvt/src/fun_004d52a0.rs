// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007806ee: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(
        dw_exception_code: u32,
        dw_exception_flags: u32,
        n_number_of_arguments: u32,
        lp_arguments: *const u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d52a0() -> i32 {
    // SAFETY: the global and RaiseException belong to the game and the OS;
    // the global is read unaligned because its address is not 4-aligned.
    // The idiv is the machine's own, so a zero divisor faults as in the original.
    unsafe {
        let divisor: i32 = (&raw mut DAT_007806ee).read_unaligned();
        if divisor == 0 {
            RaiseException(0xC000_0094, 0, 0, core::ptr::null());
        }
        let quotient: i32;
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) divisor,
            inout("eax") 1000i32 => quotient,
            out("edx") _,
        );
        quotient
    }
}
