// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b730: i32;
    static mut DAT_0071bb1b: i32;
    static mut DAT_0071bb1f: i32;
    static mut DAT_0071bb23: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421c70(param_1: u32, param_2: u32);
    fn FUN_00418cd0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0042a140(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00444160(param_1: i32, param_2: u32) {
    let i_var2 = param_1.wrapping_mul(0x5bd);
    // SAFETY: the globals and callees are the game's own; the offsets index
    // the game's record table at the byte positions the original reads.
    unsafe {
        let u_var1 = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .wrapping_offset(i_var2 as isize)
            .cast::<u32>()
            .read_unaligned();
        if u_var1 == 0xffff_ffff {
            FUN_00421c70(param_2, 0);
            let a = (&raw mut DAT_0071bb1b)
                .cast::<u8>()
                .wrapping_offset(i_var2 as isize)
                .cast::<i32>()
                .read_unaligned();
            let b = (&raw mut DAT_0071bb1f)
                .cast::<u8>()
                .wrapping_offset(i_var2 as isize)
                .cast::<i32>()
                .read_unaligned();
            let c = (&raw mut DAT_0071bb23)
                .cast::<u8>()
                .wrapping_offset(i_var2 as isize)
                .cast::<i32>()
                .read_unaligned();
            FUN_00418cd0(
                DAT_00732a2c.wrapping_sub(a),
                DAT_00732a28.wrapping_sub(b),
                DAT_00732a24.wrapping_sub(c),
            );
            return;
        }
        FUN_0042a140(u_var1, param_2);
    }
}
