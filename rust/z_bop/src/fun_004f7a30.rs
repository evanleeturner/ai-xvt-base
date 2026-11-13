// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5aac: i16;
    static mut DAT_00a921c6: [i16; 1];
    static mut DAT_00a92142: [i16; 1];
    static mut DAT_00a9216e: [i16; 1];
    static mut _DAT_006691e8: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f7b10(
        param_1: *mut i32,
        param_2: i16,
        param_3: i16,
        param_4: *mut i16,
        param_5: *mut i16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f7a30(param_1: *mut i32, param_2: i16, param_3: i16) -> u32 {
    let mut u_var4: u16 = 0;
    let mut local_2: i16 = 0;
    let mut s_var1: i16 = 999;
    let mut s_var6: i16 = 0;
    let mut local_6: i16 = 0;
    let mut local_4: i16 = 0;

    // SAFETY: the globals and the callee are the game's own; the arrays are
    // indexed through raw pointers exactly as the original does.
    unsafe {
        if 0 < DAT_00aa5aac {
            loop {
                let mut i_var2: i32 = s_var6 as i32;
                let mut i_var3: i32 = i_var2.wrapping_mul(0x562);
                let idx = (i_var2.wrapping_mul(0x2b1)) as isize;
                if (&raw mut DAT_00a921c6).cast::<i16>().offset(idx).read() != 0 {
                    FUN_004f7b10(
                        param_1,
                        (&raw mut DAT_00a92142).cast::<i16>().offset(idx).read(),
                        (&raw mut DAT_00a9216e).cast::<i16>().offset(idx).read(),
                        &raw mut local_6,
                        &raw mut local_4,
                    );
                    let a = (param_2 as i32).wrapping_sub(local_6 as i32);
                    let u5 = (a >> 31) as u32;
                    i_var2 = ((a as u32) ^ u5).wrapping_sub(u5) as i32;
                    i_var3 = i_var2;
                    if i_var2 < s_var1 as i32 {
                        let b = (param_3 as i32).wrapping_sub(local_4 as i32);
                        let u5b = (b >> 31) as u32;
                        i_var3 = ((b as u32) ^ u5b).wrapping_sub(u5b) as i32;
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
                if !(s_var6 < DAT_00aa5aac) {
                    break;
                }
            }
        }
        if s_var1 == 999 {
            return (u_var4 as u32) << 16;
        }
        _DAT_006691e8 = local_2;
        ((u_var4 as u32) << 16) | 1
    }
}
