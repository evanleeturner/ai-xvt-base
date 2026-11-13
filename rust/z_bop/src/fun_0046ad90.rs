// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00526050: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046ad90(param_1: *mut u8) -> i32 {
    let mut idx: i32 = 0;
    // SAFETY: the table lives in the game's memory at DAT_00526050 in 0x52-byte
    // records ending at an empty first byte; param_1 points at a
    // NUL-terminated string supplied by the caller.
    unsafe {
        let base = (&raw mut DAT_00526050) as *mut u8;
        if base.read() == 0 {
            return idx;
        }
        let mut rec = base;
        loop {
            let mut a = rec;
            let mut b = param_1 as *const u8;
            let equal;
            loop {
                let x = a.read();
                let y = b.read();
                if x != y {
                    equal = false;
                    break;
                }
                if x == 0 {
                    equal = true;
                    break;
                }
                a = a.byte_add(1);
                b = b.byte_add(1);
            }
            if equal {
                return idx;
            }
            rec = rec.byte_add(0x52);
            idx = idx.wrapping_add(1);
            if rec.read() == 0 {
                break;
            }
        }
    }
    idx
}
