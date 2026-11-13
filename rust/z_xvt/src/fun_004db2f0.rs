// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075d846: [i32; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32 {
    if param_1.is_null() {
        return 0;
    }
    if param_2 >= 0 && param_2 < 0x100 {
        // SAFETY: the global table and the pointers it holds are the game's own
        // memory; param_1 points at a NUL-terminated string from the caller.
        unsafe {
            let iv3 = (&raw mut DAT_0075d846)
                .cast::<i32>()
                .add(param_2 as usize)
                .read_unaligned();
            if iv3 != 0 {
                let base = iv3 as u32;
                let mut iv4: i32 = 0;
                let mut iv5: usize = 0;
                let mut c = param_1.read() as u8;
                while c != 0 {
                    let b = param_1.add(iv5).read() as u8;
                    if b > 6 {
                        let t = *((base.wrapping_add(0x504).wrapping_add(b as u32)) as *const u8);
                        let u = *((base.wrapping_add(0x609)) as *const u8);
                        iv4 = iv4.wrapping_add(t as i32).wrapping_add(u as i32);
                    }
                    iv5 += 1;
                    c = param_1.add(iv5).read() as u8;
                }
                let u = *((base.wrapping_add(0x609)) as *const u8);
                return iv4.wrapping_sub(u as i32);
            }
            return 0;
        }
    }
    0
}
