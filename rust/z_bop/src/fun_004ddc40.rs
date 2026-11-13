// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa734e: u32;
    static mut DAT_00aa7352: u32;
    static mut DAT_00aa7346: *mut u32;
    static mut DAT_00aa734a: *mut u32;
    static mut DAT_00aa721a: u32;
    static mut DAT_00aa727e: u32;
    static mut DAT_0052c100: u32;
    static mut DAT_00aa735e: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ddc40() {
    // SAFETY: all globals are the game's own memory; the copy and clear
    // loops cover the same 0x19 and 0x10 dwords the original does.
    unsafe {
        DAT_00aa734e = 10;
        DAT_00aa7352 = 10;
        DAT_00aa7346 = &raw mut DAT_00aa721a;
        DAT_00aa734a = &raw mut DAT_00aa727e;
        let mut src: *const u32 = &raw mut DAT_0052c100;
        let mut dst: *mut u32 = &raw mut DAT_00aa721a;
        let mut n: i32 = 0x19;
        while n != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        let mut p: *mut u32 = &raw mut DAT_00aa735e;
        let mut m: i32 = 0x10;
        while m != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            m -= 1;
        }
    }
}
