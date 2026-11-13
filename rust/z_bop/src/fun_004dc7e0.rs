// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acc131: i32;
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b24: i8;
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00acc145: i32;
    static mut DAT_00acc139: i32;
    static mut DAT_00acc13d: i32;
    static mut DAT_00acc149: i32;
    static mut DAT_00acc54d: i32;
    static mut DAT_00acc551: i32;
    static mut DAT_00acc55d: i32;
    static mut DAT_00acc575: i32;
    static mut DAT_00acebbd: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d4d20(param_1: *mut i32);
    fn FUN_004d4e20();
    fn FUN_004d48e0() -> i32;
    fn FUN_004b5e20(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d4ec0(param_1: i32);
    fn FUN_004d4970();
    #[link_name = "_free"]
    fn _free(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dc7e0() {
    // SAFETY: all globals and callees are the game's own memory; the pointers
    // are read and written exactly where the original does.
    unsafe {
        if DAT_00acc131 == 0 {
            return;
        }
        let i_var3 = DAT_00acc131.wrapping_sub(1);
        let mut local_10: [i32; 4] = [0; 4];
        FUN_004d6460(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
        FUN_004d4d20(local_10.as_mut_ptr());
        let c_var2 = DAT_00aa7b0d;
        let i_var8 = i_var3.wrapping_mul(0x440);
        let off = i_var8 as isize;

        let rd = |p: *mut i32| -> i32 { p.byte_offset(off).read_unaligned() };

        if rd(&raw mut DAT_00acc145) >= 1 {
            if DAT_00aa7b1f == 8 {
                if DAT_00aa7b24 == 0 {
                    DAT_00aa6cfc = FUN_004d48e0();
                } else {
                    FUN_004d4e20();
                }
                FUN_004b5e20(
                    (&raw mut DAT_00acc139).byte_offset(off),
                    rd(&raw mut DAT_00acc54d),
                    rd(&raw mut DAT_00acc551),
                );
                if DAT_00aa7b24 != 0 {
                    FUN_004d4ec0(1);
                }
                if c_var2 == 0 {
                    FUN_004d4970();
                }
            } else if DAT_00aa7b1f == 0x10 {
                if DAT_00aa7b24 == 0 {
                    DAT_00aa6cfc = FUN_004d48e0();
                } else {
                    FUN_004d4e20();
                }
                let i_var1 = rd(&raw mut DAT_00acc139);
                let mut i_var6 = rd(&raw mut DAT_00acc13d);
                let mut local_1c = rd(&raw mut DAT_00acc149) as u32 as usize as *const u16;
                let mut puVar5 = rd(&raw mut DAT_00acc551)
                    .wrapping_mul(DAT_00aa7b13)
                    .wrapping_add(rd(&raw mut DAT_00acc54d).wrapping_mul(2))
                    .wrapping_add(DAT_00aa6cfc) as u32 as usize
                    as *mut u16;
                if 0 < i_var6 {
                    loop {
                        let mut i_var4 = i_var1;
                        let mut p7 = local_1c;
                        let mut p9 = puVar5;
                        if 0 < i_var1 {
                            loop {
                                p9.write_unaligned(p7.read_unaligned());
                                i_var4 = i_var4.wrapping_sub(1);
                                p7 = p7.wrapping_add(1);
                                p9 = p9.wrapping_add(1);
                                if i_var4 == 0 {
                                    break;
                                }
                            }
                        }
                        local_1c = local_1c.wrapping_offset(i_var1 as isize);
                        puVar5 = puVar5.byte_offset((DAT_00aa7b13 & -2) as isize);
                        i_var6 = i_var6.wrapping_sub(1);
                        if i_var6 == 0 {
                            break;
                        }
                    }
                }
                if DAT_00aa7b24 != 0 {
                    FUN_004d4ec0(1);
                }
                if c_var2 == 0 {
                    FUN_004d4970();
                }
            }
            _free(rd(&raw mut DAT_00acc149) as u32 as usize as *mut core::ffi::c_void);
        }
        FUN_004d4d20((&raw mut DAT_00acc55d).byte_offset(off));
        DAT_00acebbd = rd(&raw mut DAT_00acc575);
        DAT_00acc131 = i_var3;
    }
}
