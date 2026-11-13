// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 1];
    static mut DAT_009a7ed0: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param_1: *mut core::ffi::c_void);
    fn FUN_004ae1a0(param_1: i32, param_2: *mut core::ffi::c_void) -> i32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004aef60(param_1: i32, mut param_2: i32) -> u32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the globals, callees and the pointers they return are the
    // game's own memory, read and written where the original does.
    unsafe {
        let flags = (&raw mut DAT_00521f19)
            .cast::<u8>()
            .byte_offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flags & 1 == 0 {
            return 0;
        }
        let w = (&raw mut DAT_009a7ed0)
            .cast::<u16>()
            .byte_offset((param_1 as isize).wrapping_mul(2))
            .read();
        let arg = ((((param_1.wrapping_mul(5)) as u32) >> 16) << 16) | (w as u32);
        let pi = FUN_004ac720(arg as i32).cast::<i32>();
        if pi.read() != pi as i32 {
            FUN_00471ff0(pi.cast());
        }
        let arr = pi.cast::<u8>().byte_add(10).cast::<i32>().read_unaligned() as *const i32;
        let first = arr.read_unaligned();
        let v = (first as *const u8)
            .byte_add(4)
            .cast::<i32>()
            .read_unaligned();
        if v == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let count = pi.cast::<u8>().byte_add(6).cast::<i32>().read_unaligned();
        if count <= param_2 {
            param_2 = count.wrapping_sub(1);
        }
        let mut u_var3: u32 = 0;
        let arr2 = pi.cast::<u8>().byte_add(10).cast::<i32>().read_unaligned() as *const i32;
        let elem = arr2.offset(param_2 as isize).read_unaligned();
        let i_var2 = FUN_004ae1a0(elem, pi.cast());
        if i_var2 != 0 {
            u_var3 = (i_var2 as *const u8)
                .byte_add(4)
                .cast::<u32>()
                .read_unaligned()
                & 1;
        }
        FUN_004ac740();
        u_var3
    }
}
