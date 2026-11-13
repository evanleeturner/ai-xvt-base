// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005272a4: i32;
    static mut DAT_009a7ed0: [i16; 256];
    static mut DAT_0060f1c0: i32;
    static mut DAT_0060f1e8: i32;
    static mut DAT_0060f208: i32;
    static mut DAT_0060f1f0: i32;
    static mut DAT_0060f1d8: i32;
    static mut DAT_0060f204: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac740();
    fn FUN_004ac720(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004736b0(
        param_1: i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut f32,
        param_5: *mut f32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00473580(param_1: i32, param_2: *mut f32, param_3: *mut f32) -> i32 {
    let mut ai_stack_d0: [i32; 0x34] = [0; 0x34];
    // SAFETY: globals and callees are the game's own; param_1 is an address of
    // the caller's object, and the object returned by FUN_004ac720 is read at
    // the offsets the original uses.
    unsafe {
        if DAT_005272a4 == 0 {
            return 0;
        }

        let u_var1: i16 = (&raw mut DAT_009a7ed0)
            .cast::<i16>()
            .add((param_1 as usize as *const u8).add(3).read() as usize)
            .read();
        FUN_004ac740();
        let pi_var2 = FUN_004ac720(u_var1 as u16 as i32) as *mut i32;

        if *pi_var2 != pi_var2 as usize as i32 {
            FUN_00471ff0(pi_var2);
        }

        ai_stack_d0 = [0; 0x34];

        ai_stack_d0[0] = param_1;
        ai_stack_d0[1] = 0x3f800000;
        ai_stack_d0[2] = 0;
        ai_stack_d0[3] = 0;
        ai_stack_d0[4] = 0;
        ai_stack_d0[5] = 0x3f800000;
        ai_stack_d0[6] = 0;
        ai_stack_d0[7] = 0;
        ai_stack_d0[8] = 0;
        ai_stack_d0[9] = 0x3f800000;
        ai_stack_d0[10] = 0x3f800000;
        ai_stack_d0[11] = 0;
        ai_stack_d0[12] = 0;
        ai_stack_d0[13] = 0;
        ai_stack_d0[14] = 0x3f800000;
        ai_stack_d0[15] = 0;
        ai_stack_d0[16] = 0;
        ai_stack_d0[17] = 0;
        ai_stack_d0[18] = 0x3f800000;

        DAT_0060f1c0 = 0;
        DAT_0060f1e8 = 0;
        DAT_0060f208 = 0;
        DAT_0060f1f0 = 0;
        DAT_0060f1d8 = 0;
        DAT_0060f204 = 0;

        let mut i_var4: i32 = 0;
        let mut count: i32 = pi_var2.cast::<u8>().add(6).cast::<i32>().read_unaligned();
        if 0 < count {
            let mut i_var5: i32 = 0;
            loop {
                let base: i32 = pi_var2.cast::<u8>().add(10).cast::<i32>().read_unaligned();
                let item = (base.wrapping_add(i_var5) as usize as *const i32).read_unaligned()
                    as usize as *mut i32;
                let i_var3 = FUN_004736b0(
                    pi_var2 as usize as i32,
                    item,
                    ai_stack_d0.as_mut_ptr(),
                    param_2,
                    param_3,
                );
                if i_var3 != 0 {
                    return 1;
                }
                i_var5 = i_var5.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                count = pi_var2.cast::<u8>().add(6).cast::<i32>().read_unaligned();
                if !(i_var4 < count) {
                    break;
                }
            }
        }
        0
    }
}
