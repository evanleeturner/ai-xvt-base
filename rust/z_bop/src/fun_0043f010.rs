// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9670: i32;
    static mut DAT_009e9683: i8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9a77: i8;
    static mut DAT_00a08374: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00440760(param_1: i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043f010() {
    // SAFETY: all globals and the callee belong to the game; pointers are
    // computed and dereferenced exactly as the original does.
    unsafe {
        let base3 = (&raw mut DAT_009e9683) as i32;
        let base0 = (&raw mut DAT_009e9670) as i32;
        let base7 = (&raw mut DAT_009e9a77) as i32;
        let off = DAT_009ecc34.wrapping_mul(0x5bd);

        if (base3.wrapping_add(off) as *const i8).read() != 0 {
            return;
        }
        let mut local_int: i32 = (base0.wrapping_add(off) as *const i32).read_unaligned();
        let mut i_var3 = local_int;
        let b_var2: u8;
        if i_var3 == -1 {
            b_var2 = 0;
        } else {
            let c = (i_var3
                .wrapping_mul(0x23)
                .wrapping_add(3)
                .wrapping_add(DAT_009a1fe8) as *const i8)
                .read();
            b_var2 = if c == 1 || c == 2 || c == 3 || c == 0x0e || c == 4 {
                1
            } else {
                0
            };
        }
        if b_var2 == 0 {
            i_var3 = i_var3.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
            if (i_var3.wrapping_add(3) as *const i8).read() != 5 {
                let v = (base7.wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd)) as *const i8).read();
                if v == 0 || v == 0x13 {
                    let u_var4 = DAT_00a08374 as u32;
                    local_int = (i_var3.wrapping_add(0x1f) as *const i32).read_unaligned();
                    local_int = (local_int.wrapping_add(0xa9) as *const i32).read_unaligned();
                    if ((local_int.wrapping_add(0x1e4) as *const u8).read() & 1) == 0 {
                        FUN_00440760(u_var4.wrapping_add(0x6d) as i32, 0);
                        FUN_00440760(u_var4.wrapping_add(0x6e) as i32, 0);
                    }
                    local_int = (base0.wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd))
                        as *const i32)
                        .read_unaligned();
                    local_int = (local_int
                        .wrapping_mul(0x23)
                        .wrapping_add(0x1f)
                        .wrapping_add(DAT_009a1fe8) as *const i32)
                        .read_unaligned();
                    local_int = (local_int.wrapping_add(0xa9) as *const i32).read_unaligned();
                    if ((local_int.wrapping_add(0x1e3) as *const u8).read() & 1) == 0 {
                        FUN_00440760(u_var4.wrapping_add(0x6c) as i32, 0);
                    }
                }
            }
        }
    }
}
