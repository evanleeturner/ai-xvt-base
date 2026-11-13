// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetPixel(hdc: *mut core::ffi::c_void, x: i32, y: i32) -> u32;
    fn SetPixel(hdc: *mut core::ffi::c_void, x: i32, y: i32, color: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004edeb0(param_1: *mut i32, param_2: u32) -> u32 {
    // SAFETY: param_1 is the caller's COM-style object whose first word is a
    // vtable pointer; the slots called are read and invoked where the original
    // does. The 0x6c-byte buffer is the stack structure the original passes to
    // the object, with the pointer at offset 0x24 and the byte at 0x54.
    unsafe {
        let mut buf: [u32; 27] = [0; 27];
        let mut local_70: *mut core::ffi::c_void = core::ptr::null_mut();
        let mut u_var2: u32 = 0xffff_ffff;
        let mut color: u32 = buf[0];

        let vtbl = param_1.read() as u32 as usize as *const u8;
        let slot44: *const core::ffi::c_void =
            vtbl.byte_add(0x44).cast::<usize>().read_unaligned() as *const core::ffi::c_void;
        let slot68: *const core::ffi::c_void =
            vtbl.byte_add(0x68).cast::<usize>().read_unaligned() as *const core::ffi::c_void;
        let _ = (slot44, slot68);

        if param_2 != 0xffff_ffff {
            let vtbl = param_1.read() as u32 as usize as *const u8;
            let f44: unsafe extern "C" fn(*mut i32, *mut *mut core::ffi::c_void) -> i32 =
                core::mem::transmute(vtbl.byte_add(0x44).cast::<usize>().read_unaligned());
            let i_var1 = f44(param_1, &raw mut local_70);
            color = buf[0];
            if i_var1 == 0 {
                color = GetPixel(local_70, 0, 0);
                SetPixel(local_70, 0, 0, param_2);
                let vtbl = param_1.read() as u32 as usize as *const u8;
                let f68: unsafe extern "C" fn(*mut i32, *mut core::ffi::c_void) =
                    core::mem::transmute(vtbl.byte_add(0x68).cast::<usize>().read_unaligned());
                f68(param_1, local_70);
            }
        }

        buf[0] = 0x6c;
        let mut i_var1: i32;
        loop {
            let vtbl = param_1.read() as u32 as usize as *const u8;
            let f64_: unsafe extern "C" fn(*mut i32, i32, *mut u32, i32, i32) -> i32 =
                core::mem::transmute(vtbl.byte_add(0x64).cast::<usize>().read_unaligned());
            i_var1 = f64_(param_1, 0, buf.as_mut_ptr(), 0, 0);
            if i_var1 != -0x7789fde4 {
                break;
            }
        }
        if i_var1 == 0 {
            let p = buf
                .as_ptr()
                .cast::<u8>()
                .byte_add(0x24)
                .cast::<usize>()
                .read_unaligned() as *const u32;
            let b = buf.as_ptr().cast::<u8>().byte_add(0x54).read();
            u_var2 = 1u32.wrapping_shl((b & 0x1f) as u32).wrapping_sub(1) & p.read_unaligned();
            let vtbl = param_1.read() as u32 as usize as *const u8;
            let f80: unsafe extern "C" fn(*mut i32, i32) =
                core::mem::transmute(vtbl.byte_add(0x80).cast::<usize>().read_unaligned());
            f80(param_1, 0);
        }

        if param_2 != 0xffff_ffff {
            let vtbl = param_1.read() as u32 as usize as *const u8;
            let f44: unsafe extern "C" fn(*mut i32, *mut *mut core::ffi::c_void) -> i32 =
                core::mem::transmute(vtbl.byte_add(0x44).cast::<usize>().read_unaligned());
            let i_var1 = f44(param_1, &raw mut local_70);
            if i_var1 == 0 {
                SetPixel(local_70, 0, 0, color);
                let vtbl = param_1.read() as u32 as usize as *const u8;
                let f68: unsafe extern "C" fn(*mut i32, *mut core::ffi::c_void) =
                    core::mem::transmute(vtbl.byte_add(0x68).cast::<usize>().read_unaligned());
                f68(param_1, local_70);
            }
        }
        u_var2
    }
}
