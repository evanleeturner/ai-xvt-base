// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_009e8f50: u16;
    static mut DAT_009ec476: u8;
    static mut DAT_005233fc: u16;
    static mut DAT_00a080fc: u8;
    static mut DAT_009a73f4: u8;
    static mut DAT_00527348: [u16; 1];
    static mut DAT_00527358: [u16; 1];
    static mut DAT_00527360: [u8; 1];
    static mut DAT_00527368: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00484160(param_1: u32) {
    let i_var1: u32 = (param_1 & 0xffff).wrapping_mul(2);
    // SAFETY: the globals and tables are the game's own memory; the table
    // indices are read where the original reads them.
    unsafe {
        _DAT_009e8f50 = (&raw mut DAT_00527348)
            .cast::<u16>()
            .add((i_var1 / 2) as usize)
            .read();
        DAT_009ec476 = 1;
        DAT_005233fc = (&raw mut DAT_00527358)
            .cast::<u16>()
            .add((i_var1 / 2) as usize)
            .read();
        DAT_00a080fc = (&raw mut DAT_00527360)
            .cast::<u8>()
            .add(i_var1 as usize)
            .read();
        DAT_009a73f4 = (&raw mut DAT_00527368)
            .cast::<u8>()
            .add(i_var1 as usize)
            .read();
    }
}
