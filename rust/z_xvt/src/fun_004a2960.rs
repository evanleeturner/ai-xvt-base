// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006559b4: u16;
    static mut DAT_006d9984: *const core::ffi::c_void;
    static mut DAT_0071ecf4: u32;
    static mut DAT_0070ac46: u16;
    static mut DAT_0070ac48: u16;
    static mut DAT_0070ac40: u32;
    static mut DAT_0070a178: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ee10(param_1: i16);
    fn FUN_004a2e70();
    fn FUN_0048ee20(param_1: i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a2960(param_1: i16, param_2: i8) {
    // SAFETY: the globals and callees are the game's own; the function pointer
    // in DAT_006d9984 is the game's callback taking one u32; the indexed writes
    // target the game's tables at the same byte offsets as the original.
    unsafe {
        if DAT_006559b4 == 0 {
            if param_2 != b'?' as i8
                && param_2 != b'!' as i8
                && param_2 != b':' as i8
                && param_2 != b' ' as i8
                && param_2 != b'.' as i8
            {
                let f: unsafe extern "C" fn(u32) = core::mem::transmute(DAT_006d9984);
                f(0x2e);
            }
            FUN_0048ee10(1);
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(DAT_006d9984);
            f(10);
            FUN_0048ee10(0);
        }
        if param_1 == 3 || param_1 == 7 {
            (&raw mut DAT_0070ac46)
                .cast::<u8>()
                .byte_add(DAT_0071ecf4.wrapping_mul(0x10) as usize)
                .cast::<u16>()
                .write_unaligned(0x1d8);
        } else if param_1 == 8 {
            (&raw mut DAT_0070ac48)
                .cast::<u8>()
                .byte_add(DAT_0071ecf4.wrapping_mul(0x10) as usize)
                .cast::<u16>()
                .write_unaligned(0x760);
        } else if param_1 == 4 {
            (&raw mut DAT_0070ac46)
                .cast::<u8>()
                .byte_add(DAT_0071ecf4.wrapping_mul(0x10) as usize)
                .cast::<u16>()
                .write_unaligned(0x760);
        } else if DAT_0070a178 == 0 {
            if param_1 == 2 || param_1 == 1 {
                (&raw mut DAT_0070ac40)
                    .cast::<u8>()
                    .byte_add(DAT_0071ecf4.wrapping_mul(8) as usize)
                    .cast::<u32>()
                    .write_unaligned(0x588);
            } else {
                (&raw mut DAT_0070ac40)
                    .cast::<u8>()
                    .byte_add(DAT_0071ecf4.wrapping_mul(8) as usize)
                    .cast::<u32>()
                    .write_unaligned(0x674);
            }
        } else {
            (&raw mut DAT_0070ac40)
                .cast::<u8>()
                .byte_add(DAT_0071ecf4.wrapping_mul(8) as usize)
                .cast::<u32>()
                .write_unaligned(0x162);
        }
        FUN_004a2e70();
        FUN_0048ee20(2);
    }
}
