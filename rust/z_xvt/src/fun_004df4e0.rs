// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759657: i32;
    static mut DAT_00759ad3: u8;
    static mut DAT_007596d3: [i8; 1024];
    static mut DAT_00759acf: u8;
    static mut DAT_00759ad0: u8;
    static mut DAT_00759ad1: u8;
    static mut _DAT_00759ad4: i32;
    static mut _DAT_00759ad8: i32;
    static mut _DAT_00759adc: i32;
    static mut _DAT_00759ae0: i32;
    static mut _DAT_00759ae4: i32;
    static mut _DAT_00759ae8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df760(a: i32, b: i32, c: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df4e0(param_1: *mut i32, param_2: i32) -> i32 {
    // SAFETY: the globals and the callee are the game's own; param_1 points
    // at 0x100 dwords supplied by the caller, copied into the game's table.
    unsafe {
        if DAT_00759657 != 8 {
            return 0;
        }
        DAT_00759ad3 = 1;
        let mut src = param_1;
        let mut dst = (&raw mut DAT_007596d3).cast::<i8>();
        let mut count: i32 = 0x100;
        while count != 0 {
            let v = src.read_unaligned();
            dst.cast::<i32>().write_unaligned(v);
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(4);
            count -= 1;
        }
        if param_2 != 0 {
            let base = (&raw mut DAT_007596d3).cast::<i8>();
            base.write(0);
            base.add(1).write(0);
            base.add(2).write(0);
            DAT_00759acf = 0xff;
            DAT_00759ad0 = 0xff;
            DAT_00759ad1 = 0xff;
        }
        _DAT_00759ad4 = 0xffff;
        _DAT_00759ad8 = FUN_004df760(0, 0xff, 0);
        _DAT_00759adc = FUN_004df760(0xff, 0, 0);
        _DAT_00759ae0 = FUN_004df760(0xff, 0xff, 0);
        _DAT_00759ae4 = FUN_004df760(0x32, 0x32, 0xff);
        _DAT_00759ae8 = FUN_004df760(0x80, 0x80, 0xff);
        1
    }
}
