// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00530: u8;
    static mut DAT_005233c8: u8;
    static mut DAT_00803f80: *mut *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004b32b0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040b750() {
    let mut local_64: [u32; 25] = [0; 25];
    local_64[0] = 100;
    local_64[2] = 0xcc0020;
    // SAFETY: the globals and the callee are the game's own; the index into
    // DAT_00a00530 and the function address at offset 0x14 are used as the
    // original does.
    unsafe {
        let idx = DAT_005233c8 as usize;
        let p = (&raw mut DAT_00a00530).cast::<u8>().byte_add(idx * 2);
        local_64[20] = u32::from(p.cast::<u16>().read_unaligned());
        let f = (*DAT_00803f80).cast::<u8>().byte_add(0x14);
        let func: unsafe extern "stdcall" fn(
            *mut *mut core::ffi::c_void,
            i32,
            i32,
            i32,
            i32,
            *mut u32,
        ) = core::mem::transmute(f.cast::<core::ffi::c_void>());
        func(DAT_00803f80, 0, 0, 0, 0x1000400, local_64.as_mut_ptr());
        FUN_004b32b0();
    }
}
