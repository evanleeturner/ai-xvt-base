// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00402b60(param_1: i32, param_2: *mut i32, param_3: *mut u32) -> u32;
    fn FUN_0048f3c0(param_1: u32, param_2: u32) -> u16;
    fn FUN_0048f4f0(param_1: u32) -> i32;
    fn FUN_0048f510();
    fn _ftell(param_1: i32) -> i32;
    fn _fseek(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00402a80(param_1: i32) -> u16 {
    // SAFETY: the callees are the game's own; the block returned by
    // FUN_0048f4f0 is written at the offsets the original uses.
    unsafe {
        let mut i_var7: i32 = 0;
        let mut i_var5: i32 = 0;
        let offset = _ftell(param_1);
        loop {
            let u_var2 = FUN_00402b60(param_1, core::ptr::null_mut(), core::ptr::null_mut());
            if u_var2 == 0xFFFFFFFF {
                break;
            }
            if u_var2 != 0 {
                i_var5 = i_var5.wrapping_add(u_var2 as i32);
                i_var7 = i_var7.wrapping_add(1);
            }
        }
        let mut i_var4: i32 = 0;
        _fseek(param_1, offset, 0);
        let u_var1 = FUN_0048f3c0(
            i_var5
                .wrapping_add(i_var7.wrapping_mul(4))
                .wrapping_add(0xE) as u32,
            0,
        );
        let i_var3 = FUN_0048f4f0(u_var1 as u32);
        let base = i_var3 as usize as *mut u8;
        base.byte_add(6).cast::<i32>().write_unaligned(i_var7);
        i_var5 = 0;
        base.cast::<i32>().write_unaligned(i_var3);
        base.byte_add(4).cast::<u16>().write_unaligned(u_var1);
        base.byte_add(10)
            .cast::<i32>()
            .write_unaligned(i_var3.wrapping_add(0xE));
        let mut pi_var6 = i_var3
            .wrapping_add(0xE)
            .wrapping_add(i_var7.wrapping_mul(4)) as usize as *mut i32;
        if 0 < i_var7 {
            loop {
                let tbl = base.byte_add(10).cast::<i32>().read_unaligned();
                let u_var2 = FUN_00402b60(
                    param_1,
                    pi_var6,
                    tbl.wrapping_add(i_var4) as usize as *mut u32,
                );
                if u_var2 != 0 {
                    i_var4 = i_var4.wrapping_add(4);
                    i_var5 = i_var5.wrapping_add(1);
                    pi_var6 = (pi_var6 as i32).wrapping_add(u_var2 as i32) as usize as *mut i32;
                }
                if !(i_var5 < i_var7) {
                    break;
                }
            }
        }
        FUN_0048f510();
        u_var1
    }
}
