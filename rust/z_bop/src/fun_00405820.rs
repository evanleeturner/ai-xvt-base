// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_0051c573: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405820(param_1: u32) -> i32 {
    // SAFETY: the globals are the game's own memory, and the computed
    // addresses point into the game's tables, read exactly as the original does.
    unsafe {
        let u_addr: u32 = (param_1 & 0xffff)
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8 as u32);
        let u_ptr1: u32 = (u_addr as *const u32).read_unaligned();
        let u_ptr2: u32 = u_ptr1.wrapping_add(0xa9);
        let tmp1: u32 = (u_ptr2 as *const u32).read_unaligned();
        let b_val: u8 = ((tmp1.wrapping_add(4)) as *const u8).read();
        let u_addr2: u32 = (DAT_0051c573 as u32).wrapping_add((b_val as u32).wrapping_mul(0x101));
        let tmp2: u32 = (u_addr2 as *const u32).read_unaligned();
        tmp2.wrapping_mul(2) as i32
    }
}
