// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081f7de: i32;
    static mut DAT_0081fa9e: i32;
    static mut DAT_0071bcb8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431c90(param_1: *mut i32) -> *mut i32;
    fn FUN_00432c10(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00425280() -> i32 {
    let mut local_4: i32 = 0;
    let mut i_var4: i32 = 0;
    // SAFETY: the callees and globals are the game's own memory; the
    // pointers walk the game's tables exactly as the original does.
    unsafe {
        let p_var1 = FUN_00431c90(&raw mut local_4);
        if 0 < local_4 {
            let mut pi_var5 = p_var1.add(8);
            loop {
                let mut i_var3: i32 = 0;
                let mut pi_var2 = (&raw mut DAT_0081f7de) as *mut i32;
                let end = (&raw mut DAT_0081fa9e) as usize;
                loop {
                    let v = pi_var2.read_unaligned();
                    if v != 0 && pi_var5.read_unaligned() == v {
                        break;
                    }
                    pi_var2 = pi_var2.byte_add(0x16 * 4);
                    i_var3 = i_var3.wrapping_add(1);
                    if (pi_var2 as usize) >= end {
                        break;
                    }
                }
                if i_var3 < 8 {
                    let val = pi_var5.read_unaligned();
                    let i_var3b = FUN_00432c10(val);
                    (&raw mut DAT_0071bcb8)
                        .byte_offset(i_var3b.wrapping_mul(0x5bd) as isize)
                        .cast::<i32>()
                        .write_unaligned(pi_var5.read_unaligned());
                }
                pi_var5 = pi_var5.add(10);
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < local_4) {
                    break;
                }
            }
        }
    }
    1
}
