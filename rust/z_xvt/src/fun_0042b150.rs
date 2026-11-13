// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042b150(param_1: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 and the pointers chained from it are addresses in the
    // game's own memory; reads and writes happen at the original's offsets
    // and in its order.
    unsafe {
        let psv = (param_1 as usize as *const u8)
            .byte_add(0x432)
            .cast::<i32>()
            .read_unaligned();
        let psv = psv as usize as *const i16;
        if psv.is_null() {
            return param_1.wrapping_add(0x2a);
        }
        let a = ((param_1.wrapping_add(900)) as usize as *const i16).read_unaligned();
        let b = psv.read_unaligned();
        if a != b {
            ((param_1 as usize as *mut u8).byte_add(0x432))
                .cast::<i32>()
                .write_unaligned(0);
            return param_1.wrapping_add(0x2a);
        }
        let tmp_1f = (psv as *const u8)
            .byte_add(0x1f)
            .cast::<i32>()
            .read_unaligned();
        if tmp_1f == 0 {
            return param_1.wrapping_add(0x2a);
        }
        let tmp_ab = (tmp_1f as usize as *const u8)
            .byte_add(0xab)
            .cast::<i32>()
            .read_unaligned();
        if tmp_ab != 0 {
            return tmp_ab.wrapping_add(4);
        }
        let tmp_a7 = (tmp_1f as usize as *const u8)
            .byte_add(0xa7)
            .cast::<i32>()
            .read_unaligned();
        if tmp_a7 != 0 {
            if (tmp_a7.wrapping_add(6) as usize as *const u8).read() == 0 {
                (param_1.wrapping_add(6) as usize as *mut u8).write(1);
                let tmp_nested = (tmp_1f as usize as *const u8)
                    .byte_add(0xa7)
                    .cast::<i32>()
                    .read_unaligned();
                let r = FUN_0042b150(tmp_nested);
                (param_1.wrapping_add(6) as usize as *mut u8).write(0);
                return r;
            }
            return param_1.wrapping_add(0x2a);
        }
        param_1.wrapping_add(0x2a)
    }
}
