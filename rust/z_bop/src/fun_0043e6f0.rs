// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0a1e0: [u8; 0];
    static mut DAT_00a08ca0: [u8; 0];
    static mut DAT_00a08ca2: [u8; 0];
    static mut DAT_00a08ca4: [u8; 0];
    static mut DAT_009d80c4: *const core::ffi::c_void;
    static mut DAT_009ed240: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0043e6f0(param_1: u16, param_2: u32, param_3: u16, param_4: i16) {
    let u_var3: u32 = param_2 & 0xffff;
    // SAFETY: the globals are the game's own tables and function pointer;
    // reads, writes and the call happen in the original's order.
    unsafe {
        let slot = (&raw mut DAT_00a0a1e0)
            .cast::<u8>()
            .byte_add((u_var3 as usize) * 2)
            .cast::<u16>();
        if slot.read_unaligned() != param_1 {
            let mut u_var6: u16 = 0;
            slot.write_unaligned(param_1);
            let i_var4: usize = (u_var3 as usize).wrapping_mul(0xc);
            let u_var1: u16 = (&raw mut DAT_00a08ca0)
                .cast::<u8>()
                .byte_add(i_var4)
                .cast::<u16>()
                .read_unaligned();
            let mut s_var5: i16 = (&raw mut DAT_00a08ca2)
                .cast::<u8>()
                .byte_add(i_var4)
                .cast::<i16>()
                .read_unaligned();
            let u_var2: u16 = (&raw mut DAT_00a08ca4)
                .cast::<u8>()
                .byte_add(i_var4)
                .cast::<u16>()
                .read_unaligned();
            if param_3 != 0 {
                loop {
                    let b_var7: u32 = (u_var6 < param_1) as u32;
                    u_var6 = u_var6.wrapping_add(1);
                    let idx = (b_var7.wrapping_add(u_var2 as u32) as usize).wrapping_mul(4);
                    let p = (&raw mut DAT_009ed240)
                        .cast::<u8>()
                        .byte_add(idx)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned();
                    let f: unsafe extern "C" fn(*mut core::ffi::c_void, u16, i16, i32, i32) =
                        core::mem::transmute(DAT_009d80c4);
                    f(p, u_var1, s_var5, 0xfd, 0);
                    s_var5 = s_var5.wrapping_sub(param_4);
                    if !(u_var6 < param_3) {
                        break;
                    }
                }
            }
        }
    }
}
