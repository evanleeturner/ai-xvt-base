// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004150a0(param_1: *mut f32, param_2: *mut f32) {
    // SAFETY: both pointers refer to caller-owned arrays of nine floats;
    // reads and writes happen in the original's order.
    unsafe {
        let v0 = param_1.read_unaligned();
        let v1 = param_1.add(1).read_unaligned();
        let v2 = param_1.add(2).read_unaligned();
        let v3 = param_1.add(3).read_unaligned();
        let v4 = param_1.add(4).read_unaligned();
        let v5 = param_1.add(5).read_unaligned();
        let v6 = param_1.add(6).read_unaligned();
        let v7 = param_1.add(7).read_unaligned();
        let v8 = param_1.add(8).read_unaligned();

        let p = |i: usize| -> f32 { param_2.add(i).read_unaligned() };

        param_1
            .add(1)
            .write_unaligned(p(1) * v0 + v1 * p(4) + v2 * p(7));
        param_1
            .add(2)
            .write_unaligned(p(2) * v0 + v2 * p(8) + p(5) * v1);
        param_1
            .add(3)
            .write_unaligned(p(3) * v4 + v5 * p(6) + p(0) * v3);
        param_1
            .add(0)
            .write_unaligned(p(6) * v2 + p(0) * v0 + v1 * p(3));
        param_1
            .add(5)
            .write_unaligned(p(2) * v3 + p(5) * v4 + v5 * p(8));
        param_1
            .add(4)
            .write_unaligned(p(4) * v4 + p(1) * v3 + v5 * p(7));
        param_1
            .add(6)
            .write_unaligned(v7 * p(3) + p(6) * v8 + p(0) * v6);
        param_1
            .add(7)
            .write_unaligned(v7 * p(4) + p(7) * v8 + p(1) * v6);
        param_1
            .add(8)
            .write_unaligned(p(8) * v8 + v6 * p(2) + p(5) * v7);
    }
}
