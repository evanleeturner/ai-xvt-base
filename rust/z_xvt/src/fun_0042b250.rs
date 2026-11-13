// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042b250(param_1: i32) -> u32 {
    // SAFETY: param_1 points at the game's own object, and the pointers
    // read from it refer to the game's own structures; every access is
    // unaligned and happens in the original's order.
    unsafe {
        let base = param_1 as u32;
        let p1 = ((base.wrapping_add(0x432)) as *const u32).read_unaligned();
        if p1 == 0 {
            return ((base.wrapping_add(9)) as *const u16).read_unaligned() as u32;
        }
        let a = ((base.wrapping_add(0x384)) as *const u16).read_unaligned();
        let b = (p1 as *const u16).read_unaligned();
        if a != b {
            ((base.wrapping_add(0x432)) as *mut u32).write_unaligned(0);
            let f9 = ((base.wrapping_add(9)) as *const u16).read_unaligned();
            return (p1 & 0xFFFF0000) | f9 as u32;
        }
        let field_1f = (p1.wrapping_add(0x1f) as *const i32).read_unaligned();
        if field_1f == 0 {
            return ((base.wrapping_add(9)) as *const u16).read_unaligned() as u32;
        }
        let p2 = ((field_1f as u32).wrapping_add(0xab) as *const u32).read_unaligned();
        if p2 != 0 {
            let v = (p2 as *const u16).read_unaligned();
            return (p2 & 0xFFFF0000) | v as u32;
        }
        ((base.wrapping_add(9)) as *const u16).read_unaligned() as u32
    }
}
