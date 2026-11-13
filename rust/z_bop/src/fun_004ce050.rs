// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005182a0: u32;
    static mut DAT_005182a4: u32;
    static mut DAT_005182a8: u32;
    static mut DAT_005182ac: u32;
    static mut DAT_005182b0: u32;
    static mut DAT_005182b4: u32;
    static mut DAT_005182b8: u32;
    static mut DAT_005182bc: u32;
    static mut DAT_005182c0: u32;
    static mut DAT_005182c4: u32;
    static mut DAT_005182c8: u32;
    static mut DAT_005182cc: u32;
    static mut DAT_005182d0: u32;
    static mut DAT_005182d4: u32;
    static mut DAT_005182d8: u32;
    static mut DAT_005182dc: u32;
    static mut DAT_00665040: u32;
    static mut DAT_00665044: u32;
    static mut DAT_00665048: u32;
    static mut DAT_0066504c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ce050(param_1: u32) -> *mut u8 {
    // SAFETY: the globals are the game's own memory; each case copies four
    // words into the output block and returns its address, as the original does.
    unsafe {
        match param_1 {
            0 => {
                DAT_00665040 = DAT_005182a0;
                DAT_00665044 = DAT_005182a4;
                DAT_00665048 = DAT_005182a8;
                DAT_0066504c = DAT_005182ac;
                (&raw mut DAT_00665040).cast::<u8>()
            }
            1 => {
                DAT_00665040 = DAT_005182c0;
                DAT_00665044 = DAT_005182c4;
                DAT_00665048 = DAT_005182c8;
                DAT_0066504c = DAT_005182cc;
                (&raw mut DAT_00665040).cast::<u8>()
            }
            2 => {
                DAT_00665040 = DAT_005182b0;
                DAT_00665044 = DAT_005182b4;
                DAT_00665048 = DAT_005182b8;
                DAT_0066504c = DAT_005182bc;
                (&raw mut DAT_00665040).cast::<u8>()
            }
            3 => {
                DAT_00665040 = DAT_005182d0;
                DAT_00665044 = DAT_005182d4;
                DAT_00665048 = DAT_005182d8;
                DAT_0066504c = DAT_005182dc;
                (&raw mut DAT_00665040).cast::<u8>()
            }
            _ => core::ptr::null_mut(),
        }
    }
}
