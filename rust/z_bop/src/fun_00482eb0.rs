// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00860: u16;
    static mut DAT_009e9689: [u8; 1];
    static mut DAT_009ecc34: u32;
    static mut DAT_00a081ec: u32;
    static mut DAT_005233ec: u32;
    static mut DAT_009a7bac: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009e9683: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004830d0(
        param_1: u32,
        param_2: u32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut u32,
    );
    fn FUN_00483030(param_1: u32) -> i32;
    fn FUN_004aedf0(param_1: u32, param_2: u32) -> i32;
    fn FUN_00437860(param_1: i32, param_2: i32, param_3: u32, param_4: u32, param_5: u8);
    fn FUN_00497e00(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00482eb0(param_1: u16, param_2: u16, param_3: u8) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the table pointers are indexed as the original does.
    unsafe {
        if param_1 == 0xffff || DAT_00a00860 != 0 {
            return;
        }
        let idx = DAT_009ecc34.wrapping_mul(0x5bd) as usize;
        if (&raw mut DAT_009e9689).cast::<u8>().byte_add(idx).read() == 0 {
            return;
        }
        let mut local_4: i32 = 0;
        let mut local_8: i32 = 0;
        let mut local_c: u32 = 0;
        let u_var3 = param_1 as u32;
        FUN_004830d0(
            u_var3,
            param_2 as u32,
            &mut local_4,
            &mut local_8,
            &mut local_c,
        );
        if 0 < (local_c as i32) {
            let mut i_var1: i32;
            if param_2 == 0xffff {
                i_var1 = FUN_00483030(u_var3);
            } else {
                let addr = u_var3
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_009a1fe8);
                let b = (addr as usize as *const u8).read();
                i_var1 = FUN_004aedf0(b as u32, param_2 as u32);
            }
            let prod = (i_var1 as i64).wrapping_mul(DAT_00a081ec as i64) as u64 & 0xffff_ffff;
            i_var1 = (prod / (local_c as u64)) as i32;
            let i_var4: i32 = if DAT_005233ec == 0x13 { 4 } else { 8 };
            let i_var2 = ((DAT_009a7bac >> 2).wrapping_add(DAT_009a7bac >> 1)) as i32;
            if i_var1 < i_var4 {
                i_var1 = i_var4;
            }
            if i_var2 < i_var1 {
                i_var1 = i_var2;
            }
            let u_var3 = i_var1.wrapping_add(4) as u32;
            let half = (u_var3 as i32) / 2;
            if (&raw mut DAT_009e9683).cast::<u8>().byte_add(idx).read() != 0 {
                FUN_00437860(
                    local_4.wrapping_sub(half),
                    local_8.wrapping_sub(half),
                    u_var3,
                    u_var3,
                    param_3,
                );
                return;
            }
            FUN_00497e00(
                local_4.wrapping_sub(half),
                local_8.wrapping_sub(half),
                u_var3,
                u_var3,
                param_3 as u32,
                local_c,
            );
        }
    }
}
