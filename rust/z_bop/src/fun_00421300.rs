// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005180c4: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sinf(x: f32) -> f32;
    fn cosf(x: f32) -> f32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421300(param_1: *mut f32, param_2: *mut f32) {
    // SAFETY: param_1 and param_2 point at the caller's float arrays; the
    // global and the math functions are the game's own.
    unsafe {
        let f_var5 = param_2.read();
        let f_var6 = param_2.add(1).read();
        let f_var7 = param_2.add(2).read();
        let angle = param_2.add(3).read();
        let f_var8 = sinf(angle);
        let f_var9 = cosf(param_2.add(3).read());
        let f_var10 = DAT_005180c4 - f_var9;
        let f_var1 = f_var10 * f_var6 * f_var5;
        let f_var2 = f_var10 * f_var7;
        let f_var3 = f_var10 * f_var7 * f_var5;
        let f_var4 = f_var2 * f_var6;
        param_1.write(f_var10 * f_var5 * f_var5 + f_var9);
        param_1.add(1).write(f_var1 + f_var8 * f_var7);
        param_1.add(3).write(f_var1 - f_var8 * f_var7);
        param_1.add(2).write(f_var3 - f_var8 * f_var6);
        param_1.add(4).write(f_var6 * (f_var10 * f_var6) + f_var9);
        param_1.add(5).write(f_var4 + f_var8 * f_var5);
        param_1.add(6).write(f_var3 + f_var8 * f_var6);
        param_1.add(8).write(f_var7 * f_var2 + f_var9);
        param_1.add(7).write(f_var4 - f_var8 * f_var5);
    }
}
