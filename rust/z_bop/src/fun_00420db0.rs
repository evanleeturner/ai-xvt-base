// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00420db0(param_1: i32, param_2: *mut f32, param_3: i32) {
    // SAFETY: param_1 and param_3 are addresses of the game's own structures,
    // param_2 points at six floats; reads, writes and calls keep the original's order.
    unsafe {
        ((param_1.wrapping_add(0x10)) as *mut f32).write_unaligned(param_2.read_unaligned());
        ((param_1.wrapping_add(0x14)) as *mut f32).write_unaligned(param_2.add(1).read_unaligned());
        ((param_1.wrapping_add(0x18)) as *mut f32).write_unaligned(param_2.add(2).read_unaligned());
        FUN_00420fe0(
            param_1.wrapping_add(0x10) as *mut f32,
            param_3.wrapping_add(0xc) as *mut f32,
        );
        ((param_1.wrapping_add(0x1c)) as *mut f32).write_unaligned(param_2.add(3).read_unaligned());
        ((param_1.wrapping_add(0x20)) as *mut f32).write_unaligned(param_2.add(4).read_unaligned());
        ((param_1.wrapping_add(0x24)) as *mut f32).write_unaligned(param_2.add(5).read_unaligned());
        FUN_00420fe0(
            param_1.wrapping_add(0x1c) as *mut f32,
            param_3.wrapping_add(0xc) as *mut f32,
        );
    }
}
