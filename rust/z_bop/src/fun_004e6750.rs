// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6af0: i32;
    static mut DAT_00aa6af4: i32;
    static mut DAT_00aa6af8: i32;
    static mut DAT_00aa6ae8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d1400() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfe80(param_1: *mut i32) -> *mut i32;
    fn FUN_004d2250(param_1: i32) -> u32;
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e6750(param_1: i32) {
    let mut local_4: i32 = 0;
    // SAFETY: the globals and callees are the game's own; the record pointer
    // returned by FUN_004cfe80 addresses local_4 records of 40 bytes, read
    // where the original does, and the table writes land in the game's buffer.
    unsafe {
        DAT_00aa6af0 = 0x74;
        DAT_00aa6af4 = 8;
        let mut i_var3: i32 = 3;
        DAT_00aa6af8 = FUN_004d1400();
        let mut i_var5: i32 = 0;
        let p_var1 = FUN_004cfe80(&raw mut local_4);
        if 0 < local_4 {
            let mut pi_var4 = p_var1.add(8);
            loop {
                if pi_var4.add(1).read_unaligned() == 1 {
                    let base = &raw mut DAT_00aa6ae8;
                    base.offset(i_var3.wrapping_add(2) as isize)
                        .write_unaligned(pi_var4.read_unaligned());
                    i_var3 = i_var3.wrapping_add(3);
                    let b = pi_var4.sub(8).cast::<u8>().read();
                    base.offset(i_var3 as isize)
                        .write_unaligned((b as i32).wrapping_sub(1));
                    let u_var2 = FUN_004d2250(pi_var4.read_unaligned());
                    let addr = i_var3.wrapping_mul(4).wrapping_add(0xaa6aec) as u32;
                    (addr as usize as *mut u32).write_unaligned(u_var2);
                }
                pi_var4 = pi_var4.add(10);
                i_var5 = i_var5.wrapping_add(1);
                if !(i_var5 < local_4) {
                    break;
                }
            }
        }
        FUN_004cef70(
            param_1,
            &raw mut DAT_00aa6af0 as *mut u32,
            (i_var3 as u32) << 2,
        );
    }
}
