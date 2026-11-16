// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732540: i32;
    static mut DAT_00732544: i32;
    static mut DAT_00732550: i32;
    static mut DAT_00732560: i32;
    static mut DAT_00732564: i32;
    static mut DAT_00732580: i32;
    static mut DAT_007329c4: i32;
    static mut DAT_007329c8: i32;
    static mut DAT_00732a14: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004187b0(param_1: i16) -> i64;
    fn FUN_00418450(param_1: i16) -> i64;
    fn FUN_00413db0(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_00413df0(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_00413d00(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_00413ca0(a: i32, b: i32) -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00483cf0(param_1: i32, param_2: i32, param_3: i32, param_4: i16) {
    if param_4 == 0 {
        return;
    }
    // SAFETY: the callees and globals are the game's own.
    unsafe {
        let l_var5 = FUN_004187b0(param_4);
        let l_var6 = FUN_00418450(param_4);
        let mut i_var1 = (l_var5 as i16) as i32;
        let neg = (l_var5 as i16) < 0;
        let f: unsafe extern "C" fn(i32, i32, i32, i32) -> i32 =
            if neg { FUN_00413df0 } else { FUN_00413db0 };
        let i_var4: i32 = if neg {
            0u32.wrapping_sub(i_var1 as u32) as i32
        } else {
            0x7fffu32.wrapping_sub(i_var1 as u32) as i32
        };
        let mut i_var3 = (l_var6 as i16) as i32;

        let local_14 = f(param_1, param_1, i_var4, i_var1);
        let u = FUN_00413ca0(i_var3, param_3);
        let local_c = f(param_1, param_2, i_var4, u as i32);
        let u = FUN_00413ca0(i_var3, param_2);
        let local_4 = f(
            param_1,
            param_3,
            i_var4,
            0u32.wrapping_sub(u as i32 as u32) as i32,
        );
        let u = FUN_00413ca0(i_var3, param_3);
        let local_18 = f(
            param_1,
            param_2,
            i_var4,
            0u32.wrapping_sub(u as i32 as u32) as i32,
        );
        let local_10 = f(param_2, param_2, i_var4, i_var1);
        let u = FUN_00413ca0(i_var3, param_1);
        let local_8 = f(param_2, param_3, i_var4, u as i32);
        let u = FUN_00413ca0(i_var3, param_2);
        let local_1c = f(param_1, param_3, i_var4, u as i32);
        let u = FUN_00413ca0(i_var3, param_1);
        i_var3 = f(
            param_2,
            param_3,
            i_var4,
            0u32.wrapping_sub(u as i32 as u32) as i32,
        );
        i_var1 = f(param_3, param_3, i_var4, i_var1);

        let a = FUN_00413d00(
            DAT_00732550,
            DAT_00732540,
            DAT_00732544,
            local_14,
            local_18,
            local_1c,
        );
        let b = FUN_00413d00(
            DAT_00732550,
            DAT_00732540,
            DAT_00732544,
            local_c,
            local_10,
            i_var3,
        );
        DAT_00732544 = FUN_00413d00(
            DAT_00732550,
            DAT_00732540,
            DAT_00732544,
            local_4,
            local_8,
            i_var1,
        );
        DAT_00732540 = b;
        DAT_00732550 = a;

        let a = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            local_14,
            local_18,
            local_1c,
        );
        let b = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            local_c,
            local_10,
            i_var3,
        );
        DAT_00732564 = FUN_00413d00(
            DAT_00732580,
            DAT_00732560,
            DAT_00732564,
            local_4,
            local_8,
            i_var1,
        );
        DAT_00732560 = b;
        DAT_00732580 = a;

        let a = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            local_14,
            local_18,
            local_1c,
        );
        let b = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            local_c,
            local_10,
            i_var3,
        );
        DAT_00732a14 = FUN_00413d00(
            DAT_007329c4,
            DAT_007329c8,
            DAT_00732a14,
            local_4,
            local_8,
            i_var1,
        );
        DAT_007329c4 = a;
        DAT_007329c8 = b;
    }
}
