// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a5510(
        p1: *mut i32,
        p2: *mut i32,
        p3: *mut i32,
        p4: *mut i32,
        p5: *mut u32,
        p6: *mut u32,
        p7: *mut u32,
    );
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0048f800() {
    // SAFETY: the original takes no declared parameters (`ret` with no
    // adjustment, symbol `@0`) and reads its two caller-pushed arguments at
    // [ebp+8] and [ebp+0xc]; a naked body keeps that exact frame layout.
    core::arch::naked_asm!(
        "push ebp",
        "mov ebp, esp",
        "sub esp, 0x34",
        "lea eax, [ebp - 0xc]",
        "push eax",
        "lea eax, [ebp - 0x18]",
        "push eax",
        "lea eax, [ebp - 0x24]",
        "push eax",
        "lea eax, [ebp - 0x30]",
        "push eax",
        "lea eax, [ebp - 0x34]",
        "push eax",
        "lea eax, [ebp - 0x28]",
        "push eax",
        "lea eax, [ebp - 0x2c]",
        "push eax",
        "call {f}",
        "add esp, 0x1c",
        "mov ax, word ptr [ebp - 0x34]",
        "mov ecx, dword ptr [ebp + 8]",
        "mov word ptr [ecx], ax",
        "mov ax, word ptr [ebp - 0x30]",
        "mov ecx, dword ptr [ebp + 0xc]",
        "mov word ptr [ecx], ax",
        "mov esp, ebp",
        "pop ebp",
        "ret",
        f = sym FUN_004a5510,
    );
}
