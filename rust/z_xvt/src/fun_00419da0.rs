// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005200ac: f32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00419da0(
    param_1: *mut f32,
    param_2: *mut f32,
    param_3: *mut f32,
    param_4: *mut f32,
    param_5: *mut f32,
) -> i32 {
    // SAFETY: all pointers are the caller's float vectors (3 floats each,
    // param_5 one float); the global is the game's own memory.
    unsafe {
        let mut local_8: f32 = (param_3.read() - param_2.read()) * param_1.read()
            + (param_3.add(1).read() - param_2.add(1).read()) * param_1.add(1).read()
            + (param_3.add(2).read() - param_2.add(2).read()) * param_1.add(2).read();
        let mut local_4: f32 = (param_4.add(2).read() - param_2.add(2).read())
            * param_1.add(2).read()
            + (param_4.read() - param_2.read()) * param_1.read()
            + (param_4.add(1).read() - param_2.add(1).read()) * param_1.add(1).read();

        if ((local_8 as i32) < 0x41200000) && ((local_8 as i64 as u32) < 0xc1200000u32) {
            local_8 = 0.0;
        }
        if ((local_4 as i32) < 0x41200000) && ((local_4 as i64 as u32) < 0xc1200000u32) {
            local_4 = 0.0;
        }

        if local_8.abs() == 0.0 {
            param_5.write(0.0);
            return 1;
        }
        if local_4.abs() == 0.0 {
            param_5.write(1.0);
            return 1;
        }

        if (0x80000000u32 < (local_8 as i64 as u32)) && (0 < (local_4 as i32)) {
            let f_var2: f32 = local_8 / (local_4 - local_8);
            param_5.write(local_8);
            let b_var1: i32 = (f_var2 < _DAT_005200ac) as i32;
            param_5.write(f_var2);
            if b_var1 != 0 {
                param_5.write(-param_5.read());
            }
            return 1;
        }
        if (0x80000000u32 < (local_4 as i64 as u32)) && (0 < (local_8 as i32)) {
            let f_var2: f32 = local_8 / (local_8 - local_4);
            param_5.write(local_8);
            let b_var1: i32 = (f_var2 < _DAT_005200ac) as i32;
            param_5.write(f_var2);
            if b_var1 != 0 {
                param_5.write(-param_5.read());
            }
            return 1;
        }
        0
    }
}
