// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fbf: i32;
    static mut DAT_00aa7fc3: i32;
    static mut DAT_00aa7fc7: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncmp(a: *const i8, b: *const i8, n: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df070(param_1: *mut i8) {
    // SAFETY: the globals and the table they point at are the game's own
    // memory; param_1 points at a caller-owned record of 0x145 bytes.
    unsafe {
        let mut i_var3: i32 = 0;
        if 0 < DAT_00aa7fc7 {
            let mut i_var4: i32 = 0;
            loop {
                let i_var1 = _strncmp(
                    param_1 as *const i8,
                    DAT_00aa7fbf.wrapping_add(i_var4) as *const i8,
                    0x40,
                );
                if i_var1 < 0 {
                    break;
                }
                i_var4 = i_var4.wrapping_add(0x145);
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3 < DAT_00aa7fc7) {
                    break;
                }
            }
        }
        if i_var3 < DAT_00aa7fc7 {
            let mut i_var4: i32 = DAT_00aa7fc7.wrapping_mul(0x145);
            let mut i_var1: i32 = DAT_00aa7fc7.wrapping_sub(i_var3);
            loop {
                let mut p6 = DAT_00aa7fbf.wrapping_add(i_var4) as *mut u8;
                i_var4 = i_var4.wrapping_sub(0x145);
                let mut p5 = (p6 as i32).wrapping_sub(0x145) as *const u8;
                let mut i_var2: i32 = 0x51;
                while i_var2 != 0 {
                    p6.cast::<i32>()
                        .write_unaligned(p5.cast::<i32>().read_unaligned());
                    p5 = p5.byte_add(4);
                    p6 = p6.byte_add(4);
                    i_var2 -= 1;
                }
                *p6 = *p5;
                i_var1 = i_var1.wrapping_sub(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        let mut i_var1: i32 = 0;
        let mut pc7 = i_var3.wrapping_mul(0x145).wrapping_add(DAT_00aa7fbf) as *mut u8;
        let mut src = param_1 as *const u8;
        let mut i_var4: i32 = 0x51;
        while i_var4 != 0 {
            pc7.cast::<i32>()
                .write_unaligned(src.cast::<i32>().read_unaligned());
            src = src.byte_add(4);
            pc7 = pc7.byte_add(4);
            i_var4 -= 1;
        }
        *pc7 = *src;
        DAT_00aa7fc7 = DAT_00aa7fc7.wrapping_add(1);
        loop {
            let addr = DAT_00aa7fc3.wrapping_add(i_var1) as *mut i32;
            let v = addr.read_unaligned();
            if i_var3 <= v {
                addr.write_unaligned(v.wrapping_add(1));
            }
            i_var1 = i_var1.wrapping_add(0xc);
            if !(i_var1 < 0x90) {
                break;
            }
        }
    }
}
