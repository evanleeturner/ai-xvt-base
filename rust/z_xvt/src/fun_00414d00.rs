// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00414d00(param_1: i32, param_2: *mut u32, param_3: i32) {
    let p1 = param_1 as usize as *mut u8;
    let m = param_3 as usize as *const u8;
    // SAFETY: the caller supplies a destination block at param_1, a source
    // vector at param_2 and a matrix at param_3; all are read and written at
    // the same offsets and in the same order as the original.
    unsafe {
        let rm = |o: usize| -> f32 { m.byte_add(o).cast::<f32>().read_unaligned() };

        p1.byte_add(0x10)
            .cast::<u32>()
            .write_unaligned(param_2.read_unaligned());
        let f1 = p1.byte_add(0x10).cast::<f32>().read_unaligned();
        p1.byte_add(0x14)
            .cast::<u32>()
            .write_unaligned(param_2.add(1).read_unaligned());
        let f2 = p1.byte_add(0x14).cast::<f32>().read_unaligned();
        p1.byte_add(0x18)
            .cast::<u32>()
            .write_unaligned(param_2.add(2).read_unaligned());
        let f3 = p1.byte_add(0x18).cast::<f32>().read_unaligned();

        let v = rm(0xc) * f1 + rm(0x18) * f2 + rm(0x24) * f3;
        p1.byte_add(0x10).cast::<f32>().write_unaligned(v);
        let v = rm(0x10) * f1 + rm(0x1c) * f2 + rm(0x28) * f3;
        p1.byte_add(0x14).cast::<f32>().write_unaligned(v);
        let v = f3 * rm(0x2c) + rm(0x20) * f2 + rm(0x14) * f1;
        p1.byte_add(0x18).cast::<f32>().write_unaligned(v);

        p1.byte_add(0x1c)
            .cast::<u32>()
            .write_unaligned(param_2.add(3).read_unaligned());
        let f1 = p1.byte_add(0x1c).cast::<f32>().read_unaligned();
        p1.byte_add(0x20)
            .cast::<u32>()
            .write_unaligned(param_2.add(4).read_unaligned());
        let f2 = p1.byte_add(0x20).cast::<f32>().read_unaligned();
        p1.byte_add(0x24)
            .cast::<u32>()
            .write_unaligned(param_2.add(5).read_unaligned());
        let f3 = p1.byte_add(0x24).cast::<f32>().read_unaligned();

        let v = rm(0xc) * f1 + rm(0x18) * f2 + rm(0x24) * f3;
        p1.byte_add(0x1c).cast::<f32>().write_unaligned(v);
        let v = rm(0x10) * f1 + rm(0x28) * f3 + rm(0x1c) * f2;
        p1.byte_add(0x20).cast::<f32>().write_unaligned(v);
        let v = f3 * rm(0x2c) + rm(0x20) * f2 + rm(0x14) * f1;
        p1.byte_add(0x24).cast::<f32>().write_unaligned(v);
    }
}
