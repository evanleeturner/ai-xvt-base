// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: i32;
    static mut DAT_009d80c9: i8;
    static mut DAT_00bb7372: i8;
    static mut DAT_00bb737b: i8;
    static mut DAT_009e9670: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9681: i8;
    static mut DAT_00a0afb8: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9300(param_1: i32, param_2: i32) -> u32;
    fn FUN_004a9380(param_1: i32) -> u8;
    fn FUN_0042d120(
        param_1: *mut i8,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: u32,
        param_6: u32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042f110() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers computed from them are read where the original reads.
    unsafe {
        if DAT_00523410 == 0 && DAT_009d80c9 != 0 && DAT_00bb7372 != 0 && DAT_00bb737b != 0 {
            let off = DAT_009ecc34.wrapping_mul(0x5bd) as isize;
            let mut i_var1: i32 = (&raw mut DAT_009e9670)
                .cast::<u8>()
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            if i_var1 == -1 {
                let u_var2 = FUN_004a9300(0x13, 0x100);
                if u_var2 != 0 {
                    FUN_004a9380(0x13);
                    return;
                }
            } else {
                let a = i_var1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8);
                let b = (a as usize as *const i32).read_unaligned();
                i_var1 = (b.wrapping_add(0xa9) as usize as *const i32).read_unaligned();
                if (i_var1.wrapping_add(0x246) as usize as *const i8).read() == 1 {
                    let c = (&raw mut DAT_009e9681).byte_offset(off).read();
                    if c == 1 {
                        let u_var2 = FUN_004a9300(0x13, 0x100);
                        if u_var2 != 0 {
                            FUN_004a9380(0x13);
                            return;
                        }
                    } else {
                        let mut i_var3: i32 = 0x7f;
                        if DAT_00bb737b < 10 {
                            i_var3 = ((DAT_00bb737b as i32 as u32).wrapping_mul(0xd)) as i32;
                        }
                        let s =
                            (i_var1.wrapping_add(0x248) as usize as *const i16).read_unaligned();
                        if s == 0 {
                            let u_var2 = FUN_004a9300(0x13, 0x100);
                            if u_var2 != 0 {
                                FUN_004a9380(0x13);
                            }
                        } else {
                            let u_var2 = FUN_004a9300(0x13, 0x100);
                            if u_var2 == 0 {
                                FUN_0042d120(
                                    &raw mut DAT_00a0afb8,
                                    1,
                                    1,
                                    0x7d,
                                    (i_var3 >> 2) as u32,
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
