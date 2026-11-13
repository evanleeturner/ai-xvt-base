// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00402e00(param_1: u16) -> i32 {
    if param_1 < 0x8000 {
        return 0;
    }
    2 - (param_1 < 0xc000) as i32
}
