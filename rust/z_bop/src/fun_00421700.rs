// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d1260: u16;
    static mut DAT_00a07cd0: i16;
    static mut DAT_009ed230: i16;
    static mut DAT_009d8c2c: i16;
    static mut DAT_009d77f0: i16;
    static mut DAT_009a8d50: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009d77c4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00486470(
        param_1: *mut u8,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421700(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's array of at least 8 ints; the
    // globals and callee are the game's own.
    unsafe {
        let mut i_var7 = (DAT_009d1260 as i32).wrapping_sub(param_1.add(1).read_unaligned());
        param_1.add(1).write_unaligned(i_var7);
        let i_var5 = (DAT_009d1260 as i32).wrapping_sub(param_1.add(3).read_unaligned());
        param_1.add(3).write_unaligned(i_var5);
        let i_var1 = (DAT_009d1260 as i32).wrapping_sub(param_1.add(5).read_unaligned());
        param_1.add(5).write_unaligned(i_var1);
        let i_var2 = (DAT_009d1260 as i32).wrapping_sub(param_1.add(7).read_unaligned());
        param_1.add(7).write_unaligned(i_var2);
        let mut i_var6 = param_1.read_unaligned();
        let mut i_var3 = i_var6;
        if param_1.add(2).read_unaligned() < i_var6 {
            i_var3 = param_1.add(2).read_unaligned();
        }
        if param_1.add(4).read_unaligned() < i_var3 {
            i_var3 = param_1.add(4).read_unaligned();
        }
        if param_1.add(6).read_unaligned() < i_var3 {
            i_var3 = param_1.add(6).read_unaligned();
        }
        if i_var6 < param_1.add(2).read_unaligned() {
            i_var6 = param_1.add(2).read_unaligned();
        }
        if i_var6 < param_1.add(4).read_unaligned() {
            i_var6 = param_1.add(4).read_unaligned();
        }
        if i_var6 < param_1.add(6).read_unaligned() {
            i_var6 = param_1.add(6).read_unaligned();
        }
        let mut i_var4 = i_var7;
        if i_var5 < i_var7 {
            i_var4 = i_var5;
        }
        if i_var1 < i_var4 {
            i_var4 = i_var1;
        }
        if i_var2 < i_var4 {
            i_var4 = i_var2;
        }
        if i_var7 < i_var5 {
            i_var7 = i_var5;
        }
        if i_var7 < i_var1 {
            i_var7 = i_var1;
        }
        if i_var7 < i_var2 {
            i_var7 = i_var2;
        }
        i_var3 = i_var3.wrapping_sub(2);
        i_var4 = i_var4.wrapping_sub(2);
        i_var6 = i_var6.wrapping_add(2);
        i_var7 = i_var7.wrapping_add(2);
        if -1 < i_var7 && i_var4 < DAT_00a07cd0 as i32 {
            if DAT_00a07cd0 as i32 <= i_var7 {
                i_var7 = DAT_009ed230 as i32;
            }
            if i_var4 < 0 {
                i_var4 = 0;
            }
            if -1 < i_var6 && i_var3 < DAT_009d8c2c as i32 {
                if DAT_009d8c2c as i32 <= i_var6 {
                    i_var6 = DAT_009d77f0 as i32;
                }
                if i_var3 < 0 {
                    i_var3 = 0;
                }
                FUN_00486470(
                    i_var4
                        .wrapping_mul(DAT_009a8d50)
                        .wrapping_add(i_var3.wrapping_mul(DAT_005233d8))
                        .wrapping_add(DAT_009d77c4) as usize as *mut u8,
                    i_var3
                        .wrapping_sub(i_var6)
                        .wrapping_mul(DAT_005233d8)
                        .wrapping_add(DAT_009a8d50),
                    i_var3,
                    i_var4,
                    i_var6,
                    i_var7,
                );
            }
        }
    }
}
