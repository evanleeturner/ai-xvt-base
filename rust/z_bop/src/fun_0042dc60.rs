// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a606d4: i32;
    static mut DAT_00a1051c: i8;
    static mut DAT_00a604cc: i32;
    static mut DAT_00a6052c: i32;
}

unsafe extern "C" {
    fn strncmp(a: *const i8, b: *const i8, n: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042dc60(param_1: *mut i8) {
    // SAFETY: the globals are the game's own memory; param_1 points at a
    // 0x145-byte record, copied into the game's record table as the original does.
    unsafe {
        let base: *mut i8 = &raw mut DAT_00a1051c;
        let mut i_var5: i32 = 0;
        if 0 < DAT_00a606d4 {
            let mut pc_var6: *mut i8 = base;
            loop {
                let i_var1 = strncmp(param_1, pc_var6, 0x40);
                if i_var1 < 0 {
                    break;
                }
                pc_var6 = pc_var6.byte_add(0x145);
                i_var5 = i_var5.wrapping_add(1);
                if !(i_var5 < DAT_00a606d4) {
                    break;
                }
            }
        }
        if i_var5 < DAT_00a606d4 {
            let mut i_var1 = DAT_00a606d4.wrapping_sub(i_var5);
            let mut pu_var4: *mut i8 =
                base.byte_offset((DAT_00a606d4.wrapping_mul(0x145)) as isize);
            loop {
                let mut src: *const i8 = pu_var4.byte_offset(-0x145);
                let mut dst: *mut i8 = pu_var4;
                let mut i_var2 = 0x51;
                while i_var2 != 0 {
                    dst.cast::<i32>()
                        .write_unaligned(src.cast::<i32>().read_unaligned());
                    src = src.byte_add(4);
                    dst = dst.byte_add(4);
                    i_var2 -= 1;
                }
                *dst = *src;
                i_var1 = i_var1.wrapping_sub(1);
                pu_var4 = pu_var4.byte_offset(-0x145);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        let mut pc_var6: *mut i8 = base.byte_offset((i_var5.wrapping_mul(0x145)) as isize);
        let mut p1: *mut i8 = param_1;
        let mut i_var1 = 0x51;
        while i_var1 != 0 {
            pc_var6
                .cast::<i32>()
                .write_unaligned(p1.cast::<i32>().read_unaligned());
            p1 = p1.byte_add(4);
            pc_var6 = pc_var6.byte_add(4);
            i_var1 -= 1;
        }
        *pc_var6 = *p1;
        let mut pi_var3: *mut i32 = &raw mut DAT_00a604cc;
        DAT_00a606d4 = DAT_00a606d4.wrapping_add(1);
        let end = (&raw mut DAT_00a6052c) as usize;
        loop {
            if i_var5 <= pi_var3.read_unaligned() {
                pi_var3.write_unaligned(pi_var3.read_unaligned().wrapping_add(1));
            }
            pi_var3 = pi_var3.byte_add(12);
            if !((pi_var3 as usize) < end) {
                break;
            }
        }
    }
}
