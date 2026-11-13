// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00663658: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    // The callee leaves its result in ST0; it is declared as returning f64
    // so that the x87 top-of-stack value is what the following __ftol sees.
    fn FUN_004add60(param_1: i32) -> f64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004adff0(param_1: i32) -> i64 {
    // SAFETY: the global table and callee are the game's own; the table is
    // indexed as the original does, and ST0 is consumed by the truncating
    // conversion exactly once on each path.
    unsafe {
        let p = (&raw mut DAT_00663658)
            .cast::<u8>()
            .byte_add(param_1.wrapping_mul(4) as usize)
            .cast::<i32>();
        if p.read_unaligned() == 0 {
            let v = FUN_004add60(param_1);
            return v as i64;
        }
        let lo: u32;
        let hi: u32;
        core::arch::asm!(
            "sub esp, 8",
            "fisttp qword ptr [esp]",
            "mov {l}, dword ptr [esp]",
            "mov {h}, dword ptr [esp + 4]",
            "add esp, 8",
            l = out(reg) lo,
            h = out(reg) hi,
        );
        (((hi as u64) << 32) | lo as u64) as i64
    }
}
