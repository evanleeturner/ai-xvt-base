// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081bfb6: u8;
    static mut DAT_007438d8: [u8; 1];
    static mut DAT_0081f7ea: [u8; 1];
    static mut DAT_0081faaa: [u8; 1];
    static mut DAT_0081b740: [i8; 1];
    static mut DAT_005321b8: [i8; 1];
    static mut DAT_005321a8: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: i32) -> *mut i8;
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004da7c0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fc650() -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all globals and callees are the game's own memory and code; the
    // table pointer walks the game's records exactly as the original does.
    unsafe {
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x54, 0x5a, 0x1b2, 0x6a);
        let pc = FUN_004dcfa0(300);
        FUN_004da920(0xf, pc, local_10.as_mut_ptr(), 0xffff);

        let pc = FUN_004dcfa0(0xae);
        FUN_004da7c0(0xc, pc, 0x58, 0x72, DAT_0081bcd0);
        let pc = FUN_004dcfa0(0x13d);
        FUN_004da7c0(0xc, pc, 0xf8, 0x72, DAT_0081bcd0);
        let pc = FUN_004dcfa0(0x143);
        FUN_004da7c0(0xc, pc, 0x134, 0x72, DAT_0081bcd0);
        let pc = FUN_004dcfa0(0x144);
        FUN_004da7c0(0xc, pc, 0x170, 0x72, DAT_0081bcd0);

        let mut local_20: i32 = 0;
        let mut local_1c: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_14: i32 = 0;
        let mut i_var3: i32 = 0;
        let mut i_var4: i32 = 0x86;
        let mut p: *mut u8 = &raw mut DAT_0081f7ea as *mut u8;
        let end = &raw mut DAT_0081faaa as *mut u8 as usize;
        let buf = &raw mut DAT_0081b740 as *mut i8;
        let f1 = &raw const DAT_005321b8 as *const i8;
        let f2 = &raw const DAT_005321a8 as *const i8;
        loop {
            let m3 = p.byte_sub(12).cast::<i32>().read_unaligned();
            if m3 != 0 {
                let m4 = p.byte_sub(16).cast::<i32>().read_unaligned();
                let idx = (m4 as u32).wrapping_mul(0x562) as i32;
                let tbl = (&raw mut DAT_007438d8 as *mut u8).offset(idx as isize);
                if tbl.read() == DAT_0081bfb6 {
                    let a0 = p.cast::<i32>().read_unaligned();
                    let a1 = p.byte_add(4).cast::<i32>().read_unaligned();
                    let a2 = p.byte_add(8).cast::<i32>().read_unaligned();
                    let a3 = p.byte_add(12).cast::<i32>().read_unaligned();
                    let a4 = p.byte_add(16).cast::<i32>().read_unaligned();
                    let m1 = p.byte_sub(4).cast::<i32>().read_unaligned();
                    i_var3 = i_var3.wrapping_add(a0);
                    local_20 = local_20.wrapping_add(a1);
                    local_1c = local_1c.wrapping_add(a3);
                    local_18 = local_18.wrapping_add(a2);
                    local_14 = local_14.wrapping_add(a4);
                    FUN_004da7c0(0xc, p.byte_sub(0x1e) as *mut i8, 0x58, i_var4, 0xffff);
                    sprintf(buf, f1, m1);
                    FUN_004da7c0(0xc, buf, 0xf8, i_var4, 0xffff);
                    sprintf(buf, f2, a0, a1);
                    FUN_004da7c0(0xc, buf, 0x134, i_var4, 0xffff);
                    sprintf(buf, f1, a2);
                    FUN_004da7c0(0xc, buf, 0x170, i_var4, 0xffff);
                }
                i_var4 = i_var4.wrapping_add(0x14);
            }
            p = p.byte_add(0x58);
            if (p as usize) >= end {
                break;
            }
        }

        let pc = FUN_004dcfa0(0x11c);
        FUN_004da7c0(0xc, pc, 0x58, 0x132, DAT_0081bcd0);
        sprintf(buf, f1, i_var3);
        FUN_004da7c0(0xc, buf, 0x120, 0x132, 0xffff);

        let pc = FUN_004dcfa0(0x141);
        FUN_004da7c0(0xc, pc, 0x58, 0x146, DAT_0081bcd0);
        sprintf(buf, f1, local_20);
        FUN_004da7c0(0xc, buf, 0x120, 0x146, 0xffff);

        let pc = FUN_004dcfa0(0x11e);
        FUN_004da7c0(0xc, pc, 0x58, 0x15a, DAT_0081bcd0);
        sprintf(buf, f1, local_1c);
        FUN_004da7c0(0xc, buf, 0x120, 0x15a, 0xffff);

        let pc = FUN_004dcfa0(0x11f);
        FUN_004da7c0(0xc, pc, 0x58, 0x16e, DAT_0081bcd0);
        sprintf(buf, f1, local_18);
        FUN_004da7c0(0xc, buf, 0x120, 0x16e, 0xffff);

        let pc = FUN_004dcfa0(0x122);
        FUN_004da7c0(0xc, pc, 0x58, 0x182, DAT_0081bcd0);
        sprintf(buf, f1, local_14);
        FUN_004da7c0(0xc, buf, 0x120, 0x182, 0xffff);
    }
    1
}
