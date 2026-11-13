// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536ef4: i32;
    static mut DAT_00536f25: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00428980(param_1: *mut u8) {
    // SAFETY: param_1 points at the game's node structure; child pointers and
    // the base pointer are the game's own memory, read and written where the
    // original does. The globals are the game's own.
    unsafe {
        if param_1.add(2).read() != 0 {
            let mut i_var1: i32 = 0;
            loop {
                if ((1i32.wrapping_shl(i_var1 as u32 & 0x1f)) as u32)
                    & (param_1.add(2).read() as u32)
                    != 0
                {
                    let child = param_1
                        .byte_add((i_var1.wrapping_mul(4)).wrapping_add(0x32) as usize)
                        .cast::<u32>()
                        .read_unaligned();
                    FUN_00428980(child as usize as *mut u8);
                }
                i_var1 = i_var1.wrapping_add(1);
                if i_var1 >= 8 {
                    break;
                }
            }
        }
        if param_1.add(1).read() as i32 == DAT_00536ef4 {
            let base = param_1.byte_add(0x2e).cast::<u32>().read_unaligned() as usize as *mut u8;

            let b = base.byte_add(2).read();
            let mask = !(1i32.wrapping_shl(param_1.read() as u32 & 0x1f));
            base.byte_add(2).write(((b as i32) & mask) as u8);

            let a = base.byte_add(10).cast::<i32>().read_unaligned();
            let s = param_1.byte_add(10).cast::<i32>().read_unaligned();
            base.byte_add(10)
                .cast::<i32>()
                .write_unaligned(a.wrapping_add(s));

            let a = base.byte_add(0x16).cast::<f64>().read_unaligned();
            let s = param_1.byte_add(0x16).cast::<f64>().read_unaligned();
            base.byte_add(0x16).cast::<f64>().write_unaligned(s + a);

            let a = base.byte_add(0x1e).cast::<f64>().read_unaligned();
            let s = param_1.byte_add(0x1e).cast::<f64>().read_unaligned();
            base.byte_add(0x1e).cast::<f64>().write_unaligned(s + a);

            let a = base.byte_add(0x26).cast::<f64>().read_unaligned();
            let s = param_1.byte_add(0x26).cast::<f64>().read_unaligned();
            base.byte_add(0x26).cast::<f64>().write_unaligned(s + a);

            DAT_00536f25 = DAT_00536f25.wrapping_sub(1);
        }
    }
}
