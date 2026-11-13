// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4a50: i32;
    static mut DAT_006ffbec: i32;
    static mut DAT_0072f4e8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004264b0(param_1: i32) {
    // SAFETY: param_1 points at the caller's struct; the globals are the
    // game's own memory, read and written where the original does.
    unsafe {
        let p = param_1 as *mut u8;

        let a = p.byte_add(4).cast::<i32>();
        let v = a.read_unaligned().wrapping_add(DAT_006d4a50);
        a.write_unaligned(v);
        if v < -0x1000000 {
            a.write_unaligned(0xff000000u32 as i32);
        }
        if 0x1000000 < a.read_unaligned() {
            a.write_unaligned(0x1000000);
        }

        let b = p.byte_add(8).cast::<i32>();
        let v = b.read_unaligned().wrapping_add(DAT_006ffbec);
        b.write_unaligned(v);
        if v < -0x1000000 {
            b.write_unaligned(0xff000000u32 as i32);
        }
        if 0x1000000 < b.read_unaligned() {
            b.write_unaligned(0x1000000);
        }

        let c = p.byte_add(0xc).cast::<i32>();
        let v = c.read_unaligned().wrapping_add(DAT_0072f4e8);
        c.write_unaligned(v);
        if v < -0x1000000 {
            c.write_unaligned(0xff000000u32 as i32);
        }
        if 0x1000000 < c.read_unaligned() {
            c.write_unaligned(0x1000000);
        }
    }
}
