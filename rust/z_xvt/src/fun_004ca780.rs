// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007806fa: *mut u32;
    static mut DAT_00780706: u32;
    static mut DAT_0078070a: u32;
    static mut DAT_0078070e: u32;
    static mut DAT_00780712: u32;
    static mut DAT_004ca7f0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ca780(param_1: u32) -> u32 {
    let mut local_50: [u32; 0x14] = [0; 0x14];
    // SAFETY: the globals are the game's own memory; DAT_007806fa points at a
    // COM-style object whose vtable holds the function called at offset 0x34.
    unsafe {
        local_50[0] = 0x50;
        local_50[6] = DAT_00780706;
        local_50[7] = DAT_0078070a;
        local_50[8] = DAT_0078070e;
        local_50[9] = DAT_00780712;
        let this = DAT_007806fa;
        let vtable = this.read_unaligned();
        let slot = vtable.wrapping_add(0x34) as *const *const core::ffi::c_void;
        let fp = slot.read_unaligned();
        let f: unsafe extern "stdcall" fn(
            *mut u32,
            *mut u32,
            u32,
            *mut core::ffi::c_void,
            u32,
            u32,
        ) -> i32 = core::mem::transmute(fp);
        let i_var1 = f(
            this,
            local_50.as_mut_ptr(),
            0,
            (&raw mut DAT_004ca7f0).cast::<core::ffi::c_void>(),
            param_1,
            1,
        );
        (((i_var1 == 0) as u32).wrapping_neg()) & 0x64f048
    }
}
