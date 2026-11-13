// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6ea: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    );
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
    fn FUN_004b9eb0(
        param_1: *mut core::ffi::c_void,
        param_2: *mut i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i8,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b91c0(param_1: i32) {
    let mut local_buf: [i32; 0x16] = [0; 0x16];
    let buf = local_buf.as_mut_ptr();
    // SAFETY: the callees and the global are the game's own; buf points at a
    // local buffer that outlives every call.
    unsafe {
        FUN_004dcfd0(buf.cast::<core::ffi::c_void>(), 0x58, 0xf3, 0x14c, 0x101);
        let u_var5: u32 = 0xffff;
        let i_var4: i32 = 1;
        let i_var3: i32 = 0;
        let pc_var1 = FUN_004dcfa0(0xf2);
        FUN_004daac0(0xc, pc_var1, buf, i_var3, i_var4, u_var5);
        FUN_004dd030(buf, 0, 0xf);
        let dst = (&raw mut DAT_0085e6ea).byte_offset(param_1 as isize);
        let first = buf.read();
        let byte4 = buf.cast::<i8>().add(4).read();
        FUN_004b9eb0(
            dst.cast::<core::ffi::c_void>(),
            buf,
            8,
            0xf3,
            1,
            first,
            byte4,
        );
    }
}
