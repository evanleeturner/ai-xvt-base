// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetPixel(hdc: *mut c_void, x: i32, y: i32) -> u32;
    fn SetPixel(hdc: *mut c_void, x: i32, y: i32, color: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f1070(param_1: *mut i32, param_2: u32) -> u32 {
    let mut local_70: *mut c_void = core::ptr::null_mut();
    // The decompiler's local_6c[9] .. puStack_48 .. bStack_18 are one 0x6c-byte
    // surface description; puStack_48 sits at offset 0x24, bStack_18 at 0x54.
    let mut desc: [u32; 27] = [0; 27];
    let base = desc.as_mut_ptr().cast::<u8>();
    let mut u_var2: u32 = 0xffff_ffff;

    // SAFETY: param_1 is a COM-style object whose first word is a vtable
    // pointer; the vtable slots, GetPixel and SetPixel are the game's/system's
    // own; desc is a local buffer large enough for the surface description.
    unsafe {
        base.add(0x24).cast::<u32>().write_unaligned(0);
        base.add(0x54).write(0);

        let vtbl = param_1.read() as usize;
        let mut color: u32 = base.cast::<u32>().read_unaligned();

        if param_2 != 0xffff_ffff {
            let f44: unsafe extern "C" fn(*mut i32, *mut *mut c_void) -> i32 =
                core::mem::transmute(((vtbl + 0x44) as *const usize).read_unaligned());
            let r = f44(param_1, &raw mut local_70);
            color = base.cast::<u32>().read_unaligned();
            if r == 0 {
                color = GetPixel(local_70, 0, 0);
                SetPixel(local_70, 0, 0, param_2);
                let f68: unsafe extern "C" fn(*mut i32, *mut c_void) =
                    core::mem::transmute(((vtbl + 0x68) as *const usize).read_unaligned());
                f68(param_1, local_70);
            }
        }

        base.cast::<u32>().write_unaligned(0x6c);
        let mut i_var1: i32;
        loop {
            let f64_: unsafe extern "C" fn(*mut i32, i32, *mut u32, i32, i32) -> i32 =
                core::mem::transmute(((vtbl + 0x64) as *const usize).read_unaligned());
            i_var1 = f64_(param_1, 0, base.cast::<u32>(), 0, 0);
            if i_var1 != -0x7789fde4 {
                break;
            }
        }
        if i_var1 == 0 {
            let p = base.add(0x24).cast::<usize>().read_unaligned() as *const u32;
            let b = base.add(0x54).read() as u32;
            u_var2 = (1u32.wrapping_shl(b & 0x1f).wrapping_sub(1)) & p.read_unaligned();
            let f80: unsafe extern "C" fn(*mut i32, i32) =
                core::mem::transmute(((vtbl + 0x80) as *const usize).read_unaligned());
            f80(param_1, 0);
        }

        if param_2 != 0xffff_ffff {
            let f44: unsafe extern "C" fn(*mut i32, *mut *mut c_void) -> i32 =
                core::mem::transmute(((vtbl + 0x44) as *const usize).read_unaligned());
            let r = f44(param_1, &raw mut local_70);
            if r == 0 {
                SetPixel(local_70, 0, 0, color);
                let f68: unsafe extern "C" fn(*mut i32, *mut c_void) =
                    core::mem::transmute(((vtbl + 0x68) as *const usize).read_unaligned());
                f68(param_1, local_70);
            }
        }
    }
    u_var2
}
