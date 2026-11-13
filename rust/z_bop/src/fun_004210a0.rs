// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004210a0(param_1: *mut f32, param_2: *mut f32) {
    // SAFETY: both pointers refer to the caller's 9-float matrices; reads and
    // writes happen in the same order as the original.
    unsafe {
        let r1 = |i: usize| param_1.add(i).read_unaligned();
        let r2 = |i: usize| param_2.add(i).read_unaligned();

        let f_var1 = r2(2);
        let f_var2 = r2(5);
        let f_var3 = r1(1);
        let f_var4 = r2(8);
        let f_var5 = r2(0);
        let f_var6 = r2(6);
        let f_var7 = r2(3);
        let f_var8 = r1(5);
        let f_var9 = r2(7);
        let f_var10 = r2(1);
        let f_var11 = r1(3);
        let f_var12 = r2(4);
        let f_var13 = r2(8);
        let f_var14 = r2(5);
        let f_var15 = r2(2);
        let f_var16 = r1(3);
        let f_var17 = r2(0);
        let f_var18 = r2(6);
        let f_var19 = r2(3);
        let f_var20 = r2(1);
        let f_var21 = r1(6);
        let f_var22 = r2(7);
        let f_var23 = r2(4);
        let f_var24 = r2(5);
        let f_var25 = r1(7);
        let f_var26 = r1(6);
        let f_var27 = r2(2);
        let f_var28 = r2(8);
        let f_var29 = r2(0);
        let f_var30 = r2(6);
        let f_var31 = r1(2);
        let f_var32 = r1(1);
        let f_var33 = r2(3);

        let v = r1(1) * r2(4) + r1(2) * r2(7) + r1(0) * r2(1);
        param_1.add(1).write_unaligned(v);

        let v = r1(2) * f_var4 + f_var2 * f_var3 + r1(0) * f_var1;
        param_1.add(2).write_unaligned(v);

        let v = f_var7 * r1(4) + r1(5) * f_var6 + f_var5 * r1(3);
        param_1.add(3).write_unaligned(v);

        let v = f_var30 * f_var31 + f_var29 * r1(0) + f_var32 * f_var33;
        param_1.write_unaligned(v);

        let v = f_var15 * f_var16 + f_var14 * r1(4) + r1(5) * f_var13;
        param_1.add(5).write_unaligned(v);

        let v = f_var12 * r1(4) + f_var10 * f_var11 + f_var8 * f_var9;
        param_1.add(4).write_unaligned(v);

        let v = r1(7) * f_var19 + f_var18 * r1(8) + f_var17 * r1(6);
        param_1.add(6).write_unaligned(v);

        let v = r1(7) * f_var23 + f_var22 * r1(8) + f_var20 * f_var21;
        param_1.add(7).write_unaligned(v);

        let v = f_var28 * r1(8) + f_var26 * f_var27 + f_var24 * f_var25;
        param_1.add(8).write_unaligned(v);
    }
}
