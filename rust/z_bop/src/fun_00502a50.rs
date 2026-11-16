// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b6a5a6: u8;
    static mut DAT_00a91d18: [u8; 0];
    static mut DAT_00b6d5b6: [u8; 0];
    static mut DAT_00b6d876: u8;
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_0052b460: [i8; 0];
    static mut s__d__d__0052b450: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004db4e0(
        param_1: i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: u32,
    ) -> u32;
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00502a50() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the table is walked with the same stride and bounds as the original.
    unsafe {
        let mut local_10: [i32; 4] = [0; 4];
        FUN_004d6460(local_10.as_mut_ptr(), 0x54, 0x5a, 0x1b2, 0x6a);
        let mut pc = FUN_004ddc10(0x139);
        FUN_004db640(0xf, pc, local_10.as_mut_ptr(), 0xffff);
        pc = FUN_004ddc10(0xba);
        FUN_004db4e0(0xc, pc, 0x58, 0x72, DAT_00b6a2c0);
        pc = FUN_004ddc10(0x14a);
        FUN_004db4e0(0xc, pc, 0xf8, 0x72, DAT_00b6a2c0);
        pc = FUN_004ddc10(0x150);
        FUN_004db4e0(0xc, pc, 0x134, 0x72, DAT_00b6a2c0);
        let mut i_var3: i32 = 0;
        pc = FUN_004ddc10(0x151);
        FUN_004db4e0(0xc, pc, 0x170, 0x72, DAT_00b6a2c0);
        let mut local_20: i32 = 0;
        let mut local_1c: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_14: i32 = 0;
        let mut i_var4: i32 = 0x86;
        let mut p: *mut u8 = (&raw mut DAT_00b6d5b6).cast::<u8>();
        let end: *mut u8 = (&raw mut DAT_00b6d876).cast::<u8>();
        let buf: *mut i8 = (&raw mut DAT_00b69d20).cast::<i8>();
        let fmt1: *const i8 = (&raw const DAT_0052b460).cast::<i8>();
        let fmt2: *const i8 = (&raw const s__d__d__0052b450).cast::<i8>();
        loop {
            let f_m3 = p.byte_sub(12).cast::<i32>().read_unaligned();
            if f_m3 != 0 {
                let f_m4 = p.byte_sub(16).cast::<i32>().read_unaligned();
                let idx = (f_m4 as u32).wrapping_mul(0x562) as i32;
                let b = (&raw const DAT_00a91d18)
                    .cast::<u8>()
                    .offset(idx as isize)
                    .read();
                if b == DAT_00b6a5a6 {
                    let f0 = p.cast::<i32>().read_unaligned();
                    let f1 = p.byte_add(4).cast::<i32>().read_unaligned();
                    let f2 = p.byte_add(8).cast::<i32>().read_unaligned();
                    let f3 = p.byte_add(12).cast::<i32>().read_unaligned();
                    let f4 = p.byte_add(16).cast::<i32>().read_unaligned();
                    i_var3 = i_var3.wrapping_add(f0);
                    local_20 = local_20.wrapping_add(f1);
                    local_1c = local_1c.wrapping_add(f3);
                    local_18 = local_18.wrapping_add(f2);
                    local_14 = local_14.wrapping_add(f4);
                    FUN_004db4e0(0xc, p.byte_sub(0x1e).cast::<i8>(), 0x58, i_var4, 0xffff);
                    let f_m1 = p.byte_sub(4).cast::<i32>().read_unaligned();
                    _sprintf(buf, fmt1, f_m1);
                    FUN_004db4e0(0xc, buf, 0xf8, i_var4, 0xffff);
                    _sprintf(buf, fmt2, f0, f1);
                    FUN_004db4e0(0xc, buf, 0x134, i_var4, 0xffff);
                    _sprintf(buf, fmt1, f2);
                    FUN_004db4e0(0xc, buf, 0x170, i_var4, 0xffff);
                }
                i_var4 = i_var4.wrapping_add(0x14);
            }
            p = p.byte_add(0x58);
            if !(p < end) {
                break;
            }
        }
        pc = FUN_004ddc10(0x129);
        FUN_004db4e0(0xc, pc, 0x58, 0x132, DAT_00b6a2c0);
        _sprintf(buf, fmt1, i_var3);
        FUN_004db4e0(0xc, buf, 0x120, 0x132, 0xffff);
        pc = FUN_004ddc10(0x14e);
        FUN_004db4e0(0xc, pc, 0x58, 0x146, DAT_00b6a2c0);
        _sprintf(buf, fmt1, local_20);
        FUN_004db4e0(0xc, buf, 0x120, 0x146, 0xffff);
        pc = FUN_004ddc10(299);
        FUN_004db4e0(0xc, pc, 0x58, 0x15a, DAT_00b6a2c0);
        _sprintf(buf, fmt1, local_1c);
        FUN_004db4e0(0xc, buf, 0x120, 0x15a, 0xffff);
        pc = FUN_004ddc10(300);
        FUN_004db4e0(0xc, pc, 0x58, 0x16e, DAT_00b6a2c0);
        _sprintf(buf, fmt1, local_18);
        FUN_004db4e0(0xc, buf, 0x120, 0x16e, 0xffff);
        pc = FUN_004ddc10(0x12f);
        FUN_004db4e0(0xc, pc, 0x58, 0x182, DAT_00b6a2c0);
        _sprintf(buf, fmt1, local_14);
        FUN_004db4e0(0xc, buf, 0x120, 0x182, 0xffff);
        1
    }
}
