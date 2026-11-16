// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005181d0: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421300(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004210a0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004211d0(param_1: *mut f32, param_2: *mut f32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00472360(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let _ = param_1;
    let mut local_50: [f32; 4] = [0.0; 4];
    let mut local_40: [f32; 16] = [0.0; 16];

    // SAFETY: the addresses are the game's own structures, read where the
    // original reads; the callees and the global belong to the game.
    unsafe {
        let p1 = ((param_2.wrapping_add(0x1f)) as *const i32).read_unaligned();
        let p2 = ((p1.wrapping_add(0xa9)) as *const i32).read_unaligned();
        let b = ((p2.wrapping_add(0x30a).wrapping_add(param_4)) as *const u8).read();
        local_50[0] = 0.0;
        local_50[2] = 0.0;
        local_50[3] = (b as f32) * DAT_005181d0;
        local_50[1] = -1.0;
        FUN_00421300(local_40.as_mut_ptr(), local_50.as_mut_ptr());
        FUN_004210a0(
            param_3.wrapping_add(0x44) as *mut f32,
            local_40.as_mut_ptr(),
        );
        FUN_00420fe0(
            param_3.wrapping_add(0x38) as *mut f32,
            local_40.as_mut_ptr(),
        );
        FUN_004211d0(
            param_3.wrapping_add(0x14) as *mut f32,
            local_40.as_mut_ptr(),
        );
    }
}
