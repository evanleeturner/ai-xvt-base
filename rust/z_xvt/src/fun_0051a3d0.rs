// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00742460: [u16; 0];
    static mut DAT_00742814: Option<unsafe extern "C" fn(i32, i32, i32)>;
    static mut DAT_005366fc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d5500() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051a3d0(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and the callee are the game's own; param_1 addresses
    // the caller's RGB bytes and the table is written where the original does.
    unsafe {
        if param_2 < param_2.wrapping_add(param_3) {
            let mut pb = param_1.wrapping_add(param_2.wrapping_mul(3)) as usize as *const u8;
            let mut local_4 = param_3;
            let mut local_8 = (&raw mut DAT_00742460)
                .cast::<u16>()
                .wrapping_offset(param_2 as isize);
            loop {
                let b1 = pb.read() as u32;
                let b2 = pb.add(1).read() as u32;
                let b3 = pb.add(2).read() as u32;
                let ret = FUN_004d5500();
                let u_var6: u16 = if ret == 0 {
                    ((b3 >> 3) | ((b2 >> 2) << 5) | (((b1 >> 2) & 0xfffe) << 10)) as u16
                } else {
                    ((b3 >> 3) | (((b2 >> 2) & 0xfffe) << 4) | (((b1 >> 2) & 0xfffe) << 9)) as u16
                };
                pb = pb.wrapping_add(3);
                local_8.write_unaligned(u_var6);
                local_4 = local_4.wrapping_sub(1);
                local_8 = local_8.wrapping_add(1);
                if local_4 == 0 {
                    break;
                }
            }
        }
        if let Some(f) = DAT_00742814 {
            if DAT_005366fc == 1 {
                f(param_1, param_2, param_3);
            }
        }
    }
}
