// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536e98: u32;
    static mut DAT_00536e9c: u32;
    static mut DAT_00536ea8: u32;
    static mut DAT_00536ea0: u32;
    static mut DAT_00536ea4: u32;
    static mut DAT_00536eb4: u32;
    static mut DAT_00536eac: u32;
    static mut DAT_00536eb0: u32;
    static mut DAT_00536eb8: u32;
    static mut DAT_00536ec4: u16;
    static mut DAT_00536ec0: u16;
    static mut DAT_00536ebc: u16;
    static mut DAT_00536e94: u16;
    static mut DAT_00536e90: u16;

    static mut DAT_0073255c: u32;
    static mut DAT_00732558: u32;
    static mut DAT_00732568: u32;
    static mut DAT_00732584: u32;
    static mut DAT_00732578: u32;
    static mut DAT_00732530: u32;
    static mut DAT_0073253c: u32;
    static mut DAT_00732538: u32;
    static mut DAT_00732548: u32;
    static mut DAT_007091ae: u16;
    static mut DAT_006ffbf2: u16;
    static mut DAT_00732e44: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_00703be4: u16;
    static mut DAT_006d4a42: u16;
    static mut _DAT_0071f2e4: u32;
    static mut DAT_00527e74: u16;
    static mut _DAT_006da4fc: u32;
    static mut _DAT_006db7bc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00426970() {
    // SAFETY: all globals are the game's own memory, read and written
    // by value exactly as the original does.
    unsafe {
        DAT_0073255c = DAT_00536e98;
        DAT_00732558 = DAT_00536e9c;
        DAT_00732568 = DAT_00536ea8;
        DAT_00732584 = DAT_00536ea0;
        DAT_00732578 = DAT_00536ea4;
        DAT_00732530 = DAT_00536eb4;
        DAT_0073253c = DAT_00536eac;
        DAT_00732538 = DAT_00536eb0;
        DAT_00732548 = DAT_00536eb8;
        DAT_007091ae = DAT_00536ec4;
        DAT_006ffbf2 = DAT_00536ec4.wrapping_sub(1);
        DAT_00732e44 = DAT_00536ec4 >> 1;
        DAT_006d4984 = DAT_00536ec0;
        DAT_00703be4 = DAT_00536ec0.wrapping_sub(1);
        DAT_006d4a42 = DAT_00536ec0 >> 1;
        _DAT_0071f2e4 = DAT_00536ebc as u32;
        DAT_00527e74 = 0xc000;
        _DAT_006da4fc = DAT_00536e94 as u32;
        _DAT_006db7bc = DAT_00536e90 as u32;
    }
}
