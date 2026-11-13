// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004211d0(param_1: *mut f32, param_2: *mut f32) {
    // SAFETY: both pointers are the caller's 3x3 float matrices; reads and
    // writes happen in the original's order.
    unsafe {
        let f1 = param_2.read();
        let f2 = param_2.add(3).read();
        let f3 = param_2.add(6).read();
        let f4 = param_2.add(1).read();
        let f5 = param_2.add(7).read();
        let f6 = param_2.add(4).read();
        let f7 = param_2.add(1).read();
        let f8 = param_1.add(1).read();
        let f9 = param_2.add(4).read();
        let f10 = param_2.add(7).read();
        let f11 = param_2.add(4).read();
        let f12 = param_2.add(7).read();
        let f13 = param_2.add(1).read();
        let f14 = param_1.add(2).read();
        let f15 = param_1.read();
        let f16 = param_2.add(2).read();
        let f17 = param_2.add(5).read();
        let f18 = param_1.add(3).read();
        let f19 = param_2.add(8).read();
        let f20 = param_1.add(1).read();
        let f21 = param_2.add(2).read();
        let f22 = param_2.add(5).read();
        let f23 = param_1.add(4).read();
        let f24 = param_2.add(8).read();
        let f25 = param_2.add(5).read();
        let f26 = param_1.add(5).read();
        let f27 = param_2.add(8).read();
        let f28 = param_2.add(2).read();
        let f29 = param_1.add(2).read();
        let f30 = param_2.read();
        let f31 = param_2.add(6).read();
        let f32 = param_2.add(3).read();
        let f33 = param_1.add(3).read();

        let v = param_2.add(3).read() * param_1.add(4).read()
            + param_2.add(6).read() * param_1.add(7).read()
            + param_2.read() * param_1.add(1).read();
        param_1.add(1).write(v);

        let v =
            f3 * param_1.add(8).read() + param_1.add(5).read() * f2 + f1 * param_1.add(2).read();
        param_1.add(2).write(v);

        let v = param_1.add(3).read() * f6 + param_1.add(6).read() * f5 + param_1.read() * f4;
        param_1.add(3).write(v);

        let v = f31 * param_1.add(6).read() + f30 * param_1.read() + f32 * f33;
        param_1.write(v);

        let v = f13 * f14 + f12 * param_1.add(8).read() + param_1.add(5).read() * f11;
        param_1.add(5).write(v);

        let v = f10 * param_1.add(7).read() + f9 * param_1.add(4).read() + f7 * f8;
        param_1.add(4).write(v);

        let v = param_1.add(6).read() * f19 + f17 * f18 + f15 * f16;
        param_1.add(6).write(v);

        let v = param_1.add(7).read() * f24 + f22 * f23 + f20 * f21;
        param_1.add(7).write(v);

        let v = f28 * f29 + f27 * param_1.add(8).read() + f25 * f26;
        param_1.add(8).write(v);
    }
}
