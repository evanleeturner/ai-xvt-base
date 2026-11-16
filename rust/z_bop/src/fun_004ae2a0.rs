// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7ed0: [i16; 0];
    static mut DAT_00521f19: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: u32) -> u32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004ae1a0(param_1: u32, param_2: u32) -> *mut u32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae2a0(param_1: i32, mut param_2: i32) -> u32 {
    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's own functions and are read where the original reads.
    unsafe {
        let s = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read();
        if s == 0 {
            return 0;
        }
        if param_2 < 0 {
            return 0;
        }
        let flag = (&raw mut DAT_00521f19)
            .cast::<i8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flag as i32) & 1 == 0 {
            return 0;
        }
        let key = (((s as i32 as u32).wrapping_shl(16)) | ((param_1 & 0xFFFF) as u32)) as u32;
        let pi_var1 = FUN_004ac720(key) as usize as *mut i32;
        if pi_var1.read() as usize as *mut i32 != pi_var1 {
            FUN_00471ff0(pi_var1);
        }
        let arr = pi_var1.byte_add(10).cast::<i32>().read_unaligned() as usize as *const i32;
        let first = arr.read_unaligned();
        let v = (first.wrapping_add(4) as usize as *const i32).read_unaligned();
        if v == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let len = pi_var1.byte_add(6).cast::<i32>().read_unaligned();
        if len <= param_2 {
            param_2 = len.wrapping_sub(1);
        }
        let mut u_var3: u32 = 0;
        let arr2 = pi_var1.byte_add(10).cast::<i32>().read_unaligned() as usize as *const i32;
        let elem = arr2.offset(param_2 as isize).read_unaligned();
        let pu_var2 = FUN_004ae1a0(elem as u32, pi_var1 as usize as u32);
        if !pu_var2.is_null() {
            u_var3 = pu_var2.read_unaligned();
        }
        FUN_004ac740();
        u_var3
    }
}
