// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0070a1d2: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_00483410(param_1: i16, param_2: i16, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00445d10(param_1: i32) {
    // SAFETY: all addresses are the game's own memory, accessed at the same
    // offsets and widths as the original.
    unsafe {
        let i_var1 = FUN_0042b150(DAT_00732e54);
        let mut i_var3 = param_1.wrapping_mul(0x23).wrapping_add(DAT_006d4988);
        let local_int_27 = ((i_var1 as usize).wrapping_add(0x27) as *const i32).read_unaligned();
        let i_var4 = ((i_var3 as usize).wrapping_add(0xc) as *const i32)
            .read_unaligned()
            .wrapping_sub(local_int_27);
        if i_var4 < 0 || i_var4 < 0x101 {
            ((DAT_00732e54 as usize).wrapping_add(0xd) as *mut u16).write_unaligned(0x4000);
            ((DAT_00732e54 as usize).wrapping_add(0x9c) as *mut u8).write(0);
            ((DAT_00732e54 as usize).wrapping_add(0x97) as *mut u8).write(2);
        } else {
            let local_int_1f =
                ((i_var3 as usize).wrapping_add(0x1f) as *const i32).read_unaligned();
            if ((local_int_1f as usize).wrapping_add(0x89) as *const i8).read() != 0 {
                let a = ((i_var3 as usize).wrapping_add(0x12) as *const i16).read_unaligned();
                let b = ((i_var3 as usize).wrapping_add(0x10) as *const i16).read_unaligned();
                FUN_00483410(a, b, i_var3);
            }
            let i_var2 =
                ((local_int_1f as usize).wrapping_add(0x8e) as *const i16).read_unaligned() as i32;
            let m = (i_var2 as u32).wrapping_mul(DAT_0070a1d2 as u32);
            if ((i_var3 as usize).wrapping_add(2) as *const i8).read() == 0 {
                i_var3 = m.wrapping_mul(3) as i32;
            } else {
                i_var3 = m.wrapping_mul(2) as i32;
            }
            let local_short_d =
                ((DAT_00732e54 as usize).wrapping_add(0xd) as *const u16).read_unaligned();
            if i_var4 <= i_var3.wrapping_neg() && 0x4000 < local_short_d {
                let d = ((local_short_d as i32).wrapping_sub(0x4000) >> 1) as i16;
                let v = d.wrapping_add(0x4000) as u16;
                ((i_var1 as usize).wrapping_add(0x2e) as *mut u16).write_unaligned(v);
                ((DAT_00732e54 as usize).wrapping_add(0x9c) as *mut u8).write(1);
                return;
            }
        }
    }
}
