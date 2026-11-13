// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421580(param_1: i32) {
    // SAFETY: param_1 is the address of the game's own structure; the fields
    // at the given offsets hold pointers (or zero) which are read and written
    // where the original does.
    unsafe {
        let p = (param_1.wrapping_add(0x432)) as *mut i32;
        let v = p.read_unaligned();
        if v != 0 {
            ((v as u32).wrapping_add(3) as *mut u8).write(0);
            p.write_unaligned(0);
        }
        let mut pi = (param_1.wrapping_add(0x3f2)) as *mut i32;
        let mut count: i32 = 0x10;
        loop {
            let v = pi.read_unaligned();
            if v != 0 {
                ((v as u32).wrapping_add(3) as *mut u8).write(0);
                pi.write_unaligned(0);
            }
            pi = pi.wrapping_add(1);
            count -= 1;
            if count == 0 {
                break;
            }
        }
    }
}
