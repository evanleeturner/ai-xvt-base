// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00acf571: [i32; 0];
    static mut DAT_00acf581: [i32; 0];
    static mut DAT_00acebc5: i32;
    static mut DAT_00b53575: i32;
    static mut DAT_00b53579: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004ce130();
    fn FUN_004d48e0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cffb0() -> i32 {
    // SAFETY: the globals and callees are the game's own memory and code; the
    // array reads are unaligned loads at the same offsets the original uses.
    unsafe {
        let c_var2 = DAT_00aa7b0d;
        if DAT_00acebc5 == 0 {
            return 0;
        }
        FUN_004d4970();
        FUN_004ce130();
        let mut i_var3 = DAT_00b53575;
        let mut i_var1 = DAT_00b53579;
        if i_var1 < 0x201 {
            loop {
                if i_var1 < 1 {
                    if c_var2 != 0 {
                        DAT_00aa6cfc = FUN_004d48e0();
                    }
                    return 0;
                }
                let idx = i_var3.wrapping_mul(0x84) as isize;
                let a = (&raw mut DAT_00acf571)
                    .cast::<i32>()
                    .offset(idx)
                    .read_unaligned();
                if a == 0 {
                    let b = (&raw mut DAT_00acf581)
                        .cast::<i32>()
                        .offset(idx)
                        .read_unaligned();
                    if b == 3 {
                        break;
                    }
                }
                i_var3 = i_var3.wrapping_add(1);
                if 0x3ff < i_var3 {
                    i_var3 = 0;
                }
                i_var1 = i_var1.wrapping_sub(1);
            }
            if c_var2 != 0 {
                DAT_00aa6cfc = FUN_004d48e0();
            }
            return 1;
        }
        if c_var2 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        1
    }
}
