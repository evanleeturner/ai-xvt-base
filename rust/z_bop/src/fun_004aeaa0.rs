// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 0];
    static mut DAT_009a7ed0: [u16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param: i32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param: *mut core::ffi::c_void);
    fn FUN_004ae1a0(param1: i32, param2: *mut core::ffi::c_void) -> i32;
    fn __ftol() -> i64;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004aeaa0(param_1: i32, mut param_2: i32) -> u32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals, callees and the object pointer are the game's own
    // memory; offsets are read exactly as the original does.
    unsafe {
        let flag = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if (flag & 1) == 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0)
            .cast::<u16>()
            .offset(param_1 as isize)
            .read();
        let arg = ((param_1.wrapping_mul(5) as u32) >> 16) | u32::from(w);
        let pi = FUN_004ac720(arg as i32).cast::<i32>();
        if pi.read() != pi as i32 {
            FUN_00471ff0(pi.cast());
        }
        let list = pi.byte_add(10).cast::<*mut i32>().read_unaligned();
        if list.byte_add(4).read_unaligned() == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let count = pi.byte_add(6).read_unaligned();
        if count <= param_2 {
            param_2 = count.wrapping_sub(1);
        }
        let mut u_var3: u32 = 0;
        let list = pi.byte_add(10).cast::<*mut i32>().read_unaligned();
        let item = list.offset(param_2 as isize).read_unaligned();
        let i_var2 = FUN_004ae1a0(item, pi.cast());
        if i_var2 != 0 {
            let l_var4 = __ftol();
            u_var3 = l_var4 as u32;
        }
        FUN_004ac740();
        u_var3
    }
}
