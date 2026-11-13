// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758618: i32;
    static mut DAT_00758620: i32;
    static mut DAT_00758624: i32;
    static mut DAT_00758628: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cde00() -> u32;
    fn FUN_004cc7b0(param_1: *mut i32) -> *mut u32;
    fn FUN_004ced20(param_1: i32) -> u32;
    fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e5600(param_1: i32) {
    let mut local_4: i32 = 0;
    // SAFETY: the globals and callees are the game's own; the pointer returned
    // by FUN_004cc7b0 addresses local_4 entries of 40 bytes, read where the
    // original reads them, and the table writes go to the original's addresses.
    unsafe {
        DAT_00758620 = 0x74;
        DAT_00758624 = 8;
        let mut i_var5: i32 = 3;
        DAT_00758628 = FUN_004cde00() as i32;
        let mut i_var4: i32 = 0;
        let pu_var1 = FUN_004cc7b0(&mut local_4);
        if 0 < local_4 {
            let mut pi_var3 = pu_var1.wrapping_add(8) as *mut i32;
            loop {
                if pi_var3.wrapping_add(1).read_unaligned() == 1 {
                    let base = &raw mut DAT_00758618;
                    base.wrapping_offset(i_var5.wrapping_add(2) as isize)
                        .write_unaligned(pi_var3.read_unaligned());
                    i_var5 = i_var5.wrapping_add(3);
                    let b = (pi_var3.wrapping_sub(8) as *const u8).read() as i32;
                    base.wrapping_offset(i_var5 as isize)
                        .write_unaligned(b.wrapping_sub(1));
                    let u_var2 = FUN_004ced20(pi_var3.read_unaligned());
                    let addr = (i_var5.wrapping_mul(4)).wrapping_add(0x75861c) as u32;
                    (addr as usize as *mut u32).write_unaligned(u_var2);
                }
                pi_var3 = pi_var3.wrapping_add(10);
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < local_4) {
                    break;
                }
            }
        }
        FUN_004cb830(
            param_1,
            &raw mut DAT_00758620 as *mut u32,
            (i_var5 << 2) as u32,
        );
    }
}
