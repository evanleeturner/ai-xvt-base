// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0055692c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00444eb0(param_1: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00444e60(param_1: *mut u8) {
    // SAFETY: param_1 points at the game's node structure; child pointers
    // are read from the offsets the original uses, and the callee and global
    // belong to the game.
    unsafe {
        if param_1.byte_add(2).read() != 0 {
            let mut i_var1: i32 = 0;
            loop {
                if ((1i32.wrapping_shl((i_var1 as u8 & 0x1f) as u32)) as u32
                    & u32::from(param_1.byte_add(2).read()))
                    != 0
                {
                    let ptr: *mut u8 = param_1
                        .byte_add((i_var1.wrapping_mul(4)).wrapping_add(0x32) as usize)
                        .cast::<*mut u8>()
                        .read_unaligned();
                    FUN_00444e60(ptr);
                }
                i_var1 = i_var1.wrapping_add(1);
                if i_var1 >= 8 {
                    break;
                }
            }
        }
        if u32::from(param_1.byte_add(1).read()) == DAT_0055692c {
            FUN_00444eb0(param_1);
        }
    }
}
