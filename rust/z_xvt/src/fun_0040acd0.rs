// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525550: i32;
    static mut DAT_0070a189: i8;
    static mut DAT_0085e792: i8;
    static mut DAT_0085e79b: u8;
    static mut DAT_0071b730: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0071b741: i8;
    static mut DAT_0073bbf8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439830(param_1: i32, param_2: i32) -> u32;
    fn FUN_004398b0(param_1: i32);
    fn FUN_00490a50(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040acd0() {
    // SAFETY: the globals and callees are the game's own memory; the
    // computed addresses are read exactly as the original does.
    unsafe {
        if DAT_00525550 == 0 && DAT_0070a189 != 0 && DAT_0085e792 != 0 && DAT_0085e79b != 0 {
            let base = (&raw mut DAT_0071b730) as usize as i32;
            let idx = DAT_0071ecf4.wrapping_mul(0x5bd);
            let mut i_var1 =
                ((base.wrapping_add(idx)) as u32 as usize as *const i32).read_unaligned();
            if i_var1 == -1 {
                let u_var2 = FUN_00439830(0x13, 0x100);
                if u_var2 != 0 {
                    FUN_004398b0(0x13);
                    return;
                }
            } else {
                let inner = ((i_var1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_006d4988)) as u32 as usize
                    as *const i32)
                    .read_unaligned();
                i_var1 =
                    ((inner.wrapping_add(0xa7)) as u32 as usize as *const i32).read_unaligned();
                let c = (i_var1.wrapping_add(0x23c) as u32 as usize as *const i8).read();
                if c == 1 {
                    let p = ((&raw mut DAT_0071b741) as usize as i32)
                        .wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd));
                    if (p as u32 as usize as *const i8).read() == 1 {
                        let u_var2 = FUN_00439830(0x13, 0x100);
                        if u_var2 != 0 {
                            FUN_004398b0(0x13);
                            return;
                        }
                    } else {
                        let mut i_var3: i32 = 0x7f;
                        if DAT_0085e79b < 10 {
                            i_var3 = (DAT_0085e79b as i32).wrapping_mul(0xd);
                        }
                        let s = (i_var1.wrapping_add(0x23e) as u32 as usize as *const i16)
                            .read_unaligned();
                        if s == 0 {
                            let u_var2 = FUN_00439830(0x13, 0x100);
                            if u_var2 != 0 {
                                FUN_004398b0(0x13);
                            }
                        } else {
                            let u_var2 = FUN_00439830(0x13, 0x100);
                            if u_var2 == 0 {
                                FUN_00490a50(
                                    (&raw mut DAT_0073bbf8).cast::<core::ffi::c_void>(),
                                    1,
                                    1,
                                    0x7d,
                                    i_var3 >> 2,
                                    0x40,
                                );
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
