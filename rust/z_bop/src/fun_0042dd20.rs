// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a1051c: i32;
    static mut DAT_00a606d4: i32;
    static mut DAT_00a604cc: i32;
    static mut DAT_00a6052c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042dd20(param_1: i32) {
    // SAFETY: the globals are the game's own memory; the record table and the
    // index table are read and written exactly where the original does.
    unsafe {
        if param_1 > -1 && param_1 < DAT_00a606d4 {
            if param_1 < DAT_00a606d4.wrapping_sub(1) {
                let mut i_var3 = param_1;
                let mut p4: *mut u8 = (&raw mut DAT_00a1051c)
                    .cast::<u8>()
                    .byte_offset(param_1.wrapping_mul(0x145) as isize);
                loop {
                    let mut p5: *const u8 = p4.byte_add(0x145).cast_const();
                    let mut p6: *mut u8 = p4;
                    let mut i_var1: i32 = 0x51;
                    while i_var1 != 0 {
                        p6.cast::<u32>()
                            .write_unaligned(p5.cast::<u32>().read_unaligned());
                        p5 = p5.byte_add(4);
                        p6 = p6.byte_add(4);
                        i_var1 -= 1;
                    }
                    p6.write(p5.read());
                    i_var3 = i_var3.wrapping_add(1);
                    p4 = p4.byte_add(0x145);
                    if !(i_var3 < DAT_00a606d4.wrapping_sub(1)) {
                        break;
                    }
                }
            }
            let mut pi_var2: *mut i32 = &raw mut DAT_00a604cc;
            DAT_00a606d4 = DAT_00a606d4.wrapping_sub(1);
            loop {
                let v = pi_var2.read_unaligned();
                if param_1 < v {
                    pi_var2.write_unaligned(v.wrapping_sub(1));
                }
                pi_var2 = pi_var2.byte_add(12);
                if !(pi_var2.cast_const() < (&raw mut DAT_00a6052c).cast_const()) {
                    break;
                }
            }
        }
    }
}
