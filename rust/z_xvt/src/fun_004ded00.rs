// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759af4: i32;
    static mut DAT_00759af8: i32;
    static mut DAT_00759afc: i32;
}

unsafe extern "C" {
    fn strncmp(a: *const i8, b: *const i8, n: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ded00(param_1: *mut i8) {
    // SAFETY: the globals and the table they point to belong to the game;
    // records are 0x145 bytes and are read and written where the original does.
    unsafe {
        let mut i_var3: i32 = 0;
        if 0 < DAT_00759afc {
            let mut i_var4: i32 = 0;
            loop {
                let i_var1 = strncmp(
                    param_1 as *const i8,
                    DAT_00759af4.wrapping_add(i_var4) as usize as *const i8,
                    0x40,
                );
                if i_var1 < 0 {
                    break;
                }
                i_var4 = i_var4.wrapping_add(0x145);
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3 < DAT_00759afc) {
                    break;
                }
            }
        }
        if i_var3 < DAT_00759afc {
            let mut i_var4: i32 = DAT_00759afc.wrapping_mul(0x145);
            let mut i_var1: i32 = DAT_00759afc.wrapping_sub(i_var3);
            loop {
                let mut p6 = DAT_00759af4.wrapping_add(i_var4) as usize as *mut u32;
                i_var4 = i_var4.wrapping_sub(0x145);
                let mut p5 = (p6 as usize).wrapping_sub(0x145) as *const u32;
                let mut i_var2: i32 = 0x51;
                while i_var2 != 0 {
                    p6.write_unaligned(p5.read_unaligned());
                    p5 = p5.wrapping_add(1);
                    p6 = p6.wrapping_add(1);
                    i_var2 -= 1;
                }
                (p6 as *mut u8).write(*(p5 as *const u8));
                i_var1 = i_var1.wrapping_sub(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        let mut pc7 = i_var3.wrapping_mul(0x145).wrapping_add(DAT_00759af4) as usize as *mut i8;
        let mut p1 = param_1;
        let mut i_var4: i32 = 0x51;
        while i_var4 != 0 {
            (pc7 as *mut u32).write_unaligned((p1 as *const u32).read_unaligned());
            p1 = p1.wrapping_add(4);
            pc7 = pc7.wrapping_add(4);
            i_var4 -= 1;
        }
        *pc7 = *p1;
        DAT_00759afc = DAT_00759afc.wrapping_add(1);
        let mut i_var4: i32 = 0;
        loop {
            let addr = DAT_00759af8.wrapping_add(i_var4) as usize as *mut i32;
            let i_var1 = addr.read_unaligned();
            if i_var3 <= i_var1 {
                addr.write_unaligned(i_var1.wrapping_add(1));
            }
            i_var4 = i_var4.wrapping_add(0xc);
            if !(i_var4 < 0x90) {
                break;
            }
        }
    }
}
