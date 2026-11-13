// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e28: i32;
    static mut DAT_00b69cc0: i32;
    static mut DAT_00b6a2a0: i32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b6a2e0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(p1: *mut i32, p2: *mut i32);
    fn FUN_004de090(p1: *mut i32, p2: *mut i32);
    fn FUN_004d6b90(p1: *mut i32, p2: i32, p3: i32) -> i32;
    fn FUN_004d6550(p1: *mut i32, p2: i32, p3: i32, p4: i32);
    fn FUN_004d4cf0(p1: *mut i32);
    fn FUN_004d4d20(p1: *mut i32);
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn FUN_004db7d0(p1: i32, p2: *mut i8, p3: *mut i32, p4: i32, p5: i32, p6: u32);
    fn FUN_004d64a0(p1: *mut i32, p2: i32, p3: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004dc220() -> i32;
    fn FUN_004dc240() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bef80(param_1: *mut i32, param_2: i32) -> i32 {
    // SAFETY: the globals, callees and pointers are the game's own memory;
    // locals are laid out so the callees may fill the adjacent struct slots.
    unsafe {
        if DAT_00b69e28 == 0 {
            return 0;
        }
        // local_2c, local_28, (pad), local_20
        let mut local_2c: [i32; 4] = [0, 0, 0, 0];
        let mut local_18: i32 = 0;
        let mut local_14: i32 = 0;
        let mut local_10: [i32; 4] = [0; 4];

        FUN_004d6480(local_2c.as_mut_ptr(), param_1);
        FUN_004de090(&raw mut local_14, &raw mut local_18);
        let mut i_var4: i32 = 0;
        let local_1c: i32 = ((DAT_00b69e28 as usize + 4) as *const i32).read_unaligned();
        if DAT_00b69cc0 != 0 && param_2 < param_2.wrapping_add(0xd) {
            let mut i_var5: i32 = param_2.wrapping_mul(0xe);
            let mut i_var6: i32 = param_2;
            loop {
                if local_1c <= i_var6 {
                    return i_var4;
                }
                let i_var2 = FUN_004d6b90(local_2c.as_mut_ptr(), local_14, local_18);
                if i_var2 != 0 {
                    FUN_004d6550(local_2c.as_mut_ptr(), 0, 0, DAT_00b6a2a0);
                    let mut u_var1 = FUN_004dc220();
                    if u_var1 != 0 || {
                        u_var1 = FUN_004dc240();
                        u_var1 != 0
                    } {
                        i_var4 = i_var6.wrapping_add(1);
                    }
                }
                FUN_004d4cf0(local_10.as_mut_ptr());
                FUN_004d4d20(local_2c.as_mut_ptr());
                let i_var2 = __strcmpi(
                    DAT_00b69cc0.wrapping_add(i_var5) as usize as *const i8,
                    (&raw mut DAT_00b6a2e0) as *const i8,
                );
                let mut u_var7 = DAT_00b6a2c0;
                if i_var2 != 0 {
                    u_var7 = 0xffff;
                }
                i_var6 = i_var6.wrapping_add(1);
                let pc_var3 = DAT_00b69cc0.wrapping_add(i_var5) as usize as *mut i8;
                i_var5 = i_var5.wrapping_add(0xe);
                FUN_004db7d0(0xc, pc_var3, local_2c.as_mut_ptr(), 0, 1, u_var7);
                FUN_004d4d20(local_10.as_mut_ptr());
                FUN_004d64a0(local_2c.as_mut_ptr(), 0, 0xf);
                if !(i_var6 < param_2.wrapping_add(0xd)) {
                    break;
                }
            }
        }
        i_var4
    }
}
