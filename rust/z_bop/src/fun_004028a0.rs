// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d113c: u16;
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08104: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00402cb0(param_1: u16);
    fn FUN_00402b60();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004028a0() {
    // SAFETY: all globals and callees are the game's own; the pointers read
    // are the game's records, accessed at the same offsets as the original.
    unsafe {
        let u_var2 = DAT_009d113c;
        let mut u_var4 = DAT_00a07cec;
        if (u_var4 as i32) < DAT_009a1fe0 {
            loop {
                let i_var3 = (u_var4 as i32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_009a1fe8);
                if *((i_var3.wrapping_add(3)) as *const i8) != 0 {
                    let local_ptr = ((i_var3.wrapping_add(0x1f)) as *const i32).read_unaligned();
                    if *(local_ptr as *const i8) != 0 {
                        let i_var1 =
                            ((local_ptr.wrapping_add(0xa9)) as *const i32).read_unaligned();
                        DAT_00a08104 = i_var1;
                        let b7 = *((i_var1.wrapping_add(7)) as *const i8);
                        if b7 != 3
                            && (b7 != 4
                                && ((i_var1.wrapping_add(0x3c)) as *const i32).read_unaligned()
                                    >= 1)
                        {
                            let local_val =
                                ((i_var3.wrapping_add(0x1b)) as *const i32).read_unaligned();
                            if local_val == -1 {
                                FUN_00402cb0(u_var4);
                                DAT_009d113c =
                                    ((i_var1.wrapping_add(0x40)) as *const u16).read_unaligned();
                                FUN_00402b60();
                                ((i_var1.wrapping_add(0x40)) as *mut u16)
                                    .write_unaligned(DAT_009d113c);
                            }
                            let a = ((i_var1.wrapping_add(0x3c)) as *const i32).read_unaligned();
                            let b = ((i_var1.wrapping_add(0x38)) as *const i32).read_unaligned();
                            ((i_var1.wrapping_add(0x3c)) as *mut i32)
                                .write_unaligned(a.wrapping_add(b));
                        }
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as i32) < DAT_009a1fe0) {
                    break;
                }
            }
        }
        DAT_009d113c = u_var2;
    }
}
