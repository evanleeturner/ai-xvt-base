// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f10e0: u32;
    static mut DAT_005f10d4: u32;
    static mut DAT_005279f4: u32;
    static mut _DAT_005f10e4: f32;
    static mut DAT_006572b8: u32;
    static mut DAT_00525520: u32;
    static mut _DAT_006db7bc: u32;
    static mut _DAT_005f10b4: f32;
    static mut DAT_006572d4: u32;
    static mut DAT_00525528: u32;
    static mut _DAT_006da4fc: u32;
    static mut DAT_005f10d0: u32;
    static mut DAT_006a8826: u32;
    static mut DAT_005f0fa8: u32;
    static mut DAT_0052f1c4: u32;
    static mut DAT_005f10b0: u32;
    static mut DAT_006a8820: u32;
    static mut DAT_005f10c8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045f260() {
    // SAFETY: the globals are the game's own memory; DAT_0052f1c4 holds the
    // address of a structure the game keeps valid, read at the original offsets.
    unsafe {
        DAT_005f10e0 = 0;
        DAT_005f10d4 = 0;
        DAT_005279f4 = 1;
        _DAT_005f10e4 =
            ((DAT_006572b8.wrapping_sub(DAT_00525520) >> 1).wrapping_add(_DAT_006db7bc)) as f32;
        _DAT_005f10b4 =
            ((DAT_006572d4.wrapping_sub(DAT_00525528) >> 1).wrapping_add(_DAT_006da4fc)) as f32;
        DAT_005f10d0 = (DAT_006a8826.wrapping_mul(3) & 0x1fff_ffff) >> 4;
        DAT_005f0fa8 = (DAT_006a8826.wrapping_mul(0x18) / 0x14) >> 2;
        let p = DAT_0052f1c4 as *const u8;
        let lim = p.byte_add(0x18).cast::<u32>().read_unaligned();
        if lim < DAT_005f10d0 {
            DAT_005f10d0 = lim;
        }
        if 0x100 < (DAT_005f10d0 as i32) {
            DAT_005f10d0 = 0x100;
        }
        if 0x100 < DAT_005f0fa8 {
            DAT_005f0fa8 = 0x100;
        }
        let base = p.byte_add(0x14).cast::<i32>().read_unaligned() as u32;
        let u_var1 = base.wrapping_add(DAT_005f10d0.wrapping_mul(0xffff_ffc0)) / 0x18;
        if u_var1 < DAT_005f0fa8 {
            DAT_005f0fa8 = u_var1;
        }
        DAT_005f10b0 = DAT_006a8820;
        DAT_005f10c8 = DAT_006a8820.wrapping_add((DAT_006a8826 / 2).wrapping_mul(0x18));
    }
}
