// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9652: i16;
    static mut DAT_009a8c18: i32;
    static mut DAT_009fe7d4: i32;
    static mut DAT_009ed230: i16;
    static mut DAT_009ec458: i16;
    static mut DAT_009d114c: i16;
    static mut DAT_009d1314: i16;
    static mut DAT_009a73f6: i16;
    static mut DAT_009a7bb0: i16;
    static mut DAT_00a07cd0: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004230f0() -> i32 {
    // SAFETY: the globals are the game's own memory, and DAT_009fe7d4 holds
    // the address of a game structure read at the offsets the original uses.
    unsafe {
        DAT_009e9652 = DAT_009e9652.wrapping_sub(1);
        DAT_009a8c18 = DAT_009a8c18
            .wrapping_add((DAT_009fe7d4.wrapping_add(0x3eb8) as *const i32).read_unaligned());
        if DAT_009e9652 == DAT_009ed230 {
            DAT_009ec458 = 0;
            DAT_009d114c = -1;
            DAT_009d1314 = -1;
        } else if DAT_009e9652 < 0 {
            if DAT_009e9652 == -1 {
                DAT_009a73f6 = -1;
            }
            DAT_009a73f6 = DAT_009a73f6.wrapping_add(1);
            let addr = DAT_009fe7d4
                .wrapping_add(0x1934)
                .wrapping_add((DAT_009a73f6 as i32).wrapping_mul(2));
            let v = (addr as *const i16).read_unaligned();
            DAT_009ec458 = DAT_009ec458.wrapping_add(v);
        }
        DAT_009a7bb0 = DAT_009a7bb0.wrapping_sub(1);
        if DAT_009ed230 == DAT_009a7bb0 {
            let v = (DAT_009fe7d4.wrapping_add(0xe) as *const i16).read_unaligned();
            DAT_009d114c = v.wrapping_sub(1);
            return 1;
        }
        if DAT_009a7bb0 < 0 {
            return 0;
        }
        if DAT_00a07cd0 <= DAT_009a7bb0 && DAT_009e9652 < DAT_00a07cd0 {
            DAT_009d1314 = DAT_009d1314.wrapping_add(1);
            let addr = DAT_009fe7d4
                .wrapping_add(0x1934)
                .wrapping_add((DAT_009d1314 as i32).wrapping_mul(2));
            let v = (addr as *const i16).read_unaligned();
            DAT_009d114c = DAT_009d114c.wrapping_add(v);
        }
        1
    }
}
