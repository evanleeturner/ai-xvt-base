// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ed0e: i8;
    static mut DAT_0073b650: i32;
    static mut DAT_00740880: i32;
    static mut DAT_006d4a44: i32;
    static mut DAT_0073b4b0: i8;
    static mut DAT_0073b660: i8;
    static mut DAT_0073b540: i8;
    static mut DAT_0073b6e0: i8;
    static mut DAT_00741108: i8;
    static mut DAT_0073b4c0: i8;
    static mut _DAT_00741106: i16;
    static mut DAT_0073b550: i16;
    static mut DAT_0073b661: i8;
    static mut DAT_0073b6e1: i8;
    static mut DAT_0073b4c1: i8;
    static mut DAT_00740a80: i16;
    static mut DAT_0085e79d: i8;
    static mut DAT_0073ba30: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040c6e0();
    fn FUN_00439830(param_1: i32, param_2: i32) -> u32;
    fn FUN_00490be0(param_1: *mut i8, param_2: i32, param_3: i32, param_4: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040c5b0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointer accesses mirror the original's array walks.
    unsafe {
        if DAT_0071ed0e != 0 {
            FUN_0040c6e0();
            if DAT_0073b650 == 0 || FUN_00439830(DAT_0073b650, 0x100) == 0 {
                let i_var1 = DAT_00740880;
                DAT_0073b650 = 0;
                if DAT_006d4a44 != 0 {
                    DAT_0073b4b0 = DAT_0073b660;
                    DAT_006d4a44 = DAT_006d4a44.wrapping_sub(1);
                    DAT_0073b540 = DAT_0073b6e0;
                    DAT_00741108 = DAT_0073b4c0;
                    _DAT_00741106 = DAT_0073b550;
                    if DAT_006d4a44 != 0 {
                        let u_var3 = DAT_006d4a44 as u32;
                        let mut u_var2: u32 = 0;
                        let mut p5: *mut i32 = &raw mut DAT_00740880;
                        let mut p6: *mut i16 = &raw mut DAT_0073b550;
                        let a660 = &raw mut DAT_0073b660;
                        let a661 = &raw mut DAT_0073b661;
                        let a6e0 = &raw mut DAT_0073b6e0;
                        let a6e1 = &raw mut DAT_0073b6e1;
                        let a4c0 = &raw mut DAT_0073b4c0;
                        let a4c1 = &raw mut DAT_0073b4c1;
                        loop {
                            let u_var4 = u_var2.wrapping_add(1);
                            p5.write_unaligned(p5.add(1).read_unaligned());
                            let k = u_var2 as usize;
                            *a660.add(k) = *a661.add(k);
                            *a6e0.add(k) = *a6e1.add(k);
                            *a4c0.add(k) = *a4c1.add(k);
                            p6.write_unaligned(p6.add(1).read_unaligned());
                            u_var2 = u_var4;
                            p5 = p5.add(1);
                            p6 = p6.add(1);
                            if !(u_var4 < u_var3) {
                                break;
                            }
                        }
                    }
                    let off = i_var1.wrapping_mul(2) as isize;
                    let v = (&raw mut DAT_00740a80)
                        .cast::<u8>()
                        .byte_offset(off)
                        .cast::<i16>()
                        .read_unaligned();
                    if v != 0 && DAT_0085e79d != 0 {
                        let off6 = i_var1.wrapping_mul(6) as isize;
                        FUN_00490be0(
                            (&raw mut DAT_0073ba30).byte_offset(off6),
                            1,
                            0,
                            0x7e as *mut i8,
                        );
                        DAT_0073b650 = i_var1;
                    }
                }
            }
        }
    }
}
