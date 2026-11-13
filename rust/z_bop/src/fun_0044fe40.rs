// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009fd39c: i32;
    static mut DAT_009fd3a0: i32;
    static mut DAT_009fd398: i32;
    static mut DAT_009a8e2c: i32;
    static mut DAT_009a8e30: i32;
    static mut DAT_009a8e34: i32;
    static mut DAT_009e9a5b: i32;
    static mut DAT_009e9a5f: i32;
    static mut DAT_009e9a63: i32;
    static mut DAT_009ecc34: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427420(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_00427490(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_00427500(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044fe40(param_1: u32) {
    // SAFETY: the globals and callees are the game's own memory; the table
    // pointers are read at the same byte offsets the original uses.
    unsafe {
        let i_var1: i32 = ((param_1 & 0xffff) as i32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);
        let off = DAT_009ecc34.wrapping_mul(0x5bd) as isize;

        let src1: i32 = (&raw mut DAT_009e9a5b)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        let src2: i32 = (&raw mut DAT_009e9a5f)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        let src3: i32 = (&raw mut DAT_009e9a63)
            .cast::<u8>()
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();

        DAT_009fd39c = ((i_var1.wrapping_add(4)) as *const i32)
            .read_unaligned()
            .wrapping_sub(src1);
        DAT_009fd3a0 = ((i_var1.wrapping_add(8)) as *const i32)
            .read_unaligned()
            .wrapping_sub(src2);
        DAT_009fd398 = ((i_var1.wrapping_add(0xc)) as *const i32)
            .read_unaligned()
            .wrapping_sub(src3);

        DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
        DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
    }
}
