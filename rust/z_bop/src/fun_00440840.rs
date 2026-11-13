// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0a1e0: u16;
    static mut DAT_00a08ca4: u16;
    static mut DAT_00a08ca0: i16;
    static mut DAT_00a08ca2: i16;
    static mut DAT_009ed232: i16;
    static mut DAT_009a20ae: i8;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_009e9670: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_00a08ca6: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9520(param_1: i16, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_004a9590(param_1: u32);
    fn FUN_004a9500(param_1: u16, param_2: u16);
    fn FUN_004277f0(param_1: u16, param_2: u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00440840(param_1: u16, param_2: u16, param_3: u32) {
    let mut param_1 = param_1;
    let idx = param_1 as usize;
    // SAFETY: all globals and callees belong to the game; the addresses
    // computed are the same byte offsets the original uses.
    unsafe {
        let p_a1e0 = (&raw mut DAT_00a0a1e0).byte_add(idx * 4);
        if p_a1e0.read_unaligned() != param_2 {
            p_a1e0.write_unaligned(param_2);
            let p_x = (&raw mut DAT_00a08ca0).byte_add(idx * 0x18);
            let p_y = (&raw mut DAT_00a08ca2).byte_add(idx * 0x18);
            let u_var2: u16 = (&raw mut DAT_00a08ca4)
                .byte_add(idx * 0x18)
                .read_unaligned();
            let x: i16 = p_x.read_unaligned();
            let y: i16 = p_y.read_unaligned();
            let third = (DAT_009ed232 as i32)
                .wrapping_mul(u_var2 as i32)
                .wrapping_add(x as i32)
                .wrapping_add(2) as i16;
            let fourth = ((DAT_009a20ae as i16 as u16) as i32).wrapping_add(y as i32) as i16;
            FUN_004a9520(x, y, third, fourth);
            let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
            f();
            if 0x90 < param_1 {
                let q = ((param_1 as i32 - 1) as u16 as i32) / 0x90;
                param_1 = (param_1 as i32).wrapping_add(q.wrapping_mul(-0x90)) as u16;
            }
            let v: u32;
            if param_2 < 0x15
                && (param_1 == 0x55
                    || param_1 == 0x52
                    || param_1 == 0x66
                    || param_1 == 0x56
                    || param_1 == 0x67)
            {
                v = 0x4a;
            } else if param_2 < 0x33
                && (param_1 == 0x55
                    || param_1 == 0x52
                    || param_1 == 0x56
                    || param_1 == 0x66
                    || param_1 == 0x67)
            {
                v = 0x4e;
            } else if (param_1 == 0x29 || param_1 == 0x28) && {
                let base = (&raw mut DAT_009e9670) as usize as u32;
                let a1 = base.wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd) as u32);
                let v1 = (a1 as usize as *const i32).read_unaligned();
                let a2 = v1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8);
                let v3 = (a2 as usize as *const i32).read_unaligned();
                let v4 = ((v3 as u32).wrapping_add(0xa9) as usize as *const i32).read_unaligned();
                ((v4 as u32).wrapping_add(0xbf) as usize as *const i16).read_unaligned() == 0
            } {
                v = 0x52;
            } else {
                v = (&raw mut DAT_00a08ca6)
                    .byte_add(idx * 0x18)
                    .read_unaligned() as u32;
            }
            FUN_004a9590(v);
            FUN_004a9500(p_x.read_unaligned() as u16, p_y.read_unaligned() as u16);
            FUN_004277f0(param_2, u_var2 as u32, param_3 & 0xffff);
        }
    }
}
