// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9654: i16;
    static mut DAT_009a8c18: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009d77f0: i16;
    static mut DAT_009ec458: i16;
    static mut DAT_009d114c: i16;
    static mut DAT_009d8c14: i16;
    static mut DAT_009d8c2a: i16;
    static mut DAT_009fe7d4: i32;
    static mut DAT_009a7bb2: i16;
    static mut DAT_009d8c2c: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00423810() -> i32 {
    // SAFETY: the globals are the game's own memory; DAT_009fe7d4 holds an
    // address of a game structure read at the offsets the original uses.
    unsafe {
        DAT_009e9654 = DAT_009e9654.wrapping_sub(1);
        DAT_009a8c18 = DAT_009a8c18.wrapping_sub(DAT_005233d8);
        if DAT_009e9654 == DAT_009d77f0 {
            DAT_009ec458 = 0;
            DAT_009d114c = -1;
            DAT_009d8c14 = -1;
        } else if DAT_009e9654 < 0 {
            DAT_009d8c2a = DAT_009d8c2a.wrapping_add(1);
            let addr = DAT_009fe7d4
                .wrapping_add(0x1934)
                .wrapping_add((DAT_009d8c2a as i32).wrapping_mul(2));
            let v = (addr as *const i16).read_unaligned();
            DAT_009ec458 = DAT_009ec458.wrapping_add(v);
        }
        DAT_009a7bb2 = DAT_009a7bb2.wrapping_sub(1);
        if DAT_009a7bb2 < 0 {
            return 0;
        }
        if DAT_009d77f0 == DAT_009a7bb2 {
            let v = ((DAT_009fe7d4.wrapping_add(0xe)) as *const i16).read_unaligned();
            DAT_009d114c = v.wrapping_sub(1);
            return 1;
        }
        if DAT_009d8c2c <= DAT_009a7bb2 && DAT_009e9654 < DAT_009d8c2c {
            DAT_009d8c14 = DAT_009d8c14.wrapping_add(1);
            let addr = DAT_009fe7d4
                .wrapping_add(0x1934)
                .wrapping_add((DAT_009d8c14 as i32).wrapping_mul(2));
            let v = (addr as *const i16).read_unaligned();
            DAT_009d114c = DAT_009d114c.wrapping_add(v);
        }
        1
    }
}
