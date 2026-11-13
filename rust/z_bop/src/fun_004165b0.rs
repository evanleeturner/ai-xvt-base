// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb7378: i8;
    static mut DAT_00bb737e: i8;
    static mut DAT_00b6a2fe: i32;
    static mut DAT_009fd434: u8;
    static mut DAT_00a004c8: u8;
    static mut DAT_009e967c: u8;
    static mut DAT_009ecc34: u32;
    static mut DAT_009d738a: u8;
    static mut DAT_009d738b: u8;
    static mut DAT_009a2062: u8;
    static mut DAT_009e967a: u8;
    static mut DAT_009ec468: i32;
    static mut DAT_009a7ec4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a50d0(param_1: u32, param_2: u32, param_3: u8);
    fn FUN_004a5340(param_1: i32) -> i32;
}

unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004165b0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the computed addresses point into the game's tables as the original does.
    unsafe {
        if DAT_00bb7378 == 0 {
            return;
        }
        if DAT_00bb737e == 0 {
            return;
        }
        if DAT_00b6a2fe != 1 {
            return;
        }
        let mut b_var4: u8 = 0;
        if DAT_009fd434 == 2 && DAT_00a004c8 == 0 {
            let off = DAT_009ecc34.wrapping_mul(0x5bd) as usize;
            let u_var2 = ((&raw mut DAT_009e967c) as *mut u8)
                .byte_add(off)
                .cast::<u16>()
                .read_unaligned() as usize;
            let p_a = (&raw mut DAT_009d738a) as *mut u8;
            let p_b = (&raw mut DAT_009d738b) as *mut u8;
            if p_a.byte_add(u_var2 * 3).read() == 2 || p_b.byte_add(u_var2 * 3).read() == 1 {
                b_var4 = 7;
            } else if p_a.byte_add(u_var2 * 3).read() == 1 && DAT_009a2062 != 2 {
                let s_var1 = ((&raw mut DAT_009e967a) as *mut u8)
                    .byte_add(off)
                    .cast::<i16>()
                    .read_unaligned();
                let add: u8 = if s_var1 == 1 { 2 } else { 0 };
                if DAT_009a2062 == 4 {
                    b_var4 = add + 4;
                } else {
                    b_var4 = add + 3;
                }
            }
            if b_var4 != 0 {
                FUN_004a50d0(b_var4 as u32, 0, 0);
                DAT_009ec468 = FUN_004a5340(b_var4 as i32);
                DAT_00a004c8 = 1;
                DAT_009fd434 = b_var4;
                return;
            }
        }
        let d_var3 = timeGetTime();
        DAT_009ec468 = (DAT_009ec468 as u32).wrapping_sub(d_var3.wrapping_sub(DAT_009a7ec4)) as i32;
        DAT_009a7ec4 = d_var3;
        if DAT_009ec468 < 1 {
            FUN_004a50d0(2, 0, 0);
            DAT_009ec468 = FUN_004a5340(2);
            DAT_009fd434 = 2;
        }
    }
}
