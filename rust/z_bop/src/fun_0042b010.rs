// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d12ec: i32;
    static mut DAT_009d12f0: i32;
    static mut DAT_009d1304: i32;
}

/// Truncating conversion of st0 (popped), as the runtime's `__ftol` does.
#[inline(always)]
unsafe fn ftol_low() -> i32 {
    let lo: i32;
    // SAFETY: converts and pops the x87 top of stack the caller left,
    // using a scratch area on the stack and restoring the control word.
    unsafe {
        core::arch::asm!(
            "sub esp, 12",
            "fnstcw word ptr [esp]",
            "movzx eax, word ptr [esp]",
            "or eax, 0xC00",
            "mov word ptr [esp + 2], ax",
            "fldcw word ptr [esp + 2]",
            "fistp qword ptr [esp + 4]",
            "fldcw word ptr [esp]",
            "mov eax, dword ptr [esp + 4]",
            "add esp, 12",
            out("eax") lo,
            options(preserves_flags),
        );
    }
    lo
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042b010() {
    // SAFETY: the globals are the game's own; the x87 stack holds the
    // values the caller prepared.
    unsafe {
        DAT_009d12ec = (ftol_low() as i16) as i32;
        DAT_009d12f0 = (ftol_low() as i16) as i32;
        DAT_009d1304 = (ftol_low() as i16) as i32;
    }
}
