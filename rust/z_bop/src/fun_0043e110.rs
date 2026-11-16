// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: i8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08374: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a95c0(param_1: u32);
    fn FUN_004a9640(param_1: i8);
    fn FUN_00440840(param_1: i32, param_2: u16, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043e110() {
    // SAFETY: all globals, callees and the pointers derived from them are
    // the game's own memory, read and called in the original's order.
    unsafe {
        let off = DAT_009ecc34.wrapping_mul(0x5bd) as isize;
        let idx = (&raw mut DAT_009e9670)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        let addr = idx
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8);
        let base_ptr = (addr as *const i32).read_unaligned();
        let ptr_a9 = base_ptr.wrapping_add(0xa9);
        let local_ptr_a9 = (ptr_a9 as *const i32).read_unaligned();
        let flags = ((local_ptr_a9.wrapping_add(0x1e1)) as *const u8).read();
        if (flags & 0x40) != 0 {
            FUN_004a95c0(0x40);
            let i_var1 = local_ptr_a9;
            let local_ushort_bd = (i_var1.wrapping_add(0xbd) as *const u16).read_unaligned();
            let mut u_var2: u16 = local_ushort_bd / 0x28f;
            let local_short_bf = (i_var1.wrapping_add(0xbf) as *const i16).read_unaligned();
            if local_short_bf == 0 {
                u_var2 = u_var2.wrapping_mul(2);
            }
            let c_var3: i8 = if DAT_00a08374 == 0 { 2 } else { 0 };
            FUN_004a9640(c_var3);
            FUN_00440840(DAT_00a08374.wrapping_add(0x29), u_var2, 1);
        }
    }
}
