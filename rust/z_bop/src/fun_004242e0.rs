// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9654: i16;
    static mut DAT_009a8c18: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009ec458: i16;
    static mut DAT_009d114c: i16;
    static mut DAT_009d8c14: i16;
    static mut DAT_009d8c2c: i16;
    static mut DAT_009d8c2a: i16;
    static mut DAT_009a7bb2: i16;
    static mut DAT_009fe7d4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004242e0() -> i32 {
    // SAFETY: the globals are the game's own memory; DAT_009fe7d4 holds an
    // address of the game's table, read at the offsets the original uses.
    unsafe {
        DAT_009a8c18 = DAT_009a8c18.wrapping_add(DAT_005233d8);
        DAT_009e9654 = DAT_009e9654.wrapping_add(1);
        if DAT_009e9654 == 0 {
            DAT_009ec458 = 0;
            DAT_009d114c = -1;
            DAT_009d8c14 = -1;
        } else if DAT_009d8c2c <= DAT_009e9654 {
            if DAT_009e9654 != DAT_009d8c2c {
                DAT_009d8c2a = -1;
            }
            DAT_009d8c2a = DAT_009d8c2a.wrapping_add(1);
            let addr = DAT_009fe7d4
                .wrapping_add(0x1934)
                .wrapping_add((DAT_009d8c2a as i32).wrapping_mul(2));
            let v = (addr as usize as *const i16).read_unaligned();
            DAT_009ec458 = (DAT_009ec458 as i32).wrapping_add(v as i32) as i16;
        }
        DAT_009a7bb2 = DAT_009a7bb2.wrapping_add(1);
        if DAT_009a7bb2 == 0 {
            let v = (DAT_009fe7d4.wrapping_add(0xe) as usize as *const i16).read_unaligned();
            DAT_009d114c = (v as i32).wrapping_sub(1) as i16;
            return 1;
        }
        if DAT_009d8c2c <= DAT_009a7bb2 {
            return 0;
        }
        if DAT_009a7bb2 < 0 && -1 < DAT_009e9654 {
            DAT_009d8c14 = DAT_009d8c14.wrapping_add(1);
            let addr = DAT_009fe7d4
                .wrapping_add(0x1934)
                .wrapping_add((DAT_009d8c14 as i32).wrapping_mul(2));
            let v = (addr as usize as *const i16).read_unaligned();
            DAT_009d114c = (DAT_009d114c as i32).wrapping_add(v as i32) as i16;
        }
        1
    }
}
