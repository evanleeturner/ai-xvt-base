// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664e98: i32;
    static mut DAT_00bb73a7: u8;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_0052a71c: [i8; 1];
    static mut DAT_0052a714: [i8; 1];
    static mut DAT_0052a708: [i8; 1];
    static mut DAT_0052a700: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de090(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004b4cc0(param_1: *mut i8, param_2: *mut i32);
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d4e20();
    fn FUN_004b4d40(param_1: *mut i8, param_2: i32, param_3: i32) -> u32;
    fn FUN_004d4ec0(param_1: i32);
    fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_004dc220() -> i32;
    fn FUN_004dc240() -> i32;
    fn FUN_004de5a0(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8ee0(
    param_1: *mut u8,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    // a[0] = local_28, a[1] = local_24, a[2] = local_20 (adjacent stack slots)
    let mut a: [i32; 3] = [0; 3];
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: [i32; 2] = [0; 2];
    let local_8: i32 = 0;

    // SAFETY: all callees and globals are the game's own; pointers passed point
    // at live locals or the caller's buffers.
    unsafe {
        FUN_004de090(&raw mut local_14, &raw mut local_18);
        FUN_004b4cc0((&raw mut DAT_0052a71c).cast::<i8>(), local_10.as_mut_ptr());
        let mut u_var3: u32 = 0;
        FUN_004d6480(a.as_mut_ptr(), param_2);
        loop {
            a[2] = local_8
                .wrapping_sub(local_10[0])
                .wrapping_add(a[0])
                .wrapping_add(1);
            if DAT_00664e98 != 0 {
                FUN_004d4e20();
                let pc = if u_var3 == 0 {
                    (&raw mut DAT_0052a71c).cast::<i8>()
                } else {
                    (&raw mut DAT_0052a714).cast::<i8>()
                };
                FUN_004b4d40(pc, a[0], a[1]);
                FUN_004d4ec0(0);
                let pc = if u_var3 == 0 {
                    (&raw mut DAT_0052a71c).cast::<i8>()
                } else {
                    (&raw mut DAT_0052a714).cast::<i8>()
                };
                FUN_004b4d40(pc, a[0], a[1]);
            }
            if param_5 == 0
                && FUN_004d6b90(a.as_mut_ptr(), local_14, local_18) != 0
                && (FUN_004dc220() != 0 || FUN_004dc240() != 0)
            {
                if u32::from(param_1.read()) != u_var3 && DAT_00bb73a7 != 0 {
                    FUN_004de5a0((&raw mut DAT_0052a708).cast::<i8>(), 1, 0, 0xff);
                }
                param_1.write(u_var3 as u8);
            }
            if u32::from(param_1.read()) == u_var3 {
                if param_4 == 0 {
                    FUN_004b5570((&raw mut DAT_0052a700).cast::<i8>(), a[0], a[1]);
                } else {
                    FUN_004b4d40((&raw mut DAT_0052a700).cast::<i8>(), a[0], a[1]);
                }
            }
            a[0] = a[2].wrapping_add(5);
            u_var3 = u_var3.wrapping_add(1);
            a[2] = a[2].wrapping_add(0x69);
            let u_var7 = DAT_00b6a2c0;
            let pc = FUN_004ddc10(param_3.wrapping_sub(1).wrapping_add(u_var3 as i32));
            FUN_004db7d0(0xc, pc, a.as_mut_ptr(), 0, 1, u_var7);
            a[0] = param_2.read().wrapping_add(0x82);
            if (u_var3 as i32) >= 2 {
                break;
            }
        }
    }
}
