// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_0051815c: f32;
}

unsafe extern "C" {
    fn sinl(x: f64) -> f64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046a3c0(param_1: i16) -> i64 {
    // SAFETY: the global and sinl are the game's own; the value is read once
    // and passed to sinl, whose result is converted in place of __ftol.
    unsafe {
        let r = sinl((param_1 as f64) * (_DAT_0051815c as f64));
        r as i64
    }
}
