// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233d8: i32;
    static mut DAT_005233f4: i32;
    static mut DAT_005505e4: *mut i32;
    static mut DAT_005505e0: *mut i32;
    static mut DAT_009d8c08: i32;
    static mut DAT_009cc460: [i32; 1];
    static mut DAT_0054f9e0: [i32; 1];
    static mut _DAT_0054f9b0: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac7a0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e0c0(param_1: i32, param_2: i32, param_3: u32, param_4: i32) {
    // SAFETY: all globals and the callee are the game's own memory; the
    // arrays are written at the indices the original uses.
    unsafe {
        let i_var1 = DAT_005233d8;
        if param_1 == 0 {
            DAT_005233f4 = FUN_004ac7a0();
            DAT_005505e4 = &raw mut DAT_009d8c08;
            DAT_005505e0 = (&raw mut DAT_009cc460).cast::<i32>();
            return;
        }
        DAT_005233f4 = param_1;
        if param_4 != -1 {
            let arr = (&raw mut DAT_0054f9e0).cast::<i32>();
            let mut i: u32 = 0;
            if param_3 != 0 {
                loop {
                    arr.add(i as usize)
                        .write(param_2.wrapping_mul(i_var1).wrapping_mul(i as i32));
                    i = i.wrapping_add(1);
                    if i >= param_3 {
                        break;
                    }
                }
            }
            _DAT_0054f9b0 = param_4;
            DAT_005505e4 = &raw mut _DAT_0054f9b0;
            DAT_005505e0 = arr;
            return;
        }
        let arr = (&raw mut DAT_009cc460).cast::<i32>();
        let mut i: u32 = 0;
        if param_3 != 0 {
            loop {
                arr.add(i as usize)
                    .write(DAT_009d8c08.wrapping_mul(i as i32));
                i = i.wrapping_add(1);
                if i >= param_3 {
                    break;
                }
            }
        }
        DAT_005505e4 = &raw mut DAT_009d8c08;
        DAT_005505e0 = arr;
    }
}
