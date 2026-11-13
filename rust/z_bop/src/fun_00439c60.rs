// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0a2fe: i16;
    static mut DAT_009a20ae: u16;
    static mut DAT_00a08ca0: i8;
    static mut DAT_00a08ca2: i8;
    static mut DAT_00a08ca6: i8;
    static mut DAT_00a0a0e4: *mut i8;
    static mut DAT_00a0a0e8: *mut i8;
    static mut DAT_00a0a0e0: *mut i8;
    static mut DAT_00a08c34: *mut i8;
    static mut DAT_009a1ed0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a95c0(param_1: u32);
    fn FUN_004a9640(param_1: i8);
    fn FUN_004a9500(param_1: u16, param_2: u16);
    fn FUN_004a9590(param_1: u32);
    fn FUN_004a97f0(param_1: *mut i8);
    fn FUN_004a9860(param_1: u32);
    fn FUN_004278c0(param_1: *mut core::ffi::c_void) -> u64;
    fn FUN_004a9520(param_1: i16, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_004a9960(param_1: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004419d0();
    fn FUN_0043f390();
    fn FUN_00440140();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00439c60() {
    // SAFETY: all globals and callees are the game's own; the table at
    // DAT_00a08ca0 is read at the same byte offsets as the original.
    unsafe {
        if DAT_00a0a2fe == -2 {
            FUN_004419d0();
            let mut u_var7: u32 = 0x8b;
            FUN_004a95c0(0x2c);
            FUN_004a9640(2);
            loop {
                let i_var1: u32 = u_var7.wrapping_mul(0xc);
                let p0 = (&raw mut DAT_00a08ca0).byte_add(i_var1 as usize);
                let p2 = (&raw mut DAT_00a08ca2).byte_add(i_var1 as usize);
                let p6 = (&raw mut DAT_00a08ca6).byte_add(i_var1 as usize);
                FUN_004a9500(
                    p0.cast::<u16>().read_unaligned(),
                    p2.cast::<u16>().read_unaligned(),
                );
                FUN_004a9590(u32::from(p6.cast::<u16>().read_unaligned()));
                let pc_var5: Option<*mut i8> = match u_var7 {
                    0x8b => Some(DAT_00a0a0e4),
                    0x8c => Some(DAT_00a0a0e8),
                    0x8d => Some(DAT_00a0a0e0),
                    0x8e => Some(DAT_00a08c34),
                    _ => None,
                };
                if let Some(p) = pc_var5 {
                    FUN_004a97f0(p);
                }
                let u_var6: u16 = (u_var7 as u16).wrapping_add(1);
                u_var7 = u32::from(u_var6);
                FUN_004a9860(0x3a);
                let s_var2: i16 = p2.cast::<i16>().read_unaligned();
                let s_var3: i16 = p0.cast::<i16>().read_unaligned();
                let s_var4: i16 = (DAT_009a20ae as i16).wrapping_add(s_var2);
                let u_var8 = FUN_004278c0((&raw mut DAT_009a1ed0).cast::<core::ffi::c_void>());
                FUN_004a9520(s_var3, s_var2, (u_var8 as i16).wrapping_add(s_var3), s_var4);
                FUN_004a9960((&raw mut DAT_009a1ed0).cast::<core::ffi::c_void>());
                if u_var6 >= 0x8f {
                    break;
                }
            }
            DAT_00a0a2fe = 0;
        }
        FUN_0043f390();
        FUN_00440140();
    }
}
