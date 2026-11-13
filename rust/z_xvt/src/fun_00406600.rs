// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703be4: i32;
    static mut DAT_00732a20: i32;
    static mut DAT_0071f2e0: i32;
    static mut DAT_0070acec: i32;
    static mut DAT_0070a170: i32;
    static mut DAT_00525518: i32;
    static mut DAT_006db6d0: i32;
    static mut DAT_0070a144: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00465eb0(p1: *mut u8, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406600(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's array of at least 8 ints; the
    // globals and the callee are the game's own.
    unsafe {
        let mut i_var7 = DAT_00703be4.wrapping_sub(param_1.add(1).read_unaligned());
        param_1.add(1).write_unaligned(i_var7);
        let i_var5 = DAT_00703be4.wrapping_sub(param_1.add(3).read_unaligned());
        param_1.add(3).write_unaligned(i_var5);
        let i_var1 = DAT_00703be4.wrapping_sub(param_1.add(5).read_unaligned());
        param_1.add(5).write_unaligned(i_var1);
        let i_var2 = DAT_00703be4.wrapping_sub(param_1.add(7).read_unaligned());
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

        if -1 < i_var7 && i_var4 < DAT_00732a20 {
            if DAT_00732a20 <= i_var7 {
                i_var7 = DAT_0071f2e0;
            }
            if i_var4 < 0 {
                i_var4 = 0;
            }
            if -1 < i_var6 && i_var3 < DAT_0070acec {
                if DAT_0070acec <= i_var6 {
                    i_var6 = DAT_0070a170;
                }
                if i_var3 < 0 {
                    i_var3 = 0;
                }
                let addr = DAT_00525518
                    .wrapping_mul(i_var3)
                    .wrapping_add(i_var4.wrapping_mul(DAT_006db6d0))
                    .wrapping_add(DAT_0070a144);
                let stride = i_var3
                    .wrapping_sub(i_var6)
                    .wrapping_mul(DAT_00525518)
                    .wrapping_add(DAT_006db6d0);
                FUN_00465eb0(
                    addr as usize as *mut u8,
                    stride,
                    i_var3,
                    i_var4,
                    i_var6,
                    i_var7,
                );
            }
        }
    }
}
