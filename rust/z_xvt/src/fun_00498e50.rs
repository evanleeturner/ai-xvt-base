// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e798: u8;
    static mut DAT_0085e79e: u8;
    static mut DAT_0081bd0e: i32;
    static mut DAT_0072f4e4: u8;
    static mut DAT_0073257c: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_00709d0a: u8;
    static mut DAT_00709d0b: u8;
    static mut DAT_006d4a02: u8;
    static mut DAT_0071e528: i32;
    static mut DAT_006da844: u32;
    static mut DAT_0071b73c: u8;
    static mut DAT_0071b73a: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004392c0(param_1: u32, param_2: u32, param_3: u8) -> u32;
    fn FUN_00439530(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00498e50() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the pointer reads stay at the offsets the original uses.
    unsafe {
        if DAT_0085e798 == 0 {
            return;
        }
        if DAT_0085e79e == 0 {
            return;
        }
        if DAT_0081bd0e != 1 {
            return;
        }
        let mut b_var4: u8 = 0;
        if DAT_0072f4e4 == 2 && DAT_0073257c == 0 {
            let rec = DAT_0071ecf4.wrapping_mul(0x5bd) as isize;
            let u_var2: u32 = (&raw mut DAT_0071b73c)
                .byte_offset(rec)
                .cast::<u16>()
                .read_unaligned() as u32;
            let off = u_var2.wrapping_mul(3) as usize;
            let a = (&raw mut DAT_00709d0a).byte_add(off).read();
            let b = (&raw mut DAT_00709d0b).byte_add(off).read();
            if a == 2 || b == 1 {
                b_var4 = 7;
            } else if a == 1 && DAT_006d4a02 != 2 {
                let s_var1 = (&raw mut DAT_0071b73a)
                    .byte_offset(rec)
                    .cast::<i16>()
                    .read_unaligned();
                let add: u32 = if s_var1 == 1 { 2 } else { 0 };
                if DAT_006d4a02 == 4 {
                    b_var4 = (add + 4) as u8;
                } else {
                    b_var4 = (add + 3) as u8;
                }
            }
            if b_var4 != 0 {
                FUN_004392c0(b_var4 as u32, 0, 0);
                DAT_0071e528 = FUN_00439530(b_var4 as u32 as i32);
                DAT_0073257c = 1;
                DAT_0072f4e4 = b_var4;
                return;
            }
        }
        let d_var3 = timeGetTime();
        DAT_0071e528 = DAT_0071e528.wrapping_sub(d_var3.wrapping_sub(DAT_006da844) as i32);
        DAT_006da844 = d_var3;
        if DAT_0071e528 < 1 {
            FUN_004392c0(2, 0, 0);
            DAT_0071e528 = FUN_00439530(2);
            DAT_0072f4e4 = 2;
        }
    }
}
