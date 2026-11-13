// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7ed0: [i16; 1];
    static mut DAT_00521f19: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param: i32) -> *mut i32;
    fn FUN_00471ff0(param: *mut i32);
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004adc40(param_1: i32) -> i32 {
    // SAFETY: the globals, arrays and callees are the game's own memory;
    // pointers returned by the game are read where the original reads.
    unsafe {
        let a = (&raw mut DAT_009a7ed0).cast::<i16>();
        let s = a.offset(param_1 as isize).read_unaligned();
        if s == 0 {
            return 0;
        }
        let b = (&raw mut DAT_00521f19).cast::<u8>();
        if (b.offset(param_1.wrapping_mul(0x1a) as isize).read() & 1) == 0 {
            return 0;
        }
        let arg = (((param_1 as u32) & 0xFFFF0000) | (s as i32 as u32)) as i32;
        let pi_var1 = FUN_004ac720(arg);
        if pi_var1.read_unaligned() != (pi_var1 as usize as u32 as i32) {
            FUN_00471ff0(pi_var1);
        }
        let mut i_var2 = pi_var1.byte_add(6).cast::<i32>().read_unaligned();
        let inner = pi_var1.byte_add(10).cast::<i32>().read_unaligned();
        if ((inner as u32 as usize + 4) as *const i32).read_unaligned() == 0x14 {
            i_var2 = i_var2.wrapping_sub(1);
        }
        FUN_004ac740();
        if 0x32 < i_var2 {
            i_var2 = 0x32;
        }
        i_var2
    }
}
