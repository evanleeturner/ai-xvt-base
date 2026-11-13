// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f34d0(param_1: i16, param_2: i16, param_3: i16) -> i16 {
    let mut p1 = param_1;
    let s_var1: i16;
    'out: {
        if p1 > param_2 {
            let t = (p1 as u16).wrapping_sub(param_3 as u16) as i16;
            p1 = param_2;
            if !(t < param_2) {
                s_var1 = t;
                break 'out;
            }
        }
        // p1 <= param_2 here, or p1 was set to param_2
        let t = p1;
        if !(p1 < param_2) {
            s_var1 = t;
            break 'out;
        }
        let t = (p1 as u16).wrapping_add(param_3 as u16) as i16;
        if param_2 < t {
            s_var1 = param_2;
        } else {
            s_var1 = t;
        }
    }
    s_var1
}
