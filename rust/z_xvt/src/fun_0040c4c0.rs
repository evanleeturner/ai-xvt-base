// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525550: i32;
    static mut DAT_006db590: i8;
    static mut DAT_00740a80: [i16; 0];
    static mut DAT_0085e79d: i8;
    static mut DAT_006d4a44: i8;
    static mut DAT_0073b65f: [i8; 0];
    static mut DAT_0073b6df: [i8; 0];
    static mut DAT_0073b54e: [i16; 0];
    static mut DAT_0073b550: [i16; 0];
    static mut DAT_00740880: [i32; 0];
    static mut DAT_0073b660: [i8; 0];
    static mut DAT_0073b6e0: [i8; 0];
    static mut DAT_0073b4c0: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040c4c0(
    param_1: i32,
    param_2: i8,
    param_3: i8,
    param_4: i8,
    param_5: i16,
) -> i32 {
    // SAFETY: all globals and arrays are the game's own memory; indices are
    // used exactly as the original does.
    unsafe {
        if DAT_00525550 != 0 {
            return 0;
        }
        if DAT_006db590 == 0 {
            return 0;
        }
        if (&raw mut DAT_00740a80)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read_unaligned()
            == 0
        {
            return 0;
        }
        if DAT_0085e79d != 0 {
            if (DAT_006d4a44 as u8) == 0x80 {
                return 0;
            }
            if DAT_006d4a44 != 0 && param_2 == 2 && param_3 == 1 && param_4 == 0 {
                let u = (DAT_006d4a44 as i32 as u32) as usize;
                if (&raw mut DAT_0073b65f).cast::<i8>().add(u).read() == 2
                    && (&raw mut DAT_0073b6df).cast::<i8>().add(u).read() == 1
                    && (&raw mut DAT_0073b54e)
                        .cast::<i16>()
                        .add(u)
                        .read_unaligned()
                        == param_5
                {
                    return 0;
                }
            }
            let u = (DAT_006d4a44 as i32 as u32) as usize;
            (&raw mut DAT_0073b550)
                .cast::<i16>()
                .add(u)
                .write_unaligned(param_5);
            (&raw mut DAT_00740880)
                .cast::<i32>()
                .add(u)
                .write_unaligned(param_1);
            (&raw mut DAT_0073b660).cast::<i8>().add(u).write(param_2);
            (&raw mut DAT_0073b6e0).cast::<i8>().add(u).write(param_3);
            (&raw mut DAT_0073b4c0).cast::<i8>().add(u).write(param_4);
            DAT_006d4a44 = DAT_006d4a44.wrapping_add(1);
            return 1;
        }
        0
    }
}
