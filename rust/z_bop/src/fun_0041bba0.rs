// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041bb00(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041bba0(param_1: u32) {
    // SAFETY: the global and callee are the game's own; the addresses
    // computed are read where the original reads them.
    unsafe {
        let i_var1 = ((param_1 & 0xffff) as i32).wrapping_mul(0x23);
        let mut i_var3 =
            ((DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var1)) as *const i32).read_unaligned();
        if i_var3 != 0 {
            let mut u_var2 = u32::from(((i_var3.wrapping_add(0x12)) as *const u8).read());
            if u_var2 != 0 {
                i_var3 = 0;
                loop {
                    i_var3 = i_var3.wrapping_add(2);
                    let base = ((DAT_009a1fe8.wrapping_add(0x1f).wrapping_add(i_var1))
                        as *const i32)
                        .read_unaligned();
                    let local_short = (base.wrapping_add(0x51).wrapping_add(i_var3) as *const u16)
                        .read_unaligned();
                    FUN_0041bb00(local_short);
                    u_var2 = u_var2.wrapping_sub(1);
                    if u_var2 == 0 {
                        break;
                    }
                }
            }
        }
    }
}
