// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075766c: i16;
    static mut DAT_00743d86: i16;
    static mut DAT_00743d02: i16;
    static mut DAT_00743d2e: i16;
    static mut _DAT_00650408: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f33c0(
        param_1: *mut i32,
        param_2: i16,
        param_3: i16,
        param_4: *mut i16,
        param_5: *mut i16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f32e0(param_1: *mut i32, param_2: i16, param_3: i16) -> u32 {
    let mut local_6: i16 = 0;
    let mut local_4: i16 = 0;
    let mut local_2: i16 = 0;
    let mut s_var6: i16 = 0;
    let mut s_var1: i16 = 999;
    let mut u_var4: u16 = 0;

    // SAFETY: the globals and the callee are the game's own; the table is
    // indexed by entry as the original does, and the locals outlive the call.
    unsafe {
        if DAT_0075766c > 0 {
            loop {
                let mut i_var2 = s_var6 as i32;
                let mut i_var3 = i_var2.wrapping_mul(0x562);
                let idx = i_var2.wrapping_mul(0x2b1) as isize;
                if (&raw mut DAT_00743d86).offset(idx).read() != 0 {
                    FUN_004f33c0(
                        param_1,
                        (&raw mut DAT_00743d02).offset(idx).read(),
                        (&raw mut DAT_00743d2e).offset(idx).read(),
                        &raw mut local_6,
                        &raw mut local_4,
                    );
                    let d = (param_2 as i32).wrapping_sub(local_6 as i32);
                    let u_var5 = d >> 31;
                    i_var2 = (d ^ u_var5).wrapping_sub(u_var5);
                    i_var3 = i_var2;
                    if i_var2 < s_var1 as i32 {
                        let d2 = (param_3 as i32).wrapping_sub(local_4 as i32);
                        let u5 = d2 >> 31;
                        i_var3 = (d2 ^ u5).wrapping_sub(u5);
                        if i_var3 < s_var1 as i32 {
                            s_var1 = i_var3 as i16;
                            local_2 = s_var6;
                            if i_var3 <= i_var2 {
                                s_var1 = i_var2 as i16;
                            }
                        }
                    }
                }
                u_var4 = ((i_var3 as u32) >> 16) as u16;
                s_var6 = s_var6.wrapping_add(1);
                if !(s_var6 < DAT_0075766c) {
                    break;
                }
            }
        }

        if s_var1 == 999 {
            return (u_var4 as u32) << 16;
        }
        _DAT_00650408 = local_2;
        ((u_var4 as u32) << 16) | 1
    }
}
