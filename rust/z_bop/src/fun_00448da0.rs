// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8b6c: *const core::ffi::c_void;
    static mut DAT_005233e0: u32;
    static mut DAT_005233dc: u32;
    static mut DAT_009a20ae: u8;
    static mut DAT_005233e4: u32;
    static mut DAT_005233e8: u32;
    static mut DAT_005236a0: *mut core::ffi::c_void;
    static mut DAT_005233d8: u32;
    static mut DAT_005236a4: u32;
    static mut DAT_00527ed0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9640(param_1: i32);
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn _free(ptr: *mut core::ffi::c_void);
    fn FUN_004abee0();
    fn FUN_004aba50();
    fn FUN_004abe50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00448da0() {
    // SAFETY: the callees and globals are the game's own memory; the
    // function pointer in DAT_009d8b6c is the game's callback with the
    // call site's signature.
    unsafe {
        FUN_004a9640(0);
        let u_var5: u32 = DAT_005233e0.wrapping_sub(DAT_005233dc) >> 1;
        let u_var2: u32 = DAT_009a20ae as u32;
        let u_var1: u32 = DAT_005233e0.wrapping_add(DAT_005233dc);
        let u_var4: u32 = DAT_005233e4.wrapping_add(DAT_005233e8);
        let u_var6: u32 = u_var5.wrapping_add(2);
        let i_var7: u32 = u_var2.wrapping_mul(5).wrapping_add(2);

        if DAT_005236a0.is_null() {
            DAT_005236a0 = _malloc(DAT_005233d8.wrapping_mul(i_var7).wrapping_mul(u_var6));
        } else {
            let u_var3 = DAT_005233d8.wrapping_mul(i_var7).wrapping_mul(u_var6);
            if u_var3 > DAT_005236a4 {
                _free(DAT_005236a0);
                DAT_005236a0 = _malloc(DAT_005233d8.wrapping_mul(i_var7).wrapping_mul(u_var6));
            }
        }

        if DAT_005236a0.is_null() {
            return;
        }
        DAT_005236a4 = DAT_005233d8.wrapping_mul(i_var7).wrapping_mul(u_var6);

        FUN_004abee0();
        DAT_00527ed0 = 0;
        FUN_004aba50();
        let f: unsafe extern "C" fn(*mut core::ffi::c_void, u32, u32, u32, u32) =
            core::mem::transmute(DAT_009d8b6c);
        f(
            DAT_005236a0,
            ((u_var1 >> 1).wrapping_sub(u_var5 / 2)).wrapping_sub(1) & 0xffff,
            ((u_var4 >> 1).wrapping_sub(u_var2.wrapping_mul(5) / 2)).wrapping_sub(1) & 0xffff,
            u_var6 & 0xffff,
            i_var7,
        );
        FUN_004abe50();
        DAT_00527ed0 = 1;
        FUN_004abee0();
    }
}
