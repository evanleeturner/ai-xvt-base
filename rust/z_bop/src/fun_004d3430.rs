// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fd7: u32;
    static mut DAT_00aa7fef: u32;
    static mut DAT_00aa7ff7: u32;
    static mut DAT_00aa7fe3: u32;
    static mut DAT_00aa7feb: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d3020(param_1: u32, param_2: u32, param_3: u8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d3430() -> u32 {
    // SAFETY: the globals and callees are the game's own; the globals are
    // read and written unaligned-safe by value where the original does.
    unsafe {
        let v_aa7fd7: u32 = (&raw const DAT_00aa7fd7).read_unaligned();
        let v_aa7fef: u32 = (&raw const DAT_00aa7fef).read_unaligned();
        let v_aa7feb: u32 = (&raw const DAT_00aa7feb).read_unaligned();

        FUN_004d3020(
            v_aa7fd7,
            v_aa7fef / 60000,
            ((v_aa7fef % 60000) / 1000) as u8,
        );
        let d_var1 = GetTickCount();
        (&raw mut DAT_00aa7ff7).write_unaligned(0);
        let v_aa7fe3 = v_aa7feb.wrapping_add(d_var1).wrapping_add(2000);
        (&raw mut DAT_00aa7fe3).write_unaligned(v_aa7fe3);
        1
    }
}
