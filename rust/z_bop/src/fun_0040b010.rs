// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00999450: i32;
    static mut DAT_00999462: i32;
    static mut DAT_00999470: i32;
    static mut DAT_0099949e: i32;
    static mut DAT_009994a2: i32;
    static mut DAT_00999458: i32;
    static mut DAT_00999466: i32;
    static mut DAT_00999474: i32;
    static mut DAT_00999498: i32;
    static mut DAT_0054f990: i32;
    static mut DAT_0054f994: i32;
    static mut DAT_0052f868: i32;
    static mut DAT_0054f9a0: i32;
    static mut DAT_0053f970: *mut u32;
    static mut DAT_0054f988: *mut u32;
    static mut DAT_005270b4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00470140(param_1: i32);
    fn FUN_004b1d10(param_1: *mut u32, param_2: u32);
    fn FUN_004b1d70();
    fn FUN_004b1de0(param_1: *mut u32, param_2: u32);
    fn FUN_004b1c50();
    fn FUN_00408110();
    fn FUN_00408bc0(param_1: *mut i32);
    fn FUN_004084e0(param_1: *mut i32);
    fn FUN_00408e70(param_1: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00408140();
    fn FUN_004b1c10();
    fn FUN_004b1c90();
    fn FUN_004b2020();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040b010(param_1: *mut u32) {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a record of at least 0x34 dwords, copied into the game's record table.
    unsafe {
        let i_var1 = DAT_00999450;
        DAT_00999462 = 0;
        DAT_00999470 = 0;
        DAT_00999450 = i_var1;
        if DAT_0099949e != DAT_009994a2
            && (param_1.add(0x23).read_unaligned() as i32).wrapping_add(DAT_00999450)
                <= DAT_00999458
            && (param_1.add(0x1e).read_unaligned() as i32) <= DAT_00999466
            && (param_1.add(0x24).read_unaligned() as i32) <= DAT_00999474
        {
            let addr = DAT_009994a2.wrapping_mul(0xd0).wrapping_add(DAT_00999498);
            let mut dst = addr as *mut u32;
            let mut src = param_1 as *const u32;
            let mut n = 0x34;
            while n != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                n -= 1;
            }
            let pi_var3 = addr as *mut i32;
            FUN_00470140(addr);
            let i_var2 = pi_var3.add(0x31).read_unaligned();
            if i_var2 != 0 {
                if DAT_0054f990 < DAT_0054f994.wrapping_add(i_var2.wrapping_mul(8))
                    || DAT_0052f868 < i_var2.wrapping_mul(2).wrapping_add(DAT_0054f9a0)
                {
                    FUN_00408140();
                    FUN_004b1c10();
                    FUN_004b1c90();
                    FUN_004b1d10(DAT_0053f970, DAT_0054f994 as u32);
                    FUN_004b1d70();
                    FUN_004b1de0(DAT_0054f988, DAT_0054f9a0 as u32);
                    FUN_004b2020();
                    FUN_004b1c50();
                    FUN_00408110();
                    DAT_0054f9a0 = 0;
                    DAT_0054f994 = 0;
                }
                if DAT_005270b4 == 0 {
                    FUN_004084e0(pi_var3);
                } else {
                    FUN_00408bc0(pi_var3);
                }
                FUN_00408e70(pi_var3);
                DAT_00999450 = i_var1;
            }
        }
    }
}
