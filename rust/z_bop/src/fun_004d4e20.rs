// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b2e: u32;
    static mut DAT_00aa7b32: *mut core::ffi::c_void;
    static mut DAT_00aa7b13: u32;
    static mut DAT_00aa7b1b: u32;
    static mut DAT_00aa6cfc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4e20() -> u32 {
    // SAFETY: the globals are the game's own; DAT_00aa7b32 is a COM-style
    // object whose first field is a vtable pointer, and the vtable slots
    // 25 and 27 hold stdcall functions. local_6c is a 0x6c-byte struct
    // whose word at offset 0x24 overlaps the C's uStack_48.
    unsafe {
        if DAT_00aa7b2e == 0 {
            return 0;
        }
        if DAT_00aa7b32.is_null() {
            return 0;
        }
        let mut local_6c: [u32; 27] = [0; 27];
        local_6c[0] = 0x6c;
        loop {
            loop {
                let obj = DAT_00aa7b32;
                let vtbl = obj
                    .cast::<*const *const core::ffi::c_void>()
                    .read_unaligned();
                let f25 = vtbl.add(25).read_unaligned();
                let f: unsafe extern "stdcall" fn(
                    *mut core::ffi::c_void,
                    u32,
                    *mut u32,
                    u32,
                    u32,
                ) -> i32 = core::mem::transmute(f25);
                let r = f(obj, 0, local_6c.as_mut_ptr(), 0, 0);
                if r == 0 {
                    DAT_00aa7b13 = DAT_00aa7b1b;
                    DAT_00aa6cfc = core::ptr::read_volatile(&local_6c[9]);
                    return 1;
                }
                if r != -0x7789fe3e {
                    if r == -0x7789fde4 {
                        break;
                    }
                    return 0;
                }
                let obj = DAT_00aa7b32;
                let vtbl = obj
                    .cast::<*const *const core::ffi::c_void>()
                    .read_unaligned();
                let f27 = vtbl.add(27).read_unaligned();
                let g: unsafe extern "stdcall" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f27);
                g(obj);
            }
        }
    }
}
