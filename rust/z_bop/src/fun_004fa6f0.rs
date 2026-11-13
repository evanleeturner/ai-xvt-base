// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a5a6: i32;
    static mut DAT_00aa5e10: [i32; 1];
    static mut DAT_00aa5ac0: [i32; 1];
    static mut DAT_00aa5cb0: [i32; 1];
    static mut DAT_00aa5aac: i16;
    static mut DAT_00a91d24: i8;
    static mut DAT_00aa6af4: i32;
    static mut DAT_00aa6af8: i32;
    static mut _DAT_00aa6af0: i32;
    static mut DAT_00aa6afc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fa6f0() -> i32 {
    // SAFETY: all globals and the callee are the game's own memory; arrays
    // are indexed exactly as the original does.
    unsafe {
        let e10 = (&raw mut DAT_00aa5e10).cast::<i32>();
        let a_c0 = (&raw mut DAT_00aa5ac0).cast::<i32>();
        let a_b0 = (&raw mut DAT_00aa5cb0).cast::<i32>();

        let mut local_4: i32 = 0;
        let count: i32 = e10.offset(DAT_00b6a5a6 as isize).read();

        if count > 0 {
            loop {
                let i_var3: i32 = DAT_00b6a5a6.wrapping_mul(8);
                let i_var2: i32 = local_4.wrapping_add(i_var3);

                if a_c0.offset(i_var2 as isize).read() == -1
                    && a_b0.offset(i_var2 as isize).read() != 0
                {
                    let mut i_var5: i32 = 0;
                    if DAT_00aa5aac > 0 {
                        let mut pc_var6: *const i8 =
                            (&raw mut DAT_00a91d24).cast::<i8>() as *const i8;
                        loop {
                            if pc_var6.read() != 0
                                && (pc_var6.wrapping_sub(0xc).cast::<u8>().read() as i32)
                                    == DAT_00b6a5a6
                            {
                                let mut i_var4: i32 = 0;
                                let i_var1: i32 = e10.offset(DAT_00b6a5a6 as isize).read();

                                if i_var1 > 0 {
                                    loop {
                                        if a_c0.offset(i_var3.wrapping_add(i_var4) as isize).read()
                                            == i_var5
                                        {
                                            break;
                                        }
                                        i_var4 = i_var4.wrapping_add(1);
                                        if !(i_var4 < i_var1) {
                                            break;
                                        }
                                    }
                                }

                                a_c0.offset(i_var2 as isize).write(i_var5);

                                if i_var4 == i_var1 {
                                    DAT_00aa6af4 = DAT_00b6a5a6;
                                    DAT_00aa6af8 = local_4;
                                    _DAT_00aa6af0 = 0x6e;
                                    DAT_00aa6afc = i_var5;
                                    FUN_004cef70(0, &raw mut _DAT_00aa6af0 as *mut u32, 0x10);
                                    break;
                                }
                            }
                            pc_var6 = pc_var6.wrapping_add(0x562);
                            i_var5 = i_var5.wrapping_add(1);
                            if !(i_var5 < DAT_00aa5aac as i32) {
                                break;
                            }
                        }
                    }
                }
                local_4 = local_4.wrapping_add(1);
                if !(local_4 < count) {
                    break;
                }
            }
        }
        1
    }
}
