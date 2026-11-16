// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009e9a5b: i8;
    static mut DAT_009e9a5f: i8;
    static mut DAT_009e9a63: i8;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(param_1: u32, param_2: i32);
    fn FUN_004ae640(param_1: u32, param_2: i32) -> i32;
    fn FUN_004ae6e0(param_1: u32, param_2: i32) -> i32;
    fn FUN_004ae5a0(param_1: u32, param_2: i32) -> i32;
    fn FUN_004037b0(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_00427500(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_00427420(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_00427490(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_00427570(param_1: u32, param_2: u32) -> u32;
    fn FUN_00427590(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004830d0(
    param_1: u32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
) {
    // SAFETY: the callees and globals are the game's own memory; the
    // pointers are the caller's and are read/written where the original does.
    unsafe {
        FUN_00458e20(param_1, 0);
        if param_2 != 0xffff {
            let i_var8 = (param_1 as i32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8);
            let u_var4 = u32::from((i_var8.wrapping_add(3) as *const u8).read());
            let i_var1 = FUN_004ae640(u_var4, param_2);
            let i_var1 = i_var1.wrapping_neg();
            let i_var2 = FUN_004ae6e0(u_var4, param_2);
            let i_var3 = FUN_004ae5a0(u_var4, param_2);
            FUN_004037b0(i_var8, i_var3, i_var2, i_var1);
            DAT_00a07cdc = DAT_00a07cdc.wrapping_add(DAT_009d8a58);
            DAT_00a07cd8 = DAT_00a07cd8.wrapping_add(DAT_009d8a54);
            DAT_00a07cd4 = DAT_00a07cd4.wrapping_add(DAT_009d8b60);
        }
        let off = DAT_009ecc34.wrapping_mul(0x5bd) as isize;
        let local_a = (&raw mut DAT_009e9a5b)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        let local_b = (&raw mut DAT_009e9a5f)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        let local_c = (&raw mut DAT_009e9a63)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        let i_var3 = DAT_00a07cdc.wrapping_sub(local_a);
        let i_var1 = DAT_00a07cd8.wrapping_sub(local_b);
        let i_var2 = DAT_00a07cd4.wrapping_sub(local_c);
        let u_var4 = FUN_00427500(i_var3, i_var1, i_var2);
        param_5.write_unaligned(u_var4);
        if 0 < (u_var4 as i32) {
            let u_var5 = FUN_00427420(i_var3, i_var1, i_var2);
            let u_var6 = FUN_00427490(i_var3, i_var1, i_var2);
            let u_var7 = FUN_00427570(u_var5, u_var4);
            param_3.write_unaligned(u_var7);
            let u_var7 = FUN_00427590(u_var6, u_var4);
            param_4.write_unaligned(u_var7);
        }
    }
}
