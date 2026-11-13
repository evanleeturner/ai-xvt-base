// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69d20: i32;
    static mut DAT_0052adec: i8;
    static mut DAT_00bb73a9: i8;
    static mut DAT_00bb74c1: i8;
    static mut DAT_00b69cc4: i32;
    static mut DAT_0052c000: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn _sprintf(param_1: *mut i8, param_2: *mut i8, ...) -> i32;
    fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004db4e0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004d6550(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004da380(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut i8,
    ) -> i32;
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bda30() {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a
    // valid 16-byte rectangle buffer and the text buffer is the game's.
    unsafe {
        let p10 = local_10.as_mut_ptr();
        let buf = &raw mut DAT_00b69d20 as *mut i8;
        let fmt = &raw mut DAT_0052adec;

        FUN_004d6460(p10, 0x54, 0x5a, 0x25c, 0x6a);
        let mut pc = FUN_004ddc10(0x31a);
        let mut i4: i32 = 0;
        let mut i5: i32 = 0;
        FUN_004db640(0xf, pc, p10, 0xffff);
        loop {
            i5 = i5.wrapping_add(1);
            let i2 = i5;
            pc = FUN_004ddc10(0x31b);
            _sprintf(buf, fmt, pc, i2);
            let w = FUN_004dbfa0(buf, 0xc);
            if i4 < w {
                i4 = w;
            }
            if !(i5 < 4) {
                break;
            }
        }
        i5 = 0x7e;
        let mut p1 = &raw mut DAT_00bb73a9;
        FUN_004d6460(p10, i4.wrapping_add(0x5d), 0x7e, 600, 0x92);
        i4 = 0;
        loop {
            let i2 = i4.wrapping_add(1);
            let i8 = i2;
            let p3 = FUN_004ddc10(0x31b);
            _sprintf(buf, fmt, p3, i8);
            FUN_004db4e0(0xc, buf, 0x58, i5, 0xffff);
            FUN_004d6550(p10, 0, 0, DAT_00b69cc4);
            i4 = FUN_004da380(p10, p1, 0x2e, i4, 0xc, core::ptr::null_mut());
            if i4 != 0 {
                DAT_0052c000 = i2;
                if 3 < i2 {
                    DAT_0052c000 = 0;
                }
            }
            i5 = i5.wrapping_add(0x23);
            p1 = p1.wrapping_add(0x46);
            FUN_004d64a0(p10, 0, 0x23);
            i4 = i2;
            if !((p1 as usize) < (&raw mut DAT_00bb74c1 as usize)) {
                break;
            }
        }
    }
}
