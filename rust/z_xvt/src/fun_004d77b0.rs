// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758618: i32;
    static mut DAT_00757cee: i32;
    static mut DAT_00757e3e: i32;
    static mut DAT_0081b740: i32;
    static mut DAT_00531334: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004cde00();
    fn FUN_004d5180(param_1: *mut i32);
    fn FUN_004d51b0(param_1: *mut i32);
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d77b0() -> i32 {
    let mut local_20: [i32; 4] = [0; 4];
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; the pointers walk
    // the game's table at DAT_00757cee in 0x2a-byte records, as the original does.
    unsafe {
        FUN_004dcfd0(local_20.as_mut_ptr(), 0x58, 0xda, 0x1ae, 0xe9);
        let mut pi_var2: *mut i32 = local_20.as_mut_ptr();
        let pc_var1 = FUN_004dcfa0(0xbc);
        FUN_004daac0(0xc, pc_var1, pi_var2, 0, 1, 0xffff);
        if DAT_00758618 != -1 {
            let mut i_var3: i32 = 0;
            FUN_004cde00();
            pi_var2 = &raw mut DAT_00757cee;
            FUN_004dcfd0(local_20.as_mut_ptr(), 0x58, 0xee, 0x102, 0xfc);
            loop {
                if pi_var2.read_unaligned() != 0 {
                    i_var3 = i_var3.wrapping_add(1);
                    FUN_004d5180(local_10.as_mut_ptr());
                    FUN_004d51b0(local_20.as_mut_ptr());
                    let i_var4 = (pi_var2 as i32).wrapping_sub(0xe);
                    let u_var5: u32 = 4;
                    let idx = pi_var2.byte_add(4).read_unaligned();
                    let pc = FUN_004dcfa0((idx as u32).wrapping_add(0x8e));
                    _sprintf(
                        (&raw mut DAT_0081b740) as *mut i8,
                        (&raw const DAT_00531334) as *const i8,
                        6i32,
                        pc,
                        u_var5,
                        i_var4,
                    );
                    FUN_004daac0(
                        0xc,
                        (&raw mut DAT_0081b740) as *mut i8,
                        local_20.as_mut_ptr(),
                        0,
                        1,
                        0xffff,
                    );
                    FUN_004d51b0(local_10.as_mut_ptr());
                    if i_var3 == 4 {
                        FUN_004dcfd0(local_20.as_mut_ptr(), 0x103, 0xee, 0x1ae, 0xfc);
                    } else {
                        FUN_004dd030(local_20.as_mut_ptr(), 0, 0xf);
                    }
                }
                pi_var2 = pi_var2.byte_add(0x2a);
                if !((pi_var2 as u32) < ((&raw mut DAT_00757e3e) as u32)) {
                    break;
                }
            }
        }
    }
    1
}
