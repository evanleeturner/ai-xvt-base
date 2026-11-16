// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: i32;
    static mut DAT_009a8c10: i8;
    static mut DAT_00a0fe80: i16;
    static mut DAT_00bb737d: i8;
    static mut DAT_009a20a4: u8;
    static mut DAT_00a0aa1f: i8;
    static mut DAT_00a0aa9f: i8;
    static mut DAT_00a0a90e: i16;
    static mut DAT_00a0a910: i16;
    static mut DAT_00a0fc80: i32;
    static mut DAT_00a0aa20: i8;
    static mut DAT_00a0aaa0: i8;
    static mut DAT_00a0a880: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00430610(
    param_1: i32,
    param_2: i8,
    param_3: i8,
    param_4: i8,
    param_5: i16,
) -> i32 {
    // SAFETY: all globals are the game's own memory; indices follow the
    // original's pointer arithmetic exactly.
    unsafe {
        if DAT_00523410 != 0 {
            return 0;
        }
        if DAT_009a8c10 == 0 {
            return 0;
        }
        let p = (&raw mut DAT_00a0fe80 as *mut u8).wrapping_add(param_1.wrapping_mul(2) as usize);
        if p.cast::<i16>().read_unaligned() == 0 {
            return 0;
        }
        if DAT_00bb737d == 0 {
            if DAT_009a20a4 == 0x80 {
                return 0;
            }
            if DAT_009a20a4 != 0 && param_2 == 2 && param_3 == 1 && param_4 == 0 {
                let u = DAT_009a20a4 as usize;
                if (&raw mut DAT_00a0aa1f as *mut i8).add(u).read() == 2
                    && (&raw mut DAT_00a0aa9f as *mut i8).add(u).read() == 1
                    && (&raw mut DAT_00a0a90e as *mut i16)
                        .add(u * 2)
                        .read_unaligned()
                        == param_5
                {
                    return 0;
                }
            }
            let u = DAT_009a20a4 as usize;
            (&raw mut DAT_00a0a910 as *mut i16)
                .add(u)
                .write_unaligned(param_5);
            (&raw mut DAT_00a0fc80 as *mut i32)
                .add(u)
                .write_unaligned(param_1);
            (&raw mut DAT_00a0aa20 as *mut i8).add(u).write(param_2);
            (&raw mut DAT_00a0aaa0 as *mut i8).add(u).write(param_3);
            (&raw mut DAT_00a0a880 as *mut i8).add(u).write(param_4);
            DAT_009a20a4 = DAT_009a20a4.wrapping_add(1);
            return 1;
        }
        0
    }
}
