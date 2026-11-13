// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521050: [u8; 0];
    static mut DAT_00521038: [u8; 0];
    static mut DAT_00521068: [u8; 0];
    static mut DAT_00a0aba0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004305d0(param_1: u16) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00430540(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: the globals are the game's own tables, indexed exactly as the
    // original does; the callee is the game's function.
    unsafe {
        let b_var1: u8 = (&raw mut DAT_00521050)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read();
        let b_var2: u8 = (&raw mut DAT_00521038)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read();
        let u_var4 = FUN_004305d0(b_var1 as u16);
        let mut u_var5: u32 = u_var4 as u32;
        let b_var3: u8 = (&raw mut DAT_00521068)
            .cast::<u8>()
            .offset(param_1 as isize)
            .read();
        let base = (&raw mut DAT_00a0aba0).cast::<u8>();
        let row = param_2.wrapping_mul(0x61) as u32;
        if b_var3 != 0 {
            let mut u_var6: u32 = b_var1 as u32;
            let idx = (b_var2 as u32).wrapping_add(u_var5).wrapping_add(row);
            if b_var3 <= base.add(idx as usize).read() {
                while u_var6 != 0 {
                    u_var5 = u_var5.wrapping_add(1);
                    if (b_var1 as u32 as i32) <= (u_var5 as i32) {
                        u_var5 = 0;
                    }
                    u_var6 = u_var6.wrapping_sub(1);
                    let idx2 = (b_var2 as u32).wrapping_add(u_var5).wrapping_add(row);
                    if base.add(idx2 as usize).read() < b_var3 {
                        return u_var5;
                    }
                }
                u_var5 = 0xffffffff;
            }
        }
        u_var5
    }
}
