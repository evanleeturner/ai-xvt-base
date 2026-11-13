// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6e4: i8;
    static mut DAT_0085e6e6: i8;
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
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004b9b40(param_1: *mut i8, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b9010(param_1: i32) {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; param_1 indexes
    // the game's byte table starting at DAT_0085e6e4, as in the original.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x58, 0x6f, 0x14c, 0x7d);
        let pi_var4 = local_10.as_mut_ptr();
        let pc_var3 = FUN_004dcfa0(0xcf);
        FUN_004daac0(0xc, pc_var3, pi_var4, 0, 1, 0xffff);
        FUN_004dd030(local_10.as_mut_ptr(), 0, 0xf);
        let base = (&raw mut DAT_0085e6e4).cast::<i8>();
        let c_var1 = base.offset(param_1 as isize).read();
        FUN_004b9b40(base.offset(param_1 as isize), local_10.as_mut_ptr(), 0xd0);
        let c_var2 = base.offset(param_1 as isize).read();
        if c_var2 != c_var1 {
            (&raw mut DAT_0085e6e6)
                .cast::<i8>()
                .offset(param_1 as isize)
                .write(c_var2);
        }
    }
}
