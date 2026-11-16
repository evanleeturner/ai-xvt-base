// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebc5: u32;
    static mut DAT_00acebd1: u32;
    static mut DAT_00acebd5: u32;
    static mut DAT_00acebd9: u32;
    static mut DAT_00acebdd: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cddc0(param_1: u32) -> u8 {
    let mut local_50: [u32; 0x14] = [0; 0x14];
    // SAFETY: the globals are the game's own memory; DAT_00acebc5 holds a
    // pointer to a COM-style object whose first word is a vtable pointer,
    // and the slot at +0x60 is a stdcall function taking (this, ptr, u32).
    unsafe {
        local_50[0] = 0x50;
        local_50[1] = 0x40;
        local_50[6] = DAT_00acebd1;
        local_50[10] = 0x20;
        local_50[7] = DAT_00acebd5;
        local_50[8] = DAT_00acebd9;
        local_50[9] = DAT_00acebdd;
        local_50[12] = param_1;

        let this = DAT_00acebc5;
        let vtable = (this as usize as *const u32).read_unaligned();
        let fp = ((vtable as usize + 0x60) as *const u32).read_unaligned();
        let func: unsafe extern "stdcall" fn(u32, *mut u32, u32) -> i32 =
            core::mem::transmute(fp as usize as *const core::ffi::c_void);
        let i_var2 = func(DAT_00acebc5, local_50.as_mut_ptr(), 2);
        (i_var2 == 0) as u8
    }
}
