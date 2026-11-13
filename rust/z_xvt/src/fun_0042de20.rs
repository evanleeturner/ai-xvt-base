// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005373d0: u32;
    static mut DAT_005373d4: u32;
    static mut DAT_005373d8: u32;
    static mut DAT_00732a18: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004310c0(param_1: *mut u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042de20(param_1: *mut u32, param_2: *mut u32, param_3: u32) {
    // SAFETY: the globals and the callee are the game's own memory; param_1
    // and param_2 point at the caller's buffers, read and copied into the
    // game's buffer where the original does.
    unsafe {
        _DAT_005373d0 = 0x18;
        DAT_005373d4 = DAT_00732a18;
        let mut p1 = param_1;
        let mut p2 = param_2;
        let mut p3: *mut u32 = &raw mut DAT_005373d8;
        let mut n = param_3 & 0x3fff_ffff;
        while n != 0 {
            p3.write_unaligned(p1.read_unaligned());
            p1 = p1.wrapping_add(1);
            p3 = p3.wrapping_add(1);
            n -= 1;
        }
        let off = param_3.wrapping_mul(4).wrapping_mul(4);
        p3 = (&raw mut DAT_005373d8)
            .cast::<u8>()
            .wrapping_add(off as usize)
            .cast::<u32>();
        n = param_3 & 0x3fff_ffff;
        while n != 0 {
            p3.write_unaligned(p2.read_unaligned());
            p2 = p2.wrapping_add(1);
            p3 = p3.wrapping_add(1);
            n -= 1;
        }
        FUN_004310c0(
            &raw mut _DAT_005373d0,
            param_3.wrapping_mul(8).wrapping_add(8),
        );
    }
}
