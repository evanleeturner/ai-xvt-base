// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [i8; 1];
    static mut DAT_009a7ed0: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004ae1a0(param_1: i32, param_2: i32) -> i32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004aeec0(param_1: i32, mut param_2: i32) -> u32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own memory; all pointers
    // are read where the original reads them.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .cast::<i8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flag & 1) == 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read() as u16;
        let arg = ((((param_1.wrapping_mul(5)) as u32) >> 16) << 16) | (w as u32);
        let pi1 = FUN_004ac720(arg as i32) as *mut i32;
        if pi1.read() != (pi1 as usize as i32) {
            FUN_00471ff0(pi1);
        }
        let ptr1 = pi1.cast::<u8>().byte_add(10).cast::<i32>().read_unaligned();
        let ptr2 = (ptr1 as usize as *const i32).read_unaligned();
        let val_check = (ptr2 as usize as *const u8)
            .byte_add(4)
            .cast::<i32>()
            .read_unaligned();
        if val_check == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let val6 = pi1.cast::<u8>().byte_add(6).cast::<i32>().read_unaligned();
        if val6 <= param_2 {
            param_2 = val6.wrapping_sub(1);
        }
        let mut u_var3: u32 = 0;
        let arr_val = (ptr1 as usize as *const u8)
            .offset(param_2.wrapping_mul(4) as isize)
            .cast::<i32>()
            .read_unaligned();
        let i_var2 = FUN_004ae1a0(arr_val, pi1 as usize as i32);
        if i_var2 != 0 {
            let v = (i_var2 as usize as *const u8)
                .byte_add(4)
                .cast::<u32>()
                .read_unaligned();
            u_var3 = v & 2;
        }
        FUN_004ac740();
        u_var3
    }
}
