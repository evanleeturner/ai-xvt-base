// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044a780(a: i32, b: i32) -> i32;
    fn __ftol() -> i64;
    fn FUN_004187b0(a: i16) -> i64;
    fn FUN_00418450(a: i16) -> i64;
    fn FUN_00413db0(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_00413df0(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_00413ca0(a: i32, b: i32) -> i64;
    fn FUN_00413d00(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00483950(
    param_1: i16,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    // SAFETY: the globals and callees are the game's own; __ftol consumes the
    // x87 value the original left on the FPU stack.
    unsafe {
        DAT_0070ab18 = param_4;
        DAT_0070ab14 = param_5;
        DAT_0070ac20 = param_6;
        let i_var1 = FUN_0044a780(param_2, param_3);
        if i_var1 == 0 {
            return;
        }
        let mut i_var1 = __ftol() as i32;
        let mut i_var4 = __ftol() as i32;
        let i_var5 = __ftol() as i32;
        let i_var2 = (FUN_004187b0(param_1) as i16) as i32;
        let i_var6 = (FUN_00418450(param_1) as i16) as i32;

        let neg = i_var2 < 0;
        let i_var3: i32 = if neg {
            (i_var2 as u32).wrapping_neg() as i32
        } else {
            0x7fffu32.wrapping_sub(i_var2 as u32) as i32
        };
        let f: unsafe extern "C" fn(i32, i32, i32, i32) -> i32 =
            if neg { FUN_00413df0 } else { FUN_00413db0 };

        let local_14 = f(i_var1, i_var1, i_var3, i_var2);
        let u = FUN_00413ca0(i_var6, i_var5) as i32;
        let local_c = f(i_var1, i_var4, i_var3, u);
        let u = FUN_00413ca0(i_var6, i_var4) as i32;
        let local_4 = f(i_var1, i_var5, i_var3, (u as u32).wrapping_neg() as i32);
        let u = FUN_00413ca0(i_var6, i_var5) as i32;
        let local_18 = f(i_var1, i_var4, i_var3, (u as u32).wrapping_neg() as i32);
        let local_10 = f(i_var4, i_var4, i_var3, i_var2);
        let u = FUN_00413ca0(i_var6, i_var1) as i32;
        let local_8 = f(i_var4, i_var5, i_var3, u);
        let u = FUN_00413ca0(i_var6, i_var4) as i32;
        let local_1c = f(i_var1, i_var5, i_var3, u);
        let u = FUN_00413ca0(i_var6, i_var1) as i32;
        i_var1 = f(i_var4, i_var5, i_var3, (u as u32).wrapping_neg() as i32);
        i_var4 = f(i_var5, i_var5, i_var3, i_var2);

        let l = __ftol() as i32;
        let i_var2 = (param_4 as u32).wrapping_sub(l as u32) as i32;
        let l = __ftol() as i32;
        let i_var6 = (param_5 as u32).wrapping_add(l as u32) as i32;
        let l = __ftol() as i32;
        let i_var3 = (param_6 as u32).wrapping_sub(l as u32) as i32;

        let i_var5 = FUN_00413d00(i_var2, i_var6, i_var3, local_14, local_18, local_1c);
        let i_var1 = FUN_00413d00(i_var2, i_var6, i_var3, local_c, local_10, i_var1);
        let i_var4 = FUN_00413d00(i_var2, i_var6, i_var3, local_4, local_8, i_var4);

        let l7 = __ftol() as i32;
        let l9 = __ftol() as i32;
        let l10 = __ftol() as i32;
        DAT_0070ab18 = (l10 as u32).wrapping_add(i_var5 as u32) as i32;
        DAT_0070ab14 = (i_var1 as u32).wrapping_sub(l7 as u32) as i32;
        DAT_0070ac20 = (i_var4 as u32).wrapping_add(l9 as u32) as i32;
    }
}
