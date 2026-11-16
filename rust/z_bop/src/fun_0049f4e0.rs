// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: u32;
    static mut DAT_009a73b6: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_00527760: i32;
    static mut DAT_009a73d0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00404620(param_1: u32) -> u32;
    fn FUN_004a4cb0(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049f4e0(param_1: u32) {
    // SAFETY: all addresses are the game's own memory, read and written
    // exactly where the original does.
    unsafe {
        let u_var1 = ((DAT_00a08104.wrapping_add(0x6a)) as *const u16).read_unaligned();
        let flag = (((DAT_009a73b6.wrapping_add(0x35)) as *const u8).read() & 1) != 0;
        let s_var2: i16;
        if u_var1 == 0xffff {
            if flag {
                let a = param_1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x10)
                    .wrapping_add(DAT_009a1fe8);
                s_var2 = (a as *const i16).read_unaligned().wrapping_sub(0x4000);
            } else {
                let a = param_1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x10)
                    .wrapping_add(DAT_009a1fe8);
                s_var2 = (a as *const i16).read_unaligned().wrapping_add(0x4000);
            }
        } else if flag {
            let a = (u_var1 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x10)
                .wrapping_add(DAT_009a1fe8);
            s_var2 = (a as *const i16).read_unaligned().wrapping_sub(0x4000);
        } else {
            let p = u_var1 as u32;
            let a = p
                .wrapping_mul(0x23)
                .wrapping_add(0x10)
                .wrapping_add(DAT_009a1fe8);
            s_var2 = (a as *const i16).read_unaligned().wrapping_add(0x4000);
        }
        ((DAT_009a73b6.wrapping_add(0x32)) as *mut i16).write_unaligned(s_var2);
        let u_var3 = FUN_00404620(DAT_00a08104);
        FUN_004a4cb0((((u_var3 >> 1) as u16) & 0x7fff).wrapping_add(0x8000));
        let src = (&raw mut DAT_00527760)
            .cast::<u8>()
            .byte_add(DAT_009a73d0.wrapping_mul(2) as usize)
            .cast::<i16>()
            .read_unaligned();
        ((DAT_009a73b6.wrapping_add(0x3a)) as *mut i16).write_unaligned(src.wrapping_mul(0xec));
    }
}
