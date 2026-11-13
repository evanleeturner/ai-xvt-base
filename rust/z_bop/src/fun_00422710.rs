// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009fe7d4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004229f0();
    fn FUN_00422d90();
    fn FUN_004230f0();
    fn FUN_00423480();
    fn FUN_00423810();
    fn FUN_00423b90();
    fn FUN_00423f30();
    fn FUN_004242e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00422710() {
    // SAFETY: the global and the callees are the game's own; the global holds
    // an address of a structure whose u16 at offset 0x16 selects the callee.
    unsafe {
        let u_var1 = ((DAT_009fe7d4.wrapping_add(0x16)) as *const u16).read_unaligned();
        if u_var1 < 8 {
            match u_var1 {
                0 => FUN_004229f0(),
                1 => FUN_00422d90(),
                2 => FUN_004230f0(),
                3 => FUN_00423480(),
                4 => FUN_00423810(),
                5 => FUN_00423b90(),
                6 => FUN_00423f30(),
                7 => FUN_004242e0(),
                _ => {}
            }
        }
    }
}
