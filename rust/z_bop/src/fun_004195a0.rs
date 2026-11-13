// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00550b8c: u32;
    static mut DAT_0051bf40: u32;
    static mut DAT_00550880: *mut u32;
    static mut DAT_00550b88: *mut u32;
    static mut DAT_00550bd8: u32;
    static mut DAT_00a07cc8: u32;
    static mut DAT_00550b98: u32;
    static mut DAT_00550840: u32;
    static mut DAT_0052341c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00417450();
    fn FUN_004650e0(param_1: *mut u32, param_2: *mut u32, param_3: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00419620();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004197d0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004195a0(param_1: u32, param_2: u32) {
    // SAFETY: the globals and callees are the game's own; the copy reads
    // from and writes to the buffers the game's pointers designate.
    unsafe {
        DAT_00550b8c = param_1;
        DAT_0051bf40 = 1;
        let mut src = DAT_00550880;
        let mut dst = DAT_00550b88;
        let mut n = param_1 >> 2;
        while n != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }
        let mut src_b = src as *const u8;
        let mut dst_b = dst as *mut u8;
        let mut m = param_1 & 3;
        while m != 0 {
            dst_b.write(src_b.read());
            src_b = src_b.add(1);
            dst_b = dst_b.add(1);
            m -= 1;
        }
        DAT_00550bd8 = DAT_00550b8c;
        DAT_00a07cc8 = param_2;
        FUN_00417450();
        FUN_004650e0(&raw mut DAT_00550b98, &raw mut DAT_00550840, 0x10);
        FUN_00419620();
        DAT_0052341c = 0;
        FUN_004197d0();
    }
}
