// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004151d0(param_1: *mut f32, param_2: *mut f32) {
    // SAFETY: both pointers refer to the caller's 9-float matrices; reads and
    // writes happen in the same order as the original.
    unsafe {
        let q = |i: usize| -> f32 { param_2.add(i).read_unaligned() };
        let p = |i: usize| -> f32 { param_1.add(i).read_unaligned() };

        let o0 = p(0);
        let o1 = p(1);
        let o2 = p(2);
        let o3 = p(3);
        let o4 = p(4);
        let o5 = p(5);

        let v = p(1) * q(0) + q(3) * p(4) + q(6) * p(7);
        param_1.add(1).write_unaligned(v);

        let v = p(2) * q(0) + q(6) * p(8) + p(5) * q(3);
        param_1.add(2).write_unaligned(v);

        let v = q(1) * p(0) + p(3) * q(4) + p(6) * q(7);
        param_1.add(3).write_unaligned(v);

        let v = q(6) * p(6) + q(0) * p(0) + q(3) * o3;
        param_1.add(0).write_unaligned(v);

        let v = q(7) * p(8) + p(5) * q(4) + q(1) * o2;
        param_1.add(5).write_unaligned(v);

        let v = q(4) * p(4) + q(7) * p(7) + q(1) * o1;
        param_1.add(4).write_unaligned(v);

        let v = q(2) * o0 + p(6) * q(8) + q(5) * o3;
        param_1.add(6).write_unaligned(v);

        let v = q(5) * o4 + p(7) * q(8) + o1 * q(2);
        param_1.add(7).write_unaligned(v);

        let v = q(8) * p(8) + q(5) * o5 + q(2) * o2;
        param_1.add(8).write_unaligned(v);
    }
}
