// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4984: u32;
    static mut _DAT_005201dc: f32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00485c90(
    param_1: *mut u32,
    param_2: *mut f32,
    param_3: *mut f32,
) -> u32 {
    // SAFETY: the pointers are the caller's records and the globals are the
    // game's own; reads and writes follow the original's order.
    unsafe {
        let mut param_3 = param_3;
        let mut u_var6: u32;
        let mut u_var4: u32;
        let u_var5: u32;
        let pf_var7: *mut f32;

        let a1 = param_2.add(1).read_unaligned();
        if (a1 as u32) < 0x80000001 {
            u_var6 = (a1 as i64) as u32;
            if (u_var6 as i32 as f32) != a1 {
                u_var6 = u_var6.wrapping_add(1);
            }
        } else {
            u_var6 = 0;
        }
        let b1 = param_3.add(1).read_unaligned();
        if (b1 as u32) < 0x80000001 {
            u_var4 = (b1 as i64) as u32;
            if (u_var4 as i32 as f32) != b1 {
                u_var4 = u_var4.wrapping_add(1);
            }
        } else {
            u_var4 = 0;
        }
        if u_var4 == u_var6 {
            return 0xffffffff;
        }
        let mut u5 = u_var4;
        let mut p7 = param_2;
        if (u_var4 as i32) < (u_var6 as i32) {
            u5 = u_var6;
            u_var6 = u_var4;
            p7 = param_3;
            param_3 = param_2;
        }
        u_var5 = u5;
        pf_var7 = p7;
        if (u_var5 as i32) < 1 {
            return 0xffffffff;
        }
        u_var4 = DAT_006d4984;
        if (u_var4 as i32) <= (u_var6 as i32) {
            return 0xffffffff;
        }
        let mut u_var5 = u_var5;
        if (u_var4 as i32) < (u_var5 as i32) {
            u_var5 = u_var4;
        }
        param_1.write_unaligned(u_var5);
        let f_var3: f32 =
            _DAT_005201dc / (param_3.add(1).read_unaligned() - pf_var7.add(1).read_unaligned());
        param_1.add(4).write_unaligned(
            ((param_3.read_unaligned() - pf_var7.read_unaligned()) * f_var3) as u32,
        );
        let f_var1 = param_3.add(3).read_unaligned();
        let f_var2 = pf_var7.add(3).read_unaligned();
        param_1.add(6).write_unaligned(0);
        param_1
            .add(5)
            .write_unaligned(((f_var1 - f_var2) * f_var3) as u32);
        let f_var1 = pf_var7.add(1).read_unaligned();
        let v6f = u_var6 as i32 as f32;
        let p14 = param_1.add(4).read_unaligned() as f32;
        param_1
            .add(2)
            .write_unaligned((p14 * (v6f - f_var1) + pf_var7.read_unaligned()) as u32);
        let f_var2 = pf_var7.add(3).read_unaligned();
        param_1.add(1).write_unaligned(u_var6);
        let p15 = param_1.add(5).read_unaligned() as f32;
        param_1
            .add(3)
            .write_unaligned(((v6f - f_var1) * p15 + f_var2) as u32);
        u_var6
    }
}
