// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005373d4: u32;
    static mut DAT_005373d0: u32;
    static mut DAT_005373d8: u32;
    static mut DAT_00732a18: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432ca0() -> i32;
    fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042ddb0(param_1: *mut u32, param_2: *mut u32, param_3: u32) {
    // SAFETY: the globals and callees are the game's own memory; the
    // pointers are the caller's buffers, read and written where the
    // original does.
    unsafe {
        DAT_005373d4 = DAT_00732a18;
        DAT_005373d0 = 4;
        let mut p1 = param_1;
        let mut p2 = param_2;
        let mut p3: *mut u32 = &raw mut DAT_005373d8;
        let mut n = param_3 & 0x3fffffff;
        while n != 0 {
            p3.write_unaligned(p1.read_unaligned());
            p1 = p1.wrapping_add(1);
            p3 = p3.wrapping_add(1);
            n -= 1;
        }
        let mut p3: *mut u32 = (&raw mut DAT_005373d8).byte_add(param_3.wrapping_mul(4) as usize);
        let mut n = param_3 & 0x3fffffff;
        while n != 0 {
            p3.write_unaligned(p2.read_unaligned());
            p2 = p2.wrapping_add(1);
            p3 = p3.wrapping_add(1);
            n -= 1;
        }
        let size = param_3.wrapping_mul(8).wrapping_add(8);
        let p4: *mut u32 = &raw mut DAT_005373d0;
        let r = FUN_00432ca0();
        FUN_00431110(r, p4, size);
    }
}
