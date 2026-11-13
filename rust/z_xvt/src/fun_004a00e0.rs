// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4a60: i8;
    static mut DAT_006d4a61: i8;
    static mut DAT_006d4a6a: i8;
    static mut DAT_006d4a6c: i8;
    static mut DAT_006d4a70: i8;
    static mut DAT_006d4a74: i8;
    static mut DAT_006d4a78: i8;
    static mut DAT_006d4a7c: i8;
    static mut DAT_006d4a80: i8;
    static mut DAT_006d4a88: i8;
    static mut DAT_006d4aa2: i8;
    static mut DAT_006d4aa3: i8;
    static mut DAT_006d4aa5: i8;
    static mut DAT_006d4aa7: i8;
    static mut DAT_006d4aa9: i8;
    static mut DAT_006d4aab: i8;
    static mut DAT_006d4aad: i8;
    static mut DAT_006d4ab1: i8;
}

/// SAFETY: caller guarantees `base + off` is valid game memory.
unsafe fn add16(base: *mut i8, off: i32, v: i16) {
    unsafe {
        let p = base.byte_offset(off as isize).cast::<i16>();
        p.write_unaligned(p.read_unaligned().wrapping_add(v));
    }
}

/// SAFETY: caller guarantees `base + off` is valid game memory.
unsafe fn add8(base: *mut i8, off: i32, v: i8) {
    unsafe {
        let p = base.byte_offset(off as isize);
        p.write(p.read().wrapping_add(v));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a00e0(param_1: i32) {
    let i_var1 = param_1.wrapping_mul(0x126);
    // SAFETY: all globals are the game's own memory; offsets are applied
    // exactly as the original does (including the 0x93 stride on the first read).
    unsafe {
        let b = (&raw mut DAT_006d4a6a)
            .byte_offset(param_1.wrapping_mul(0x93) as isize)
            .read();
        let cur = (&raw mut DAT_006d4a6c)
            .byte_offset(i_var1 as isize)
            .cast::<i16>()
            .read_unaligned();
        let s_var2: i16 = (b as i16).wrapping_sub(cur);
        let a2 = (&raw mut DAT_006d4aa2).byte_offset(i_var1 as isize).read();
        let a3 = (&raw mut DAT_006d4aa3).byte_offset(i_var1 as isize).read();
        let c_var3: i8 = a2.wrapping_sub(a3);

        add16(&raw mut DAT_006d4a6c, i_var1, s_var2);
        add16(&raw mut DAT_006d4a70, i_var1, s_var2);
        add8(&raw mut DAT_006d4aa5, i_var1, c_var3);
        add16(&raw mut DAT_006d4a7c, i_var1, s_var2);
        add8(&raw mut DAT_006d4aab, i_var1, c_var3);
        add16(&raw mut DAT_006d4a88, i_var1, s_var2);
        add8(&raw mut DAT_006d4ab1, i_var1, c_var3);
        add16(&raw mut DAT_006d4a78, i_var1, s_var2);
        add8(&raw mut DAT_006d4aa9, i_var1, c_var3);
        add16(&raw mut DAT_006d4a74, i_var1, s_var2);
        add8(&raw mut DAT_006d4aa7, i_var1, c_var3);
        add16(&raw mut DAT_006d4a80, i_var1, s_var2);
        add8(&raw mut DAT_006d4aad, i_var1, c_var3);
        (&raw mut DAT_006d4a60)
            .byte_offset(i_var1 as isize)
            .write(1);
        (&raw mut DAT_006d4a61)
            .byte_offset(i_var1 as isize)
            .write(0);
    }
}
