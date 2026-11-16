// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bfb6: i32;
    static mut DAT_007579d0: [i32; 1];
    static mut DAT_00757680: [i32; 1];
    static mut DAT_00757870: [i32; 1];
    static mut DAT_0075766c: i16;
    static mut DAT_007438e4: [i8; 1];
    static mut DAT_00758624: i32;
    static mut DAT_00758628: i32;
    static mut _DAT_00758620: i32;
    static mut DAT_0075862c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f5b20() -> i32 {
    // SAFETY: all globals and the callee are the game's own memory; indices
    // follow the original's arithmetic exactly.
    unsafe {
        let a679 = (&raw mut DAT_007579d0).cast::<i32>();
        let a680 = (&raw mut DAT_00757680).cast::<i32>();
        let a870 = (&raw mut DAT_00757870).cast::<i32>();
        let names = (&raw mut DAT_007438e4).cast::<i8>();

        let mut local_8: i32 = 0;
        let count: i32 = a679.offset(DAT_0081bfb6 as isize).read();

        if 0 < count {
            loop {
                let i_var3: i32 = DAT_0081bfb6.wrapping_mul(8);
                let i_var2: i32 = local_8.wrapping_add(i_var3);
                let mut i_var5: i32 = 0;

                if a680.offset(i_var2 as isize).read() == -1
                    && a870.offset(i_var2 as isize).read() != 0
                    && 0 < DAT_0075766c
                {
                    let mut pc_var6: *mut i8 = names;
                    loop {
                        if pc_var6.read() != 0
                            && (pc_var6.offset(-0xc).read() as u8 as i32) == DAT_0081bfb6
                        {
                            let mut i_var4: i32 = 0;
                            let i_var1: i32 = a679.offset(DAT_0081bfb6 as isize).read();

                            if 0 < i_var1 {
                                loop {
                                    if a680.offset(i_var3.wrapping_add(i_var4) as isize).read()
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

                            a680.offset(i_var2 as isize).write(i_var5);

                            if i_var4 == i_var1 {
                                DAT_00758624 = DAT_0081bfb6;
                                DAT_00758628 = local_8;
                                _DAT_00758620 = 0x6e;
                                DAT_0075862c = i_var5;
                                FUN_004cb830(0, (&raw mut _DAT_00758620).cast::<u32>(), 0x10);
                                break;
                            }
                        }
                        pc_var6 = pc_var6.offset(0x562);
                        i_var5 = i_var5.wrapping_add(1);
                        if !(i_var5 < DAT_0075766c as i32) {
                            break;
                        }
                    }
                }
                local_8 = local_8.wrapping_add(1);
                if !(local_8 < count) {
                    break;
                }
            }
        }
        1
    }
}
