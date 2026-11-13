// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6e2: i8;
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
pub unsafe extern "C" fn FUN_004b93c0(param_1: i32) {
    let mut local_buf = core::mem::MaybeUninit::<[u8; 0x164]>::uninit();
    let buf = local_buf.as_mut_ptr().cast::<u8>();
    // SAFETY: the callees and the global are the game's own; buf is a local
    // buffer of 0x164 bytes, filled by the callees and read where the original reads it.
    unsafe {
        FUN_004dcfd0(buf.cast(), 0x164, 0x6f, 0x258, 0x7d);
        let pi_var2 = buf.cast::<i32>();
        let u_var5: u32 = 0xffff;
        let i_var4: i32 = 1;
        let i_var3: i32 = 0;
        let pc_var1 = FUN_004dcfa0(0xee);
        FUN_004daac0(0xc, pc_var1, pi_var2, i_var3, i_var4, u_var5);
        FUN_004dd030(pi_var2, 0, 0xf);
        let p = (&raw mut DAT_0085e6e2).wrapping_offset(param_1 as isize);
        let a = pi_var2.read_unaligned();
        let b = buf.byte_add(4).cast::<i8>().read();
        FUN_004b9eb0(p.cast(), pi_var2, 0x14, 0xef, 1, a, b);
    }
}
