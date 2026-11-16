// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523414: i32;
    static mut DAT_00523410: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d80c9: i8;
    static mut DAT_00a0fe80: [i16; 1];
    static mut DAT_00bb7371: i8;
    static mut DAT_00bb7372: i8;
    static mut DAT_00bb737a: i8;
    static mut DAT_00bb737b: i8;
    static mut DAT_009e9670: [i8; 1];
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a0adf0: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9380(param_1: i32);
    fn FUN_0042e810(param_1: u32, param_2: u32) -> u32;
    fn FUN_0042e9a0(param_1: u32, param_2: *mut i32) -> i32;
    fn FUN_004a9300(param_1: u32, param_2: i32) -> u32;
    fn FUN_0042d120(
        param_1: *mut i8,
        param_2: i32,
        param_3: i32,
        param_4: u32,
        param_5: u32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042e4d0(param_1: u32, param_2: u32, param_3: i32) -> u32 {
    // SAFETY: all globals and callees belong to the game; pointers computed
    // from game tables are read where the original reads them.
    unsafe {
        if DAT_00523414 == 0 {
            if DAT_00523410 != 0 {
                return 0;
            }
        } else {
            if DAT_00523410 == 0 {
                return 0;
            }
            if DAT_00523414 == 2 {
                return 0;
            }
        }
        if param_3 != DAT_009ecc34 {
            return 0;
        }
        if DAT_009d80c9 == 0 {
            return 0;
        }
        if (&raw mut DAT_00a0fe80)
            .cast::<i16>()
            .add(param_1 as usize)
            .read()
            == 0
        {
            return 0;
        }
        if param_2 == 0xffff_ffff {
            if DAT_00bb7372 == 0 {
                return 0;
            }
            if DAT_00bb737b == 0 {
                return 0;
            }
        } else {
            if DAT_00bb7371 == 0 {
                return 0;
            }
            if DAT_00bb737a == 0 {
                return 0;
            }
        }
        let mut i_var2: i32;
        if 0x55 < param_1 && param_1 < 0x5a {
            i_var2 = (&raw mut DAT_009e9670)
                .cast::<i8>()
                .add((param_3 as u32).wrapping_mul(0x5bd) as usize)
                .cast::<i32>()
                .read_unaligned();
            if i_var2 == -1 {
                return 0;
            }
            let addr = (i_var2 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_009a1fe8 as u32);
            let c_var1 = (addr as usize as *const i8).read();
            if c_var1 != 1 && c_var1 != 2 {
                return 0;
            }
        }
        if param_1 == 0x51 {
            FUN_004a9380(0x50);
        }
        if param_1 == 0x54 {
            FUN_004a9380(0x53);
        }
        let mut local_4: u32 = FUN_0042e810(param_2, param_1);
        if local_4 != 0 {
            i_var2 = FUN_0042e9a0(param_2, (&raw mut local_4).cast::<i32>());
            let mut u_var5: u32 = 0x7c;
            if local_4 < 0x7d {
                u_var5 = local_4;
            }
            let mut i_var4: i32 = 0;
            let mut tmp: i32 = 0;
            if param_2 != 0xffff_ffff {
                i_var4 = param_2.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32) as i32;
                tmp = (i_var4.wrapping_add(0x1b) as u32 as usize as *const i32).read_unaligned();
            }
            if param_2 == 0xffff_ffff || tmp == DAT_009ecc34 {
                u_var5 = (0u32.wrapping_sub((param_1 == 0x25) as u32) & 2).wrapping_add(0x7d);
            } else {
                i_var4 = (i_var4.wrapping_add(0x1f) as u32 as usize as *const i32).read_unaligned();
                if i_var4 != 0 {
                    let us =
                        (i_var4.wrapping_add(0x85) as u32 as usize as *const u16).read_unaligned();
                    let a = (us as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(0x1b)
                        .wrapping_add(DAT_009a1fe8 as u32);
                    tmp = (a as usize as *const i32).read_unaligned();
                    if tmp == DAT_009ecc34 {
                        u_var5 = 0x7d;
                    }
                }
            }
            let u_var3 = FUN_004a9300(param_1, 0x100);
            if u_var3 != 0 && 0x47 < param_1 && param_1 < 0x50 {
                return 0;
            }
            FUN_0042d120(
                (&raw mut DAT_00a0adf0)
                    .cast::<i8>()
                    .add(param_1.wrapping_mul(6) as usize),
                1,
                0,
                u_var5,
                local_4,
                i_var2,
            );
        }
        1
    }
}
