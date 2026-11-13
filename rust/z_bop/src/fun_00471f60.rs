// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00471f60(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 points at the caller's structure; the embedded
    // offsets are addresses in the game's own memory, read and written
    // where the original does.
    unsafe {
        if param_1.read_unaligned() != 0 {
            param_1.write_unaligned(param_1.read_unaligned().wrapping_add(param_2));
        }
        if param_1.add(5).read_unaligned() != 0 {
            let mut i_var2: i32 = param_2.wrapping_add(param_1.add(5).read_unaligned());
            let mut i_var3: i32 = 0;
            param_1.add(5).write_unaligned(i_var2);
            if 0 < param_1.add(4).read_unaligned() {
                loop {
                    let p = (i_var2.wrapping_add(8)) as *mut i32;
                    if p.read_unaligned() != 0 {
                        p.write_unaligned(p.read_unaligned().wrapping_add(param_2));
                    }
                    i_var2 = i_var2.wrapping_add(0xc);
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3 < param_1.add(4).read_unaligned()) {
                        break;
                    }
                }
            }
        }
        let mut i_var2: i32 = 0;
        if param_1.add(3).read_unaligned() != 0 {
            let mut i_var3: i32 = 0;
            param_1
                .add(3)
                .write_unaligned(param_1.add(3).read_unaligned().wrapping_add(param_2));
            if 0 < param_1.add(2).read_unaligned() {
                loop {
                    let slot = (param_1.add(3).read_unaligned().wrapping_add(i_var2)) as *mut i32;
                    let i_var1 = slot.read_unaligned();
                    if i_var1 != 0 {
                        slot.write_unaligned(i_var1.wrapping_add(param_2));
                        FUN_00471f60(i_var1 as *mut i32, param_2);
                    }
                    i_var2 = i_var2.wrapping_add(4);
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3 < param_1.add(2).read_unaligned()) {
                        break;
                    }
                }
            }
        }
    }
}
