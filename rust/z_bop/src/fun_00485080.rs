// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9680: i8;
    static mut DAT_009ec468: i8;
    static mut DAT_009e9683: i8;
    static mut DAT_009e9a80: i8;
    static mut DAT_009e9a84: i8;
    static mut DAT_009e9a86: i8;
    static mut DAT_009e9a67: i8;
    static mut DAT_009e9a69: i8;
    static mut DAT_009e9a63: i8;
    static mut DAT_009e968a: i8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9a5b: i8;
    static mut DAT_009e9a5f: i8;
    static mut DAT_00521f1c: i8;
    static mut DAT_009e9681: i8;
    static mut DAT_009e9670: i8;
    static mut DAT_009e9c1d: i8;
    static mut DAT_009e96b6: i8;
    static mut DAT_009d6940: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427720(param_1: u32, param_2: i32);
    fn FUN_0042f270();
    fn FUN_0042f110();
    fn FUN_0042f5d0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00485080(param_1: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets are computed exactly as the original does.
    unsafe {
        let mut pc: *mut i8 = &raw mut DAT_009e9680;
        let i_var1: i32 = param_1.wrapping_mul(0x5bd);
        let mut i_var2: i32 = 0;
        let off = |a: *mut i8| -> *mut u8 { a.byte_offset(i_var1 as isize).cast::<u8>() };
        off(&raw mut DAT_009e9680).write(2);
        let end: *mut i8 = &raw mut DAT_009ec468;
        loop {
            if *pc == 1 {
                i_var2 = i_var2.wrapping_add(1);
            }
            pc = pc.byte_add(0x5bd);
            if !(pc < end) {
                break;
            }
        }

        if i_var2 != 0 {
            off(&raw mut DAT_009e9683).write(0xff);
            FUN_00427720(0x15, param_1);
            off(&raw mut DAT_009e9a80).cast::<u16>().write_unaligned(1);
            off(&raw mut DAT_009e9a84).cast::<u16>().write_unaligned(1);
            off(&raw mut DAT_009e9a86)
                .cast::<u32>()
                .write_unaligned(0x40000);
            off(&raw mut DAT_009e9a67)
                .cast::<u16>()
                .write_unaligned(0xffff);
            off(&raw mut DAT_009e9a69)
                .cast::<u16>()
                .write_unaligned(0xffff);
            off(&raw mut DAT_009e9a63)
                .cast::<u32>()
                .write_unaligned(0x40000);
            let idx: u16 = off(&raw mut DAT_009e968a).cast::<u16>().read_unaligned();
            if idx != 0xffff {
                let base = (idx as u32).wrapping_mul(0x23);
                let tbl = DAT_009a1fe8 as u32;
                let src1 = base.wrapping_add(4).wrapping_add(tbl);
                let val1: u32 = (src1 as usize as *const u32).read_unaligned();
                off(&raw mut DAT_009e9a5b)
                    .cast::<u32>()
                    .write_unaligned(val1);

                let src2 = base.wrapping_add(8).wrapping_add(tbl);
                let val2: u32 = (src2 as usize as *const u32).read_unaligned();
                off(&raw mut DAT_009e9a5f)
                    .cast::<u32>()
                    .write_unaligned(val2);

                let byte_src = base.wrapping_add(3).wrapping_add(tbl);
                let b: u8 = (byte_src as usize as *const u8).read();
                let int_src = ((&raw mut DAT_00521f1c) as usize as u32)
                    .wrapping_add((b as u32).wrapping_mul(0x1a));
                let val3: i32 = (int_src as usize as *const i32).read_unaligned();
                let val3 = val3.wrapping_shl(4);
                off(&raw mut DAT_009e9a86)
                    .cast::<i32>()
                    .write_unaligned(val3);
            }
            off(&raw mut DAT_009e9681).write(0);
            off(&raw mut DAT_009e9670)
                .cast::<u32>()
                .write_unaligned(0xffff_ffff);
            off(&raw mut DAT_009e9c1d).cast::<u32>().write_unaligned(0);
            off(&raw mut DAT_009e96b6).write(0);
            FUN_0042f270();
            FUN_0042f110();
            FUN_0042f5d0();
            return;
        }
        DAT_009d6940 = 1;
    }
}
