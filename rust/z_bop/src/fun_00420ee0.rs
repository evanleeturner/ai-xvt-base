// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a081ec: i32;
    static mut _DAT_005180d4: f32;
    static mut DAT_009d682c: i16;
    static mut DAT_009a1fe4: i16;
    static mut DAT_009d7680: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00420ee0(param_1: *mut f32, param_2: *mut f32, param_3: *mut f32) {
    // SAFETY: the pointers are the caller's float arrays, read and written
    // where the original does; the globals and callee are the game's own.
    unsafe {
        let mut loc: [f32; 4] = [0.0; 4];
        loc[0] = param_2.read();
        loc[1] = param_2.add(1).read();
        loc[2] = param_2.add(2).read();
        loc[3] = (DAT_00a081ec as f32 / param_3.add(2).read()) * _DAT_005180d4;
        FUN_00420fe0(loc.as_mut_ptr(), param_3.add(3));
        let f_var1 = param_3.read();
        let f_var2 = param_3.add(1).read();
        loc[3] = loc[3] / (param_3.add(2).read() + loc[2] + _DAT_005180d4);
        param_1.add(2).write(loc[3]);
        param_1.write(loc[3] * (f_var1 + loc[0]));
        let p2 = param_1.add(2).read();
        param_1.add(1).write(p2 * (f_var2 + loc[1]));
        let v0 = (DAT_009d682c >> 1) as f32 + param_1.read();
        param_1.write(v0);
        let sum = ((DAT_009a1fe4 >> 1) as i32 as u32).wrapping_add(DAT_009d7680 as u32) as i32;
        let v1 = sum as f32 + param_1.add(1).read();
        param_1.add(1).write(v1);
    }
}
