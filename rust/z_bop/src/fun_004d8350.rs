// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6ae8: i32;
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6138: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_00aa6118: *mut i8;
    static mut DAT_0052c208: i32;
    static mut DAT_00bb2810: i32;
    static mut s__s__c_s_0052bfcc: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: u32) -> *mut i8;
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
    fn FUN_004dbbd0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: u32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_004d9d70(
        param_1: *mut i32,
        param_2: i32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d8350() -> i32 {
    let mut local_10: [i32; 4] = [0; 4];
    let piv = local_10.as_mut_ptr();
    // SAFETY: all globals and callees are the game's own; the buffers are
    // the game's memory, accessed as the original does.
    unsafe {
        FUN_004d6460(piv, 0x58, 0x135, 0x1ae, 0x144);
        let buf = &raw mut DAT_00b69d20 as *mut i8;
        let pc3: *mut i8;
        let i2: i32;
        if DAT_00aa6ae8 == -1 {
            i2 = 0;
            pc3 = FUN_004ddc10(0xbb);
        } else {
            if DAT_00aa6114 == 0 || DAT_00aa6138 <= DAT_00aa6134 {
                let src = FUN_004ddc10(0xbb);
                let mut n: usize = 0;
                while *src.add(n) != 0 {
                    n += 1;
                }
                n += 1;
                let mut k = 0usize;
                while k < n {
                    *buf.add(k) = *src.add(k);
                    k += 1;
                }
            } else {
                let v = DAT_00aa6114
                    .wrapping_add(DAT_00aa6134.wrapping_mul(0x148))
                    .wrapping_add(0x40);
                let s = FUN_004ddc10(0xbb);
                _sprintf(buf, &raw const s__s__c_s_0052bfcc as *const i8, s, 4u32, v);
            }
            i2 = 0;
            pc3 = buf;
        }
        FUN_004db7d0(0xc, pc3, piv, i2, 1, 0xffff);
        if DAT_00aa6ae8 != -1 && !DAT_00aa6118.is_null() {
            FUN_004d6460(piv, 0x58, 0x148, 0x1a4, 0x1aa);
            let r = FUN_004dbbd0(0xc, DAT_00aa6118, piv, 0xffff, 4, 0x1000);
            let mut u7: i32 = 0x1ae;
            let r1 = (r as u32).wrapping_add(1);
            if 6 < r1 {
                FUN_004d6460(piv, 0x1a5, 0x148, 0x1ae, 0x1aa);
                DAT_0052c208 = FUN_004d9d70(piv, DAT_0052c208, r1, 0, 5, DAT_00bb2810, 6);
                u7 = 0x1a4;
            }
            FUN_004d6460(piv, 0x58, 0x148, u7, 0x1aa);
            FUN_004dbbd0(0xc, DAT_00aa6118, piv, 0xffff, 4, DAT_0052c208);
        }
    }
    1
}
