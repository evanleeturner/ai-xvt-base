// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7205: i32;
    static mut DAT_00aa7201: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncmp(s1: *const i8, s2: *const i8, n: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b7970(param_1: *mut i8) {
    // SAFETY: the globals are the game's own table; the table holds
    // DAT_00aa7205 records of 0x44 bytes with room for one more, and
    // param_1 points at a 0x44-byte record, as the original assumes.
    unsafe {
        let mut i_var3: i32 = 0;
        let mut i_var4: i32;
        let mut i_var1: i32;
        if 0 < DAT_00aa7205 {
            i_var4 = 0;
            loop {
                i_var1 = _strncmp(
                    param_1 as *const i8,
                    DAT_00aa7201.cast::<u8>().byte_offset(i_var4 as isize) as *const i8,
                    0x40,
                );
                if i_var1 < 0 {
                    break;
                }
                i_var4 = i_var4.wrapping_add(0x44);
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3 < DAT_00aa7205) {
                    break;
                }
            }
        }
        if i_var3 < DAT_00aa7205 {
            i_var4 = DAT_00aa7205.wrapping_mul(0x44);
            i_var1 = DAT_00aa7205.wrapping_sub(i_var3);
            loop {
                let mut p6 = DAT_00aa7201
                    .cast::<u8>()
                    .byte_offset(i_var4 as isize)
                    .cast::<u32>();
                i_var4 = i_var4.wrapping_sub(0x44);
                let mut p5 = p6.byte_offset(-0x44) as *const u32;
                let mut i_var2: i32 = 0x11;
                while i_var2 != 0 {
                    p6.write_unaligned(p5.read_unaligned());
                    p5 = p5.byte_add(4);
                    p6 = p6.byte_add(4);
                    i_var2 -= 1;
                }
                i_var1 = i_var1.wrapping_sub(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        let mut p5 = DAT_00aa7201
            .cast::<u8>()
            .byte_offset(i_var3.wrapping_mul(0x44) as isize)
            .cast::<u32>();
        let mut src = param_1;
        i_var4 = 0x11;
        while i_var4 != 0 {
            p5.write_unaligned(src.cast::<u32>().read_unaligned());
            src = src.byte_add(4);
            p5 = p5.byte_add(4);
            i_var4 -= 1;
        }
        DAT_00aa7205 = DAT_00aa7205.wrapping_add(1);
    }
}
