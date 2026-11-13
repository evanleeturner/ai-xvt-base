// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052552c: i32;
    static mut DAT_006da3d0: u8;
    static mut DAT_00525518: i32;
    static mut DAT_00732fe0: u16;
    static mut DAT_00525538: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00406040(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405fd0(param_1: u8) {
    // SAFETY: the globals and the callee are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        'done: {
            if DAT_0052552c < 0x102 {
                if DAT_0052552c != 0x101 {
                    DAT_006da3d0 = 0;
                    break 'done;
                }
            } else {
                if DAT_0052552c == 0x111 {
                    DAT_006da3d0 = 2;
                    break 'done;
                }
                if DAT_0052552c != 0x112 {
                    DAT_006da3d0 = 0;
                    break 'done;
                }
            }
            DAT_006da3d0 = 1;
        }
        if DAT_00525518 == 2 {
            DAT_006da3d0 = 2;
        }
        DAT_00732fe0 = 1;
        FUN_00406040(u32::from(DAT_006da3d0));
        DAT_00525538 = param_1;
    }
}
