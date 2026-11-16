// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527eac: i32;
    static mut DAT_0066e710: i32;
    static mut DAT_00803f90: i32;
    static mut DAT_0066ddc4: i32;
    static mut DAT_005233e0: i32;
    static mut DAT_0066dde0: i32;
    static mut DAT_005233e8: i32;
    static mut DAT_00803f80: *mut *mut core::ffi::c_void;
    static mut DAT_0066ddcc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004aba10() -> i32;
    fn FUN_004ac460() -> i32;
    fn FUN_004aafe0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ac260() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the copy and the COM-style call mirror the original exactly.
    unsafe {
        if DAT_00527eac == 0 {
            let mut i_var2 = FUN_004aba10();
            let mut src = &raw mut DAT_0066e710 as *mut i32;
            let mut dst = &raw mut DAT_00803f90 as *mut i32;
            i_var2 = (i_var2.wrapping_mul(0xf) & 0x7FFFFFFF).wrapping_shl(3);
            while i_var2 != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                i_var2 = i_var2.wrapping_sub(1);
            }
        } else {
            let mut local_64: [i32; 25] = [0; 25];
            local_64[0] = 100;
            local_64[2] = 0xcc0020;
            loop {
                let mut rect1: [u32; 4] = [0; 4];
                let mut rect2: [i32; 4] = [0; 4];
                rect1[0] = (DAT_0066ddc4.wrapping_sub(DAT_005233e0) as u32) >> 1;
                rect2[0] = 0;
                rect2[1] = 0;
                rect2[2] = DAT_005233e0;
                rect1[1] = (DAT_0066dde0.wrapping_sub(DAT_005233e8) as u32) >> 1;
                rect1[2] = rect1[0].wrapping_add(DAT_005233e0 as u32);
                rect1[3] = rect1[1].wrapping_add(DAT_005233e8 as u32);
                rect2[3] = DAT_005233e8;

                let obj = DAT_00803f80;
                let vtbl = *obj;
                let fp = vtbl
                    .byte_add(0x14)
                    .cast::<*const core::ffi::c_void>()
                    .read_unaligned();
                let f: unsafe extern "stdcall" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    i32,
                    *mut core::ffi::c_void,
                    u32,
                    *mut core::ffi::c_void,
                ) -> i32 = core::mem::transmute(fp);
                let i_var2 = f(
                    obj as *mut core::ffi::c_void,
                    rect1.as_mut_ptr() as *mut core::ffi::c_void,
                    DAT_0066ddcc,
                    rect2.as_mut_ptr() as *mut core::ffi::c_void,
                    0x20000,
                    local_64.as_mut_ptr() as *mut core::ffi::c_void,
                );
                if i_var2 == 0 {
                    break;
                }
                if i_var2 == -0x7789fe3e {
                    let b_var1 = FUN_004ac460();
                    if b_var1 == 0 {
                        return;
                    }
                    FUN_004aafe0();
                }
                if i_var2 != -0x7789fde4 {
                    return;
                }
            }
        }
    }
}
