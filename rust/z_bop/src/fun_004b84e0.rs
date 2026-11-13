// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72c4: i8;
    static mut DAT_00bb72c6: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004b9090(param_1: *mut i8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b84e0(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_10 is a live
    // local buffer, and the pointers computed from the globals are used as the
    // original does.
    unsafe {
        FUN_004d6460(local_10.as_mut_ptr(), 0x58, 0x6f, 0x14c, 0x7d);
        let piv = local_10.as_mut_ptr();
        let pc = FUN_004ddc10(0xdc);
        FUN_004db7d0(0xc, pc, piv, 0, 1, 0xffff);
        FUN_004d64a0(local_10.as_mut_ptr(), 0, 0xf);
        let pc2 = (&raw mut DAT_00bb72c4).byte_add(param_1 as usize);
        let c_var1 = pc2.read();
        FUN_004b9090(pc2, local_10.as_mut_ptr(), 0xdd);
        if pc2.read() != c_var1 {
            (&raw mut DAT_00bb72c6)
                .byte_add(param_1 as usize)
                .write(pc2.read());
        }
    }
}
