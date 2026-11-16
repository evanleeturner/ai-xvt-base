// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525554: i32;
    static mut DAT_00525550: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0070a189: i8;
    static mut DAT_00740a80: [u16; 1];
    static mut DAT_0085e792: i8;
    static mut DAT_0085e79b: i8;
    static mut DAT_0085e791: i8;
    static mut DAT_0085e79a: i8;
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_006d4988: i32;
    static mut DAT_0073ba30: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040a4b0(param_1: u32, param_2: u32) -> u32;
    fn FUN_0040a640(param_1: u32, param_2: *mut i32) -> i32;
    fn FUN_00439830(param_1: u32, param_2: i32) -> u32;
    fn FUN_00490a50(
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
pub unsafe extern "C" fn FUN_0040a1a0(param_1: u32, param_2: u32, param_3: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // addresses computed from game data are read where the original reads.
    unsafe {
        if DAT_00525554 == 0 {
            if DAT_00525550 != 0 {
                return 0;
            }
        } else {
            if DAT_00525550 == 0 {
                return 0;
            }
            if DAT_00525554 == 2 {
                return 0;
            }
        }
        if DAT_0071ecf4 != param_3 {
            return 0;
        }
        if DAT_0070a189 == 0 {
            return 0;
        }
        if (&raw mut DAT_00740a80)
            .cast::<u16>()
            .wrapping_add(param_1 as usize)
            .read()
            == 0
        {
            return 0;
        }
        if param_2 == 0xffffffff {
            if DAT_0085e792 == 0 {
                return 0;
            }
            if DAT_0085e79b == 0 {
                return 0;
            }
        } else {
            if DAT_0085e791 == 0 {
                return 0;
            }
            if DAT_0085e79a == 0 {
                return 0;
            }
        }
        if 0x52 < param_1 && param_1 < 0x57 {
            let p = (&raw mut DAT_0071b730)
                .cast::<u8>()
                .wrapping_add((param_3 as u32).wrapping_mul(0x5bd) as usize);
            let i_var2 = p.cast::<i32>().read_unaligned();
            if i_var2 == -1 {
                return 0;
            }
            let a = (i_var2 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_006d4988 as u32);
            let c_var1 = (a as *const i8).read();
            if c_var1 != 1 && c_var1 != 2 {
                return 0;
            }
        }
        let mut local_4: u32 = FUN_0040a4b0(param_2, param_1);
        if local_4 == 0 {
            return 1;
        }
        let i_var2 = FUN_0040a640(param_2, (&raw mut local_4).cast::<i32>());
        let mut u_var5: u32 = 0x7c;
        if local_4 < 0x7d {
            u_var5 = local_4;
        }
        let mut done = false;
        let mut i_var4: u32 = 0;
        if param_2 == 0xffffffff {
            done = true;
        } else {
            i_var4 = param_2.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32);
            let u_var3 = (i_var4.wrapping_add(0x1b) as *const i32).read_unaligned();
            if u_var3 == DAT_0071ecf4 {
                done = true;
            }
        }
        if done {
            u_var5 = (0u32.wrapping_sub((param_1 == 0x25) as u32) & 2).wrapping_add(0x7d);
        } else {
            let i_var4b = (i_var4.wrapping_add(0x1f) as *const i32).read_unaligned();
            if i_var4b != 0 {
                let u_tmp = ((i_var4b as u32).wrapping_add(0x83) as *const u16).read_unaligned();
                let a = (u_tmp as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1b)
                    .wrapping_add(DAT_006d4988 as u32);
                let u_var3 = (a as *const i32).read_unaligned();
                if u_var3 == DAT_0071ecf4 {
                    u_var5 = 0x7d;
                }
            }
        }
        let u_var3 = FUN_00439830(param_1, 0x100);
        if u_var3 != 0 && 0x47 < param_1 && param_1 < 0x50 {
            return 0;
        }
        FUN_00490a50(
            (&raw mut DAT_0073ba30)
                .cast::<i8>()
                .wrapping_add(param_1.wrapping_mul(6) as usize),
            1,
            0,
            u_var5,
            local_4,
            i_var2,
        );
        1
    }
}
