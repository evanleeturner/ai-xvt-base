// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa610c: i32;
    static mut DAT_00aa6104: i32;
    static mut DAT_00a921fd: i8;
    static mut DAT_00a921e3: i8;
    static mut DAT_00aa5e10: i32;
    static mut DAT_00aa5cb0: i32;
    static mut DAT_00aa615e: i32;
    static mut DAT_00aa5ac0: i32;
    static mut DAT_00aa6176: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ee7c0(param_1: i32) -> i8 {
    // SAFETY: all globals are the game's own memory; the arrays are indexed
    // exactly as the original does.
    unsafe {
        if param_1 == -1 {
            if DAT_00aa610c != 0 {
                let idx = DAT_00aa610c.wrapping_add(DAT_00aa6104.wrapping_mul(0x562));
                return (&raw mut DAT_00a921fd).offset(idx as isize).read();
            }
            let idx = DAT_00aa6104.wrapping_mul(0x562);
            let v = (&raw mut DAT_00a921e3).offset(idx as isize).read();
            if v != 0 {
                return v;
            }
            return 0;
        }
        let mut local_4: i32 = 0;
        let mut i_var3: i32 = 0;
        let mut pi_var2: *mut i32 = &raw mut DAT_00aa5e10;
        let mut local_8: *mut i32 = &raw mut DAT_00aa5cb0;
        let key = (&raw mut DAT_00aa615e)
            .cast::<u8>()
            .byte_offset(param_1.wrapping_mul(0x2a) as isize)
            .cast::<i32>()
            .read_unaligned();
        loop {
            let mut i_var1: i32 = 0;
            if 0 < pi_var2.read() {
                let mut pi_var4 = local_8;
                loop {
                    if pi_var4.read() == key {
                        local_4 = (&raw mut DAT_00aa5ac0)
                            .offset(i_var1.wrapping_add(i_var3) as isize)
                            .read();
                        break;
                    }
                    pi_var4 = pi_var4.add(1);
                    i_var1 = i_var1.wrapping_add(1);
                    if !(i_var1 < pi_var2.read()) {
                        break;
                    }
                }
            }
            i_var3 = i_var3.wrapping_add(8);
            pi_var2 = pi_var2.add(1);
            local_8 = local_8.add(8);
            if 0xaa5e37usize < pi_var2 as usize {
                let i_var3 = (&raw mut DAT_00aa6176)
                    .cast::<u8>()
                    .byte_offset(param_1.wrapping_mul(0x2a) as isize)
                    .cast::<i32>()
                    .read_unaligned();
                if i_var3 != 0 {
                    let idx = i_var3.wrapping_add(local_4.wrapping_mul(0x562));
                    return (&raw mut DAT_00a921fd).offset(idx as isize).read();
                }
                let idx = local_4.wrapping_mul(0x562);
                let v = (&raw mut DAT_00a921e3).offset(idx as isize).read();
                if v != 0 {
                    return v;
                }
                return 0;
            }
        }
    }
}
