// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758d45: i32;
    static mut DAT_00758d41: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncmp(a: *const i8, b: *const i8, n: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d3470(param_1: *mut i8) {
    // SAFETY: the globals and the strncmp callee are the game's own; the
    // table at DAT_00758d41 holds 0x44-byte entries and param_1 points at a
    // 0x44-byte entry, read and written where the original does.
    unsafe {
        let mut i_var3: i32 = 0;
        if 0 < DAT_00758d45 {
            let mut i_var4: i32 = 0;
            loop {
                let i_var1 = _strncmp(
                    param_1 as *const i8,
                    DAT_00758d41.byte_add(i_var4 as isize as usize) as *const i8,
                    0x40,
                );
                if i_var1 < 0 {
                    break;
                }
                i_var4 = i_var4.wrapping_add(0x44);
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3 < DAT_00758d45) {
                    break;
                }
            }
        }
        if i_var3 < DAT_00758d45 {
            let mut i_var4: i32 = DAT_00758d45.wrapping_mul(0x44);
            let mut i_var1: i32 = DAT_00758d45.wrapping_sub(i_var3);
            loop {
                let mut dst = DAT_00758d41.byte_offset(i_var4 as isize).cast::<u32>();
                i_var4 = i_var4.wrapping_sub(0x44);
                let mut src = dst.wrapping_sub(0x11) as *const u32;
                let mut c = 0x11;
                while c != 0 {
                    dst.write_unaligned(src.read_unaligned());
                    src = src.wrapping_add(1);
                    dst = dst.wrapping_add(1);
                    c -= 1;
                }
                i_var1 = i_var1.wrapping_sub(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        let mut dst = DAT_00758d41
            .byte_offset(i_var3.wrapping_mul(0x44) as isize)
            .cast::<u32>();
        let mut p = param_1;
        let mut c = 0x11;
        while c != 0 {
            dst.write_unaligned(p.cast::<u32>().read_unaligned());
            p = p.wrapping_add(4);
            dst = dst.wrapping_add(1);
            c -= 1;
        }
        DAT_00758d45 = DAT_00758d45.wrapping_add(1);
    }
}
