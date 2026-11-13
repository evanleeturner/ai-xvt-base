// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08ee2: u16;
    static mut DAT_00a08ee0: u16;
    static mut DAT_00a08374: i16;
    static mut DAT_009e9670: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a0a240: u16;
    static mut DAT_009a20ae: i16;
    static mut DAT_005215e0: i32;
    static mut DAT_009d1300: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9640(param_1: i8);
    fn FUN_004278c0(param_1: *mut core::ffi::c_void) -> u64;
    fn FUN_004a9520(param_1: u16, param_2: u16, param_3: i16, param_4: i16);
    fn FUN_004a9500(param_1: u16, param_2: u16);
    fn FUN_004a95c0(param_1: u32);
    fn FUN_004a9590(param_1: u32);
    fn FUN_004277f0(param_1: u16, param_2: u32, param_3: u32);
    fn FUN_00440760(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043ee80() {
    // SAFETY: all globals and callees are the game's own; the pointers
    // chased below are the game's data structures, read where the original does.
    unsafe {
        let u_var3 = DAT_00a08ee2;
        let u_var2 = DAT_00a08ee0;
        if DAT_00a08374 == 0 {
            let p0 = ((&raw mut DAT_009e9670) as usize as i32)
                .wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd));
            let p1 = (p0 as usize as *const i32).read_unaligned();
            let p2 = p1
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8);
            let p3 = (p2 as usize as *const i32).read_unaligned();
            let p4 = p3.wrapping_add(0xa9);
            let p5 = (p4 as usize as *const i32).read_unaligned();
            let u_var1 = ((p5.wrapping_add(0x247)) as usize as *const u8).read() as u16;

            if DAT_00a0a240 != u_var1 {
                if (DAT_00a08ee0 as u32).wrapping_add(DAT_00a08ee2 as u32) == 0 {
                    return;
                }
                FUN_004a9640(2);
                let s_var4 = DAT_009a20ae.wrapping_add(u_var3 as i16);
                let u_var5 = FUN_004278c0((&raw mut DAT_005215e0).cast());
                FUN_004a9520(
                    u_var2,
                    u_var3,
                    (u_var5 as i16).wrapping_add(u_var2 as i16),
                    s_var4,
                );
                FUN_004a9500(u_var2, u_var3);
                FUN_004a95c0(0x2c);
                let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
                f();
                FUN_004a9590(0x4e);
                FUN_004277f0(u_var1, 3, 1);
                let idx = ((DAT_00a08374 as i32 as u32).wrapping_mul(2)) as usize;
                (&raw mut DAT_00a0a240)
                    .wrapping_add(idx)
                    .write_unaligned(u_var1);
            }

            if ((p5.wrapping_add(0x248)) as usize as *const i8).read() != 0 {
                FUN_00440760(0x2f, 1);
                return;
            }
            FUN_00440760(0x2f, 0);
        }
    }
}
