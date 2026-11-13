// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425b70(param_1: u32, param_2: u16) -> u32;
    fn FUN_004a4d70(param_1: i32, param_2: i16);
    fn FUN_00425c20(param_1: u16, param_2: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a4d90(param_1: i32, param_2: u32) {
    // SAFETY: the global and the structures it points to are the game's own
    // memory, read at the offsets the original uses; callees are the game's.
    unsafe {
        let i_var1: i32 = DAT_009a1fe8
            .cast::<u8>()
            .byte_offset(param_1.wrapping_mul(0x23) as isize)
            .byte_add(0x1f)
            .cast::<i32>()
            .read_unaligned();
        let i_var2: i32 =
            ((i_var1 as u32).wrapping_add(0xa9) as usize as *const i32).read_unaligned();
        let u_var6: u16 = ((i_var1 as u32) >> 0x10) as u16;
        let b1 = *((i_var2 as u32).wrapping_add(0x219) as usize as *const u8) as i32;
        let b2 = *((i_var2 as u32).wrapping_add(0x23e) as usize as *const u8) as i32;
        let b3 = *((i_var2 as u32).wrapping_add(0x21c) as usize as *const u8) as i32;
        let u_var3: u16 = 6i32.wrapping_sub(b1).wrapping_sub(b2).wrapping_sub(b3) as u16;
        let mut u_var7: u16 =
            ((i_var2 as u32).wrapping_add(0x92) as usize as *const u16).read_unaligned();
        if u_var3 < 0x8000 {
            let a = ((u_var6 as u32) << 16) | (u_var3.wrapping_mul(0x2000) as u32);
            let u_var4 = FUN_00425b70(a, u_var7);
            u_var7 = u_var7.wrapping_add(u_var4 as u16);
        } else {
            let a = ((u_var6 as u32) << 16) | (u_var3.wrapping_mul(0xe000) as u32);
            let u_var4 = FUN_00425b70(a, u_var7);
            u_var7 = u_var7.wrapping_sub(u_var4 as u16);
        }
        if (u_var7 as u32) <= param_2 {
            FUN_004a4d70(param_1, -1);
            return;
        }
        let u_var5 = FUN_00425c20(param_2 as u16, u_var7);
        FUN_004a4d70(param_1, u_var5 as i16);
    }
}
