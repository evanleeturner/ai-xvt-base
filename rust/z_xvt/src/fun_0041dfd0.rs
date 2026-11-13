// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0073b374: u16;
    static mut DAT_006d4a60: [u8; 0];
    static mut DAT_006d4a61: [u8; 0];
    static mut DAT_0070ad42: [u8; 0];
    static mut DAT_005247e8: [u8; 0];
    static mut DAT_0052687a: [u8; 0];
    static mut DAT_006d4a02: u8;
    static mut DAT_0070ad51: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041eea0(param_1: u16) -> u64;
    fn FUN_004215c0(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041dfd0(param_1: u16) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // indices follow the original's arithmetic on the game's tables.
    unsafe {
        let u_var3: u32 = DAT_0073b374 as u32;
        let i_var1: u32 = u_var3.wrapping_mul(0x126);
        (&raw mut DAT_006d4a60 as *mut u8)
            .add(i_var1 as usize)
            .write(1);
        let i_var2: u32 = u_var3.wrapping_mul(0x566);
        let idx: u8 = (&raw mut DAT_0070ad42 as *mut u8)
            .add(i_var2 as usize)
            .read();
        let idx2: u8 = (&raw mut DAT_005247e8 as *mut u8).add(idx as usize).read();
        let val: u8 = (&raw mut DAT_0052687a as *mut u8)
            .add((idx2 as usize) * 0x16)
            .read();
        if (val & 0x80) == 0 {
            let v = (&raw mut DAT_0070ad51 as *mut u8)
                .add(i_var2 as usize)
                .read();
            (&raw mut DAT_006d4a61 as *mut u8)
                .add(i_var1 as usize)
                .write(v);
            let u_var4: u64 = FUN_0041eea0(param_1);
            return (((u_var4 >> 16) as u32).wrapping_shl(16)) | 1;
        }
        if DAT_006d4a02 == 2 {
            let v = (&raw mut DAT_0070ad51 as *mut u8)
                .add(i_var2 as usize)
                .read();
            (&raw mut DAT_006d4a61 as *mut u8)
                .add(i_var1 as usize)
                .write(v);
        } else {
            (&raw mut DAT_006d4a61 as *mut u8)
                .add(i_var1 as usize)
                .write(0);
        }
        FUN_004215c0(param_1);
        1
    }
}
