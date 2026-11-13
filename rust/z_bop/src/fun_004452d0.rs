// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_004438c0() -> *mut u32;
    fn malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_0049eb60(param_1: i32);
    fn FUN_00443a50(param_1: i32);
    fn FUN_004441f0(param_1: i32);
    fn FUN_00443c30(param_1: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004452d0(param_1: *mut u8, param_2: i32, param_3: u32, param_4: i32) {
    // SAFETY: the callees are the game's own; the object returned by
    // FUN_004438c0 and the buffers passed in are used at the offsets the
    // original uses.
    unsafe {
        let obj = FUN_004438c0() as *mut u8;
        if obj.is_null() {
            return;
        }
        obj.byte_add(0x1028).cast::<i32>().write_unaligned(0);
        obj.byte_add(0x1030).cast::<u32>().write_unaligned(param_3);
        obj.byte_add(0x1034).cast::<i32>().write_unaligned(param_4);

        let field_108e: u32 = obj
            .byte_add(0x1030)
            .cast::<u32>()
            .read_unaligned()
            .wrapping_mul(param_4 as u32);
        obj.byte_add(0x108e)
            .cast::<u32>()
            .write_unaligned(field_108e);

        let pv = malloc(field_108e.wrapping_mul(6));
        obj.byte_add(0x1086)
            .cast::<*mut core::ffi::c_void>()
            .write_unaligned(pv);

        if pv.is_null() {
            FUN_0049eb60(0);
        }

        let mut p5 = obj
            .byte_add(0x1086)
            .cast::<*mut core::ffi::c_void>()
            .read_unaligned() as *mut u8;
        let mut src = param_1;
        let mut u6: u32 = 0;
        if obj.byte_add(0x1034).cast::<u32>().read_unaligned() != 0 {
            loop {
                let mut u7: u32 = 0;
                let mut p3 = src;
                let mut p4 = p5;
                if obj.byte_add(0x1030).cast::<u32>().read_unaligned() != 0 {
                    loop {
                        src = p3.add(1);
                        p5 = p4.add(6);
                        u7 = u7.wrapping_add(1);
                        let idx = p3.read() as i32;
                        let addr = param_2.wrapping_add(idx.wrapping_mul(2));
                        let v = (addr as usize as *const u16).read_unaligned();
                        p4.write(((v >> 8) as u8) & 0xf8);
                        p4.add(1).write(((v >> 5) as u8).wrapping_shl(2));
                        p4.add(2).write((v as u8).wrapping_shl(3));
                        p4.add(4).write(0);
                        p4.add(5).write(0);
                        p4.add(3).write(0);
                        p3 = src;
                        p4 = p5;
                        if !(u7 < obj.byte_add(0x1030).cast::<u32>().read_unaligned()) {
                            break;
                        }
                    }
                }
                u6 = u6.wrapping_add(1);
                if !(u6 < obj.byte_add(0x1034).cast::<u32>().read_unaligned()) {
                    break;
                }
            }
        }
        FUN_00443a50(obj as usize as i32);
        FUN_004441f0(obj as usize as i32);
        FUN_00443c30(obj as *mut u32);
    }
}
