// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ef60: i32;
    static mut DAT_0081bcd0: u32;
    static mut DAT_005313a0: [i8; 1];
    static mut DAT_00531398: [i8; 1];
    static mut DAT_00531384: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ddc70(a: *mut u32, b: *mut u32);
    fn FUN_004d0500(a: *mut i8, b: *mut u32);
    fn FUN_004dd000(a: *mut i32, b: *mut i32);
    fn FUN_004d0580(a: *mut i8, b: i32, c: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32) -> u32;
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004daac0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d52b0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bc8b0(param_1: *mut u8, param_2: *mut i32, param_3: i32) {
    let mut local_28: i32 = 0;
    let local_24: i32 = 0;
    let mut local_20: i32;
    let mut local_18: u32 = 0;
    let mut local_14: u32 = 0;
    let mut local_10: [u32; 2] = [0; 2];
    let local_8: i32 = 0;
    // SAFETY: the callees and globals are the game's own; param_1 and
    // param_2 are the caller's valid pointers, used as the original does.
    unsafe {
        FUN_004ddc70(&raw mut local_18, &raw mut local_14);
        FUN_004d0500((&raw mut DAT_005313a0).cast::<i8>(), local_10.as_mut_ptr());
        FUN_004dd000(&raw mut local_28, param_2);
        let mut u_var1: u32 = 0;
        loop {
            local_20 = local_8
                .wrapping_sub(local_10[0] as i32)
                .wrapping_add(local_28)
                .wrapping_add(1);
            if DAT_0064ef60 != 0 {
                FUN_004d52b0();
                let pc = if u_var1 == 0 {
                    (&raw mut DAT_005313a0).cast::<i8>()
                } else {
                    (&raw mut DAT_00531398).cast::<i8>()
                };
                FUN_004d0580(pc, local_28, local_24);
                FUN_004d5350(0);
                let pc = if u_var1 == 0 {
                    (&raw mut DAT_005313a0).cast::<i8>()
                } else {
                    (&raw mut DAT_00531398).cast::<i8>()
                };
                FUN_004d0580(pc, local_28, local_24);
            }
            if u32::from(param_1.read()) == u_var1 {
                FUN_004d0e60((&raw mut DAT_00531384).cast::<i8>(), local_28, local_24);
            }
            local_28 = local_20.wrapping_add(5);
            let u_var2 = u_var1.wrapping_add(1);
            local_20 = local_20.wrapping_add(0x69);
            let _ = local_20;
            let u_var7 = DAT_0081bcd0;
            let pc = FUN_004dcfa0((u_var1 as i32).wrapping_add(param_3));
            FUN_004daac0(0xc, pc, &raw mut local_28, 0, 1, u_var7);
            local_28 = param_2.read_unaligned().wrapping_add(0x82);
            u_var1 = u_var2;
            if (u_var2 as i32) >= 2 {
                break;
            }
        }
    }
}
