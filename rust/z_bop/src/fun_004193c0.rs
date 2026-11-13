// SPDX-License-Identifier: MIT
#![no_std]

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523420: i32;
    static mut DAT_009d8a30: i32;
    static mut DAT_009e9680: i8;
    static mut DAT_009ecc34: i32;
    static mut DAT_00550b98: i32;
    static mut DAT_00550880: i32;
    static mut DAT_00550b8c: i32;
    static mut DAT_009d77d0: u32;
    static mut DAT_009ec468: i8;
    static mut DAT_0052341c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046f660(param_1: i32) -> i32;
    fn FUN_00465390(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004651f0(param_1: i32, param_2: i32);
    fn FUN_0046e040() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004193c0(param_1: i32, param_2: i32) {
    // SAFETY: all globals and callees are the game's own memory; param_2 is
    // an address of the caller's structure, read where the original reads.
    unsafe {
        if ((param_2.wrapping_add(4)) as *const i32).read_unaligned() != DAT_00523420 {
            return;
        }
        let i_var3 = FUN_0046f660(param_1);
        let a = (&raw mut DAT_009d8a30).offset(i_var3 as isize);
        let b = (&raw mut DAT_009e9680).offset(i_var3.wrapping_mul(0x5bd) as isize);
        if a.read() == 0 && b.read() != 0 {
            let mut b_var1: i32 = 0;
            if i_var3 != DAT_009ecc34 {
                let mut pi_var8 = param_2.wrapping_add(8) as *const i32;
                let mut i_var6: i32 = 0;
                let mut b_var2: i32 = b_var1;
                loop {
                    let v = (&raw mut DAT_00550b98)
                        .byte_add(i_var6 as isize as usize)
                        .read_unaligned();
                    if v != pi_var8.read_unaligned() {
                        b_var2 = 1;
                    }
                    i_var6 = i_var6.wrapping_add(4);
                    pi_var8 = pi_var8.wrapping_add(1);
                    if !(i_var6 < 0x40) {
                        break;
                    }
                }
                b_var1 = 0;
                if b_var2 != 0 {
                    let r = FUN_00465390(param_1, DAT_00550880, DAT_00550b8c);
                    if r != 0 {
                        FUN_004651f0(param_1, DAT_00550b8c);
                    }
                    b_var1 = 1;
                }
            }
            let r = FUN_0046e040();
            if r != 0 {
                (&raw mut DAT_009d77d0).offset(i_var3 as isize).write(2);
                if b_var1 == 0 {
                    (&raw mut DAT_009d77d0).offset(i_var3 as isize).write(1);
                }
                let mut u_var7: u32 = 3;
                let mut puvar5 = &raw mut DAT_009d77d0;
                let mut pcvar4 = &raw mut DAT_009e9680;
                let end = &raw mut DAT_009ec468 as usize;
                loop {
                    if pcvar4.read() != 0 {
                        u_var7 &= puvar5.read_unaligned();
                    }
                    puvar5 = puvar5.wrapping_add(1);
                    pcvar4 = pcvar4.wrapping_add(0x5bd);
                    if !((pcvar4 as usize) < end) {
                        break;
                    }
                }
                if u_var7 & 1 != 0 {
                    DAT_0052341c = 0;
                }
            }
        }
    }
}
