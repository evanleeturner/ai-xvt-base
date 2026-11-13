// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645a84: u32;
    static mut DAT_0052acbc: u32;
    static mut DAT_00645778: *mut u32;
    static mut DAT_00645a80: *mut u32;
    static mut DAT_00645ad0: u32;
    static mut DAT_00732a18: u32;
    static mut DAT_00645a90: u32;
    static mut DAT_00645738: u32;
    static mut DAT_0052555c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00499ce0();
    fn FUN_0042ddb0(param_1: *mut u32, param_2: *mut u32, param_3: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0049c210();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049bfc0(param_1: u32, param_2: u32) {
    // SAFETY: the globals, buffers and callees are the game's own memory;
    // copies run in the same order and sizes as the original.
    unsafe {
        DAT_00645a84 = param_1;
        DAT_0052acbc = 1;
        let mut p2 = DAT_00645778;
        let mut p3 = DAT_00645a80;
        let mut n = param_1 >> 2;
        while n != 0 {
            p3.write_unaligned(p2.read_unaligned());
            p2 = p2.add(1);
            p3 = p3.add(1);
            n = n.wrapping_sub(1);
        }
        let mut b2 = p2 as *const u8;
        let mut b3 = p3 as *mut u8;
        n = param_1 & 3;
        while n != 0 {
            b3.write(b2.read());
            b2 = b2.add(1);
            b3 = b3.add(1);
            n = n.wrapping_sub(1);
        }
        DAT_00645ad0 = DAT_00645a84;
        DAT_00732a18 = param_2;
        FUN_00499ce0();
        FUN_0042ddb0(&raw mut DAT_00645a90, &raw mut DAT_00645738, 0x10);
        DAT_00645a84 = DAT_00645ad0;
        let mut p2 = DAT_00645a80;
        let mut p3 = DAT_00645778;
        n = DAT_00645ad0 >> 2;
        while n != 0 {
            p3.write_unaligned(p2.read_unaligned());
            p2 = p2.add(1);
            p3 = p3.add(1);
            n = n.wrapping_sub(1);
        }
        let mut b2 = p2 as *const u8;
        let mut b3 = p3 as *mut u8;
        n = DAT_00645a84 & 3;
        while n != 0 {
            b3.write(b2.read());
            b2 = b2.add(1);
            b3 = b3.add(1);
            n = n.wrapping_sub(1);
        }
        DAT_0052555c = 0;
        FUN_0049c210();
    }
}
