// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042b1e0(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: param_1 and the pointers reachable from it are the game's own
    // structures, read and written at the offsets the original uses.
    unsafe {
        let default = param_1
            .wrapping_add(0x386)
            .wrapping_add(param_2.wrapping_mul(4));
        let psv = (param_1
            .wrapping_add(0x3f2)
            .wrapping_add(param_2.wrapping_mul(4)) as *const i32)
            .read_unaligned() as *const i16;
        if psv.is_null() {
            return default;
        }
        let slot = param_1
            .wrapping_add(0x3c6)
            .wrapping_add(param_2.wrapping_mul(2)) as *mut i16;
        if slot.read_unaligned() != psv.read_unaligned() {
            slot.write_unaligned(0);
            return default;
        }
        let inner = (psv as *const u8)
            .byte_add(0x1f)
            .cast::<i32>()
            .read_unaligned();
        if inner == 0 {
            return default;
        }
        let i_var2 = (inner.wrapping_add(0xab) as *const i32).read_unaligned();
        if i_var2 == 0 {
            return default;
        }
        i_var2.wrapping_add(0x44)
    }
}
