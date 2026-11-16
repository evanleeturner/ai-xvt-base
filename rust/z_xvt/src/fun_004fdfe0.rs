// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081c012: [u8; 4];
    static mut DAT_00654cd8: i32;
    static mut DAT_00654c10: i32;
    static mut DAT_00531990: [i8; 1];
    static mut DAT_00534390: [i8; 1];
    static mut DAT_0053439c: [i8; 1];
    static mut DAT_005343a8: [i8; 1];
    static mut DAT_005343b4: [i8; 1];
    static mut DAT_005343c0: [i8; 1];
    static mut DAT_005343cc: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d9ea0(param_1: *mut i32);
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: i32) -> *mut i8;
    fn FUN_004d9d20(param_1: *mut i32, param_2: *mut i8, param_3: *mut i8, param_4: i32);
    fn FUN_004d9a80(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: i32,
        param_6: u32,
        param_7: i32,
        param_8: *mut i8,
    ) -> i32;
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fdfe0() -> u32 {
    let mut local_2c: [i32; 4] = [0; 4];
    let mut local_1c: [i32; 7] = [0; 7];
    // SAFETY: all globals and callees belong to the game; the locals are
    // passed as the original passes them, and the unchecked index into
    // local_1c is kept as in the original.
    unsafe {
        let mut local_bfba = i32::from_ne_bytes(DAT_0081bfba);
        if DAT_0081b850 == 2 && local_bfba != 1 && local_bfba != 2 {
            local_1c[0] = 0;
        } else {
            local_1c[0] = 1;
        }
        local_1c[1] = 1;
        let local_c012 = i32::from_ne_bytes(DAT_0081c012);
        local_1c[2] = (local_c012 == 1) as i32;
        local_1c[3] = 0;
        local_1c[4] = 0;
        local_1c[5] = 0;
        local_1c[6] = 0;
        local_1c.as_mut_ptr().offset(DAT_00654cd8 as isize).write(2);
        FUN_004d9ea0(local_1c.as_mut_ptr());
        let p2c = local_2c.as_mut_ptr();
        FUN_004dcfd0(p2c, 0x16, 0xaa, 0x2a, 0xc2);
        let jewel = &raw mut DAT_00531990 as *mut i8;
        'lab: {
            if DAT_00654cd8 == 2 {
                local_bfba = i32::from_ne_bytes(DAT_0081bfba);
                if local_bfba == 1 {
                    let pc = FUN_004dcfa0(0x12e);
                    FUN_004d9d20(p2c, &raw mut DAT_005343cc as *mut i8, pc, 0xc);
                } else {
                    local_bfba = i32::from_ne_bytes(DAT_0081bfba);
                    if local_bfba == 3 {
                        let pc = FUN_004dcfa0(0x12d);
                        FUN_004d9d20(p2c, &raw mut DAT_005343cc as *mut i8, pc, 0xc);
                    }
                }
            } else {
                let c = i32::from_ne_bytes(DAT_0081c012);
                if c == 1 {
                    let r;
                    local_bfba = i32::from_ne_bytes(DAT_0081bfba);
                    if local_bfba == 1 {
                        let pc = FUN_004dcfa0(0x12e);
                        let s = &raw mut DAT_005343c0 as *mut i8;
                        r = FUN_004d9a80(p2c, s, s, pc, 0xc, 0, 0xe, jewel);
                    } else {
                        local_bfba = i32::from_ne_bytes(DAT_0081bfba);
                        if local_bfba != 3 {
                            break 'lab;
                        }
                        let pc = FUN_004dcfa0(0x12d);
                        let s = &raw mut DAT_005343c0 as *mut i8;
                        r = FUN_004d9a80(p2c, s, s, pc, 0xc, 0, 0xe, jewel);
                    }
                    if r != 0 {
                        DAT_00654c10 = 1;
                        DAT_00654cd8 = 2;
                    }
                }
            }
        }
        FUN_004dd030(p2c, 0, -0x1c);
        if DAT_00654cd8 == 1 {
            let pc = FUN_004dcfa0(299);
            FUN_004d9d20(p2c, &raw mut DAT_005343b4 as *mut i8, pc, 0xc);
        } else {
            let pc = FUN_004dcfa0(299);
            let s = &raw mut DAT_005343a8 as *mut i8;
            let r = FUN_004d9a80(p2c, s, s, pc, 0xc, 0, 0xc, jewel);
            if r != 0 {
                DAT_00654c10 = 1;
                DAT_00654cd8 = 1;
            }
        }
        FUN_004dd030(p2c, 0, -0x1c);
        if local_1c[0] != 0 {
            if DAT_00654cd8 == 0 {
                let pc = FUN_004dcfa0(0x12a);
                FUN_004d9d20(p2c, &raw mut DAT_0053439c as *mut i8, pc, 0xc);
                return 1;
            }
            let pc = FUN_004dcfa0(0x12a);
            let s = &raw mut DAT_00534390 as *mut i8;
            let r = FUN_004d9a80(p2c, s, s, pc, 0xc, 0, 0xb, jewel);
            if r != 0 {
                DAT_00654c10 = 1;
                DAT_00654cd8 = 0;
            }
        }
        1
    }
}
