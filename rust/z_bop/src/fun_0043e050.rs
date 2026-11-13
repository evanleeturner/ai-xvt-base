// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08374: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a95c0(param_1: u32);
    fn FUN_00425b70(param_1: u32, param_2: u16) -> u32;
    fn FUN_004a9640(param_1: i8);
    fn FUN_00440840(param_1: i32, param_2: u16, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043e050() {
    // SAFETY: the globals and callees are the game's own; the pointers
    // are computed and dereferenced exactly as the original does.
    unsafe {
        let base = (&raw mut DAT_009e9670) as usize as i32;
        let a1 = base.wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd));
        let ptr1: i32 = (a1 as usize as *const i32).read_unaligned();

        let a2 = ptr1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8);
        let ptr2: i32 = (a2 as usize as *const i32).read_unaligned();

        let a3 = ptr2.wrapping_add(0xa9);
        let ptr3: i32 = (a3 as usize as *const i32).read_unaligned();

        let flags: u8 = (ptr3.wrapping_add(0x1e1) as usize as *const u8).read();
        if (flags & 0x40) != 0 {
            FUN_004a95c0(0x40);
            let i_var1 = ptr1.wrapping_mul(0x23);
            let val_short: u16 = (ptr2.wrapping_add(0x79) as usize as *const u16).read_unaligned();
            let u_var2 = FUN_00425b70(((i_var1 as u32) & 0xFFFF0000) | (val_short as u32), 0x71c7);
            let c_var3: i8 = if DAT_00a08374 == 0 { 2 } else { 0 };
            FUN_004a9640(c_var3);
            FUN_00440840(DAT_00a08374.wrapping_add(0x28), u_var2 as u16, 1);
        }
    }
}
