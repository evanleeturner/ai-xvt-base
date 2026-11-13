// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4870: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048f040(param_1: u8) {
    // SAFETY: the global is the game's own string buffer; we walk to its
    // terminating NUL and append one byte plus a new terminator, as the original does.
    unsafe {
        let mut p = (&raw mut DAT_006d4870).cast::<u8>();
        while p.read() != 0 {
            p = p.byte_add(1);
        }
        p.write(param_1);
        p.byte_add(1).write(0);
    }
}
