// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
    static mut _DAT_009a73b2: i32;
    static mut DAT_009a73b6: *mut u8;
    static mut DAT_009a73ba: u16;
    static mut DAT_009a73bc: i32;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73c2: u16;
    static mut DAT_009a73c4: i32;
    static mut DAT_009a73c8: i32;
    static mut DAT_009a73cc: i32;
    static mut DAT_009a73d0: u16;
    static mut DAT_009a73d2: u16;
    static mut DAT_009a73d4: i32;
    static mut DAT_009a73d8: i32;
    static mut DAT_009a73d9: i8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_00a07cf0: [u8; 0];
    static mut DAT_0051a1d0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00458e20(param_1: u32, param_2: u32);
    fn FUN_00404620(param_1: i32) -> u32;
    fn FUN_00402e00(param_1: u16) -> i32;
    fn FUN_0046b610(param_1: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00402cb0(param_1: u16) {
    // SAFETY: all globals and callees are the game's own; the pointers
    // computed from the game's tables are read where the original reads them.
    unsafe {
        DAT_009a73b0 = param_1;
        let i_var3: i32 = (param_1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32) as i32;

        let tmp_ptr: i32 = ((i_var3.wrapping_add(0x1f)) as usize as *const i32).read_unaligned();
        let tmp_val: i32 = ((tmp_ptr.wrapping_add(0xa9)) as usize as *const i32).read_unaligned();
        let i_var2: i32 = tmp_val;

        DAT_009a73b6 = i_var2.wrapping_add(0x2a) as usize as *mut u8;
        DAT_009a73ba = ((i_var2.wrapping_add(5)) as usize as *const u8).read() as u16;

        if DAT_009a73ba == 0xff {
            DAT_009a73bc = tmp_val;
        } else {
            let addr: i32 = (DAT_009a73ba as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32) as i32;
            let p1: i32 = (addr as usize as *const i32).read_unaligned();
            let p2: i32 = ((p1.wrapping_add(0xa9)) as usize as *const i32).read_unaligned();
            DAT_009a73bc = p2;
        }

        DAT_009a73c0 = ((i_var3.wrapping_add(0x16)) as usize as *const u8).read() as u16;
        DAT_009a73c2 = DAT_009a73b6.read() as u16;
        _DAT_009a73b2 = i_var2;
        FUN_00458e20(param_1 as u32, DAT_009a73c0 as u32);
        DAT_009a73c4 = DAT_00a07cdc;
        DAT_009a73c8 = DAT_00a07cd8;
        DAT_009a73cc = DAT_00a07cd4;
        let u_var1 = FUN_00404620(_DAT_009a73b2);
        let r = FUN_00402e00(u_var1 as u16);
        DAT_009a73d0 = r as u16;

        let idx: usize = ((i_var2.wrapping_add(0x34)) as usize as *const u8).read() as usize;
        let tbl = (&raw const DAT_00a07cf0).cast::<u8>();
        DAT_009a73d4 = (tbl.add(idx).read() as i32).wrapping_add(2);
        DAT_009a73d8 = 0;
        DAT_009a73d2 = tbl.add(idx + 1).read() as u16;

        let r2 = FUN_0046b610(&raw mut DAT_0051a1d0);
        DAT_009a73d9 = r2 as i8;
    }
}
