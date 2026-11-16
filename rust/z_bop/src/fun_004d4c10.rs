// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7b2e: i32;
    static mut DAT_00aa7b3a: *mut i32;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004d37e0() -> i32;
    fn FUN_004d48e0() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4c10() {
    let mut local_74: [i32; 4] = [0; 4];
    let mut local_64: [i32; 25] = [0; 25];
    // SAFETY: globals and callees are the game's own; the COM-like object
    // pointer is read and called exactly as the original does.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        if DAT_00aa7b2e != 0 && !DAT_00aa7b3a.is_null() {
            FUN_004d4970();
            FUN_004d6460(local_74.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
            let mut p = local_64.as_mut_ptr();
            let mut i_var2: i32 = 0x19;
            while i_var2 != 0 {
                *p = 0;
                p = p.add(1);
                i_var2 -= 1;
            }
            local_64[0] = 100;
            loop {
                loop {
                    let obj = DAT_00aa7b3a;
                    let vtbl = obj.cast::<*mut i32>().read_unaligned();
                    let f = vtbl.add(0x14) as *const core::ffi::c_void;
                    let f: unsafe extern "stdcall" fn(
                        *mut i32,
                        *mut i32,
                        i32,
                        i32,
                        i32,
                        *mut i32,
                    ) -> i32 = core::mem::transmute(f);
                    i_var2 = f(
                        obj,
                        local_74.as_mut_ptr(),
                        0,
                        0,
                        0x400,
                        local_64.as_mut_ptr(),
                    );
                    if i_var2 == 0 {
                        if c_var1 == 0 {
                            return;
                        }
                        DAT_00aa6cfc = FUN_004d48e0();
                        return;
                    }
                    if i_var2 != -0x7789fe3e {
                        break;
                    }
                    i_var2 = FUN_004d37e0();
                    if i_var2 != 0 {
                        if c_var1 == 0 {
                            return;
                        }
                        DAT_00aa6cfc = FUN_004d48e0();
                        return;
                    }
                }
                if i_var2 != -0x7789fde4 {
                    break;
                }
            }
            if c_var1 != 0 {
                DAT_00aa6cfc = FUN_004d48e0();
            }
        }
    }
}
