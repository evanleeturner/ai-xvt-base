// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a90a60: u32;
    static mut _DAT_00a90a5e: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a1fe0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458780(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004021a0(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own memory; the
    // addresses computed from them are read and written where the original does.
    unsafe {
        if DAT_00a90a60 != 0 {
            _DAT_00a90a5e = _DAT_00a90a5e.wrapping_add(1);
            let u_var1: u16 = (DAT_00a90a60.wrapping_add((_DAT_00a90a5e as u32).wrapping_mul(2))
                as *const u16)
                .read_unaligned();
            if u_var1 == 0xffff {
                *((DAT_009a1fe8
                    .wrapping_add(3)
                    .wrapping_add(param_1.wrapping_mul(0x23))) as *mut u8) = 0;
                if param_1 < DAT_009a1fe0 {
                    let inner: u32 = (DAT_009a1fe8
                        .wrapping_add(0x1f)
                        .wrapping_add(param_1.wrapping_mul(0x23))
                        as *const u32)
                        .read_unaligned();
                    let i_var2: i32 = (inner.wrapping_add(0xa9) as *const i32).read_unaligned();
                    if i_var2 != 0 {
                        FUN_00458780(i_var2);
                        return;
                    }
                }
            } else {
                if u_var1 == 0xfffd {
                    _DAT_00a90a5e = 0;
                    return;
                }
                if u_var1 > 0xfeff && u_var1 != 0xfffe {
                    _DAT_00a90a5e = u_var1.wrapping_add(0x100);
                }
            }
        }
    }
}
