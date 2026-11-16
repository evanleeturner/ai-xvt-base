// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ba5c: i32;
    static mut DAT_00bb2818: i32;
    static mut DAT_0052b188: i8;
    static mut DAT_0052b174: i8;
    static mut DAT_0052b160: i8;
    static mut DAT_0052b14c: i8;
    static mut DAT_0052b138: i8;
    static mut DAT_0052b12c: i8;
    static mut DAT_0052b124: i8;
    static mut DAT_0052b110: i8;
    static mut DAT_0052b0fc: i8;
    static mut DAT_0052b0e4: i8;
    static mut DAT_0052b120: i8;
    static mut DAT_00b6a2ac: *mut core::ffi::c_void;
    static mut DAT_00b69d20: i8;
    static mut DAT_00b6a2e0: u8;
    static mut DAT_00b69cd0: *mut u32;
    static mut DAT_00b6a2c4: i32;
    static mut _DAT_00b6a2d8: i32;
    static mut DAT_00bb2810: i32;
    static mut DAT_00b69cc4: i32;
    static mut DAT_00b6a2a0: i32;
    static mut DAT_00b69e38: i32;
    static mut DAT_00b6a2a4: i32;
    static mut DAT_00b6a2c0: i32;
    static mut _DAT_00b69ccc: i32;
    static mut DAT_00b69e24: i32;
    static mut DAT_00b6a264: i32;
    static mut _DAT_00b69cf0: i32;
    static mut _DAT_00b69cf4: i32;
    static mut _DAT_00b69cf8: i32;
    static mut _DAT_00b69cfc: i32;
    static mut _DAT_00b69d00: i32;
    static mut _DAT_00b69d04: i32;
    static mut _DAT_00b69d08: i32;
    static mut _DAT_00b69d0c: i32;
    static mut _DAT_00b69d10: i32;
    static mut _DAT_00b69d14: i32;
    static mut _DAT_00b6a270: i32;
    static mut _DAT_00b6a274: i32;
    static mut _DAT_00b6a278: i32;
    static mut _DAT_00b6a27c: i32;
    static mut _DAT_00b6a280: i32;
    static mut _DAT_00b6a284: i32;
    static mut _DAT_00b6a288: i32;
    static mut _DAT_00b6a28c: i32;
    static mut _DAT_00b6a290: i32;
    static mut _DAT_00b6a294: i32;
    static mut _DAT_00b6a298: i32;
    static mut _DAT_00b6a29c: i32;
    static mut DAT_00b69cac: i32;
    static mut DAT_00bb7351: i8;
    static mut DAT_00664ec4: u32;
    static mut DAT_00bb737f: i8;
    static mut DAT_00bb737e: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004c9ee0();
    fn FUN_004d4dd0();
    fn FUN_004d4c00(param_1: i32);
    fn FUN_004de100();
    fn FUN_004d4bf0();
    fn FUN_004d4f60();
    fn FUN_004dada0(param_1: i32);
    fn FUN_004df880(param_1: *mut i8);
    fn FUN_004df700(param_1: *mut i8);
    fn FUN_004b4cc0(param_1: *mut i8, param_2: *mut i32);
    fn FUN_004b49a0(param_1: *mut i8, param_2: *mut core::ffi::c_void);
    fn FUN_004dd9d0(param_1: *mut i8);
    fn FUN_004df250(param_1: *mut i8);
    fn FUN_004cc060(param_1: *mut i8);
    fn FUN_004d6e30(param_1: u8, param_2: u8, param_3: u8) -> i32;
    fn FUN_004b97e0();
    fn FUN_004c9c50(param_1: i32);
    fn FUN_004bf8c0(param_1: *mut i8);
    fn FUN_004d2e50() -> i32;
    fn FUN_004d3330();
    fn FUN_004d34a0(param_1: u32);
    fn FUN_004d3020(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_004d3140();
    fn free(p: *mut core::ffi::c_void);
    fn malloc(size: u32) -> *mut core::ffi::c_void;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bdb90() -> i32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all globals and callees are the game's own; buffers are the
    // game's own memory, written where the original does.
    unsafe {
        DAT_0052ba5c = 1;
        DAT_00bb2818 = 0;
        FUN_004c9ee0();
        FUN_004d4dd0();
        FUN_004d4c00(0);
        FUN_004de100();
        FUN_004d4bf0();
        FUN_004d4f60();
        FUN_004dada0(0xf);
        FUN_004dada0(0xc);
        FUN_004dada0(10);
        FUN_004df880(&raw mut DAT_0052b188);
        FUN_004df880(&raw mut DAT_0052b174);
        FUN_004df880(&raw mut DAT_0052b160);
        FUN_004df880(&raw mut DAT_0052b14c);
        FUN_004df880(&raw mut DAT_0052b138);
        FUN_004df700(&raw mut DAT_0052b12c);
        FUN_004b4cc0(&raw mut DAT_0052b124, local_10.as_mut_ptr());
        if !DAT_00b6a2ac.is_null() {
            free(DAT_00b6a2ac);
            DAT_00b6a2ac = core::ptr::null_mut();
        }
        DAT_00b6a2ac = malloc(
            ((local_10[2].wrapping_add(1)) as u32)
                .wrapping_mul((local_10[3].wrapping_add(1)) as u32)
                .wrapping_mul(2),
        );
        FUN_004b49a0(&raw mut DAT_0052b124, DAT_00b6a2ac);
        sprintf(
            &raw mut DAT_00b69d20,
            &raw const DAT_0052b120,
            &raw mut DAT_00b6a2e0,
        );
        let base = (&raw mut DAT_00b6a2e0) as *mut u32;
        let mut i: usize = 0;
        while i < 0x1214b {
            base.add(i).write_unaligned(0);
            i += 1;
        }
        base.add(0x1214b).cast::<u16>().write_unaligned(0);
        FUN_004dd9d0(&raw mut DAT_0052b110);
        FUN_004df250(&raw mut DAT_0052b0fc);
        FUN_004cc060(&raw mut DAT_0052b0e4);
        DAT_00b69cd0 = malloc(0x400) as *mut u32;
        DAT_00b6a2c4 = 0;
        if !DAT_00b69cd0.is_null() {
            let mut p = DAT_00b69cd0;
            let mut n = 0x100;
            while n != 0 {
                p.write_unaligned(0);
                p = p.add(1);
                n -= 1;
            }
        }
        _DAT_00b6a2d8 = FUN_004d6e30(0, 0xff, 0);
        DAT_00bb2810 = FUN_004d6e30(0, 0, 0x80);
        DAT_00b69cc4 = FUN_004d6e30(0x40, 0x80, 0x40);
        DAT_00b6a2a0 = FUN_004d6e30(0, 0xff, 0);
        DAT_00b69e38 = FUN_004d6e30(0xff, 0, 0);
        DAT_00b6a2a4 = FUN_004d6e30(0, 0, 0xff);
        DAT_00b6a2c0 = FUN_004d6e30(0xff, 0xff, 0);
        _DAT_00b69ccc = FUN_004d6e30(0x60, 0x60, 0x60);
        DAT_00b69e24 = FUN_004d6e30(0xc4, 0xfc, 0xfc);
        DAT_00b6a264 = FUN_004d6e30(0x30, 0x6f, 0x7b);
        _DAT_00b69cf0 = DAT_00b69e38;
        _DAT_00b69cf4 = DAT_00bb2810;
        _DAT_00b69cf8 = DAT_00b6a2a4;
        _DAT_00b69cfc = DAT_00b6a2c0;
        _DAT_00b69d00 = FUN_004d6e30(0x80, 0, 0xff);
        _DAT_00b69d04 = FUN_004d6e30(0, 0xff, 0x80);
        _DAT_00b69d08 = DAT_00b69cc4;
        _DAT_00b69d0c = FUN_004d6e30(0, 0xff, 0xff);
        _DAT_00b69d10 = FUN_004d6e30(0, 0x80, 0xff);
        _DAT_00b69d14 = FUN_004d6e30(0xff, 0x80, 0);
        _DAT_00b6a270 = FUN_004d6e30(0x80, 0x80, 0);
        _DAT_00b6a274 = FUN_004d6e30(0x95, 0x95, 0);
        _DAT_00b6a278 = FUN_004d6e30(0xaa, 0xaa, 0);
        _DAT_00b6a27c = FUN_004d6e30(0xc0, 0xc0, 0);
        _DAT_00b6a280 = FUN_004d6e30(0xd5, 0xd5, 0);
        _DAT_00b6a284 = FUN_004d6e30(0xea, 0xea, 0);
        _DAT_00b6a288 = FUN_004d6e30(0xff, 0xff, 0);
        _DAT_00b6a28c = FUN_004d6e30(0xea, 0xea, 0);
        _DAT_00b6a290 = FUN_004d6e30(0xd5, 0xd5, 0);
        _DAT_00b6a294 = FUN_004d6e30(0xc0, 0xc0, 0);
        _DAT_00b6a298 = FUN_004d6e30(0xaa, 0xaa, 0);
        _DAT_00b6a29c = FUN_004d6e30(0x95, 0x95, 0);
        FUN_004b97e0();
        FUN_004c9c50(DAT_00b69cac);
        FUN_004bf8c0(&raw mut DAT_00bb7351);
        let r = FUN_004d2e50();
        DAT_00664ec4 = (r == 0) as u32;
        FUN_004d3330();
        if DAT_00bb737f != 0 {
            FUN_004d34a0((DAT_00bb737e as u32).wrapping_mul(0xffff) / 9);
            FUN_004d3020(7, 0, 0);
            return 0;
        }
        FUN_004d3140();
        0
    }
}
