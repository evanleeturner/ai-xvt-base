// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233d8: i32;
    static mut DAT_00527ecc: i32;
    static mut DAT_00612294: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00497d80(param_1: i32, param_2: i32, mut param_3: i32) {
    if 0 < param_3 {
        // SAFETY: the globals are the game's own; the pointers are addresses
        // the caller supplies, copied byte by byte exactly as the original does.
        unsafe {
            if DAT_005233d8 == 2 {
                let mut src = param_1.wrapping_add(param_2.wrapping_mul(2)) as *const u8;
                let mut dst = DAT_00527ecc
                    .wrapping_add(DAT_00612294)
                    .wrapping_add(param_2.wrapping_mul(2)) as *mut u8;
                loop {
                    let hi = src.add(1).read();
                    dst.write(src.read());
                    dst.add(1).write(hi);
                    src = src.add(2);
                    dst = dst.add(2);
                    param_3 = param_3.wrapping_sub(1);
                    if param_3 == 0 {
                        break;
                    }
                }
                return;
            }
            let mut src = param_1.wrapping_add(param_2) as *const u8;
            let mut dst = DAT_00527ecc
                .wrapping_add(param_2)
                .wrapping_add(DAT_00612294) as *mut u8;
            loop {
                let v = src.read();
                src = src.add(1);
                dst.write(v);
                dst = dst.add(1);
                param_3 = param_3.wrapping_sub(1);
                if param_3 == 0 {
                    break;
                }
            }
        }
    }
}
