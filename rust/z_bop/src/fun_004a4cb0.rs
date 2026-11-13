// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a73b6: i32;
    static mut DAT_00a08104: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a4cb0(param_1: u16) {
    // SAFETY: all globals and the addresses derived from them are the game's
    // own memory, accessed at the same offsets and in the same order as the original.
    unsafe {
        let idx = (DAT_009a73b0 as u32).wrapping_mul(0x23);
        let psv = idx.wrapping_add(0x10).wrapping_add(DAT_009a1fe8 as u32) as *mut i16;
        let a = psv.read_unaligned();
        let b = ((DAT_009a73b6 as u32).wrapping_add(0x32) as *const i16).read_unaligned();
        let mut u_var3: u16 = a.wrapping_sub(b) as u16;
        if 0x7fff < u_var3 {
            u_var3 = u_var3.wrapping_neg();
        }
        if u_var3 < 0x301 {
            let b2 = ((DAT_009a73b6 as u32).wrapping_add(0x32) as *const i16).read_unaligned();
            psv.write_unaligned(b2);
            let src = (DAT_009a73b0 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32) as *const i32;
            let ptr_val = src.read_unaligned() as u32;
            ((ptr_val.wrapping_add(0x92)) as *mut u8).write(1);
            let v = ((ptr_val.wrapping_add(0x92)) as *const u8).read();
            ((ptr_val.wrapping_add(0x8b)) as *mut u8).write(v);
            ((DAT_00a08104 as u32).wrapping_add(0xab) as *mut u8).write(3);
            return;
        }
        ((DAT_00a08104 as u32).wrapping_add(0xab) as *mut u8).write(2);
        ((DAT_00a08104 as u32).wrapping_add(0xac) as *mut u16).write_unaligned(param_1);
    }
}
