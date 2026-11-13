// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703abc: u16;
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732e54: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_004296b0(param_1: u16);
    fn FUN_00429570();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00429230() {
    // SAFETY: all globals, callees and the addresses derived from them are
    // the game's own memory, read and written exactly where the original does.
    unsafe {
        let u_var2: u16 = DAT_00703abc;
        let mut u_var4: u16 = DAT_00732a3c;
        if (u_var4 as i32) < DAT_006d4980 {
            loop {
                let mut i_var3: i32 =
                    DAT_006d4988.wrapping_add((u_var4 as u32).wrapping_mul(0x23) as i32);
                if (i_var3.wrapping_add(3) as *const u8).read() != 0 {
                    let pc_var1: *const i8 =
                        (i_var3.wrapping_add(0x1f) as *const u32).read_unaligned() as *const i8;
                    if pc_var1.read() == 0 {
                        DAT_00732e54 = pc_var1.byte_add(0xa7).cast::<i32>().read_unaligned();
                        i_var3 = FUN_0042b150(DAT_00732e54);
                        let b = (DAT_00732e54.wrapping_add(7) as *const u8).read();
                        if b != 3 && b != 4 {
                            let mut tmp_int =
                                (i_var3.wrapping_add(0x12) as *const i32).read_unaligned();
                            if tmp_int < 1 {
                                tmp_int = (DAT_006d4988
                                    .wrapping_add(0x1b)
                                    .wrapping_add((u_var4 as u32).wrapping_mul(0x23) as i32)
                                    as *const i32)
                                    .read_unaligned();
                                if tmp_int == -1 {
                                    FUN_004296b0(u_var4);
                                    DAT_00703abc =
                                        (i_var3.wrapping_add(0x16) as *const u16).read_unaligned();
                                    FUN_00429570();
                                    (i_var3.wrapping_add(0x16) as *mut u16)
                                        .write_unaligned(DAT_00703abc);
                                }
                                tmp_int =
                                    (i_var3.wrapping_add(0x12) as *const i32).read_unaligned();
                                let tmp_int2 =
                                    (i_var3.wrapping_add(0xe) as *const i32).read_unaligned();
                                tmp_int = tmp_int.wrapping_add(tmp_int2);
                                (i_var3.wrapping_add(0x12) as *mut i32).write_unaligned(tmp_int);
                            }
                        }
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        DAT_00703abc = u_var2;
    }
}
