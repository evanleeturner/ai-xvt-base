// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0054f9a0: u32;
    static mut DAT_0054f994: u32;
    static mut _DAT_0051a550: u32;
    static mut DAT_0066ddc4: u32;
    static mut DAT_005233e0: u32;
    static mut DAT_009a8e3c: u32;
    static mut _DAT_0054f9a4: f32;
    static mut DAT_0066dde0: u32;
    static mut DAT_005233e8: u32;
    static mut DAT_009a7b60: u32;
    static mut _DAT_0053f974: f32;
    static mut DAT_0051a54c: u32;
    static mut DAT_00999426: u32;
    static mut DAT_0054f990: u32;
    static mut DAT_0052f868: u32;
    static mut DAT_00528c5c: u32;
    static mut DAT_0053f970: u32;
    static mut DAT_00999420: u32;
    static mut DAT_0054f988: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040b180() {
    // SAFETY: the globals are the game's own memory; DAT_00528c5c holds a
    // pointer to a structure the original reads at offsets 0x40 and 0x44.
    unsafe {
        DAT_0054f9a0 = 0;
        DAT_0054f994 = 0;
        _DAT_0051a550 = 0;
        _DAT_0054f9a4 =
            ((DAT_0066ddc4.wrapping_sub(DAT_005233e0) >> 1).wrapping_add(DAT_009a8e3c)) as f32;
        _DAT_0053f974 =
            ((DAT_0066dde0.wrapping_sub(DAT_005233e8) >> 1).wrapping_add(DAT_009a7b60)) as f32;
        DAT_0051a54c = 1;
        DAT_0054f990 = (DAT_00999426.wrapping_mul(3) & 0x1fffffff) >> 4;
        DAT_0052f868 = (DAT_00999426.wrapping_mul(0x18) / 0x14) >> 2;
        let base = DAT_00528c5c as *const u8;
        let lim = base.byte_add(0x44).cast::<u32>().read_unaligned();
        if lim < DAT_0054f990 {
            DAT_0054f990 = lim;
        }
        if 0x100 < (DAT_0054f990 as i32) {
            DAT_0054f990 = 0x100;
        }
        if 0x100 < DAT_0052f868 {
            DAT_0052f868 = 0x100;
        }
        let v = base.byte_add(0x40).cast::<u32>().read_unaligned();
        let u_var1 = v.wrapping_add(DAT_0054f990.wrapping_mul(0xffffffc0)) / 0x18;
        if u_var1 < DAT_0052f868 {
            DAT_0052f868 = u_var1;
        }
        DAT_0053f970 = DAT_00999420;
        DAT_0054f988 = DAT_00999420.wrapping_add((DAT_00999426 / 2).wrapping_mul(0x18));
    }
}
