// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0a1e0: i8;
    static mut DAT_009ed240: i8;
    static mut DAT_00a08ca4: i8;
    static mut DAT_00a08ca0: i8;
    static mut DAT_00a08ca2: i8;
    static mut DAT_00a08ca6: i8;
    static mut DAT_009d130c: *const core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004407d0(param_1: u32, param_2: i16, param_3: u32) {
    let u_var2: u32 = param_1 & 0xffff;
    // SAFETY: all globals are the game's own memory, accessed at the same
    // offsets as the original; the function pointer is the game's callback.
    unsafe {
        let p_cur = (&raw mut DAT_00a0a1e0 as *mut u8)
            .byte_add(u_var2.wrapping_mul(2) as usize)
            .cast::<i16>();
        if p_cur.read_unaligned() != param_2 {
            p_cur.write_unaligned(param_2);
            let i_var1: u32 = u_var2.wrapping_mul(0xc);
            let idx = (&raw mut DAT_00a08ca4 as *mut u8)
                .byte_add(i_var1 as usize)
                .cast::<u16>()
                .read_unaligned() as u32;
            let a1 = (&raw mut DAT_009ed240 as *mut u8)
                .byte_add(idx.wrapping_mul(4) as usize)
                .cast::<u32>()
                .read_unaligned();
            let a2 = (&raw mut DAT_00a08ca0 as *mut u8)
                .byte_add(i_var1 as usize)
                .cast::<u16>()
                .read_unaligned();
            let a3 = (&raw mut DAT_00a08ca2 as *mut u8)
                .byte_add(i_var1 as usize)
                .cast::<u16>()
                .read_unaligned();
            let a4 = (&raw mut DAT_00a08ca6 as *mut u8)
                .byte_add(i_var1 as usize)
                .cast::<u16>()
                .read_unaligned();
            let f: unsafe extern "C" fn(u32, u16, u16, u16, i16, u32) =
                core::mem::transmute(DAT_009d130c);
            f(a1, a2, a3, a4, param_2, param_3);
        }
    }
}
