// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6166: u32;
    static mut DAT_00aa62b4: u32;
    static mut DAT_00aa6ae4: u32;
    static mut DAT_00aa6ae8: u32;
    static mut DAT_00aa6aec: u32;
    static mut DAT_00aa5ac0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004eec10() -> u32 {
    // SAFETY: all globals and the callee are the game's own memory; every
    // access goes to the same addresses the original computes.
    unsafe {
        let base = (&raw mut DAT_00aa6af0).cast::<u8>();
        let b_e4 = (&raw mut DAT_00aa6ae4).cast::<u8>();
        let b_e8 = (&raw mut DAT_00aa6ae8).cast::<u8>();
        let b_ec = (&raw mut DAT_00aa6aec).cast::<u8>();

        let mut i_var5: i32 = 1;
        base.write(0x61);
        let mut p7 = (&raw mut DAT_00aa6166).cast::<u8>() as *const u8;
        let limit = (&raw mut DAT_00aa62b4 as usize).wrapping_add(2);
        loop {
            let u2 = p7.byte_add(4).cast::<u32>().read_unaligned();
            base.byte_offset((i_var5 as isize).wrapping_mul(16))
                .cast::<u32>()
                .write_unaligned(p7.cast::<u32>().read_unaligned());
            let u3 = p7.byte_add(8).cast::<u32>().read_unaligned();
            let p6 = p7.byte_add(0x2a);
            b_e4.byte_offset(((i_var5.wrapping_add(4)) as isize).wrapping_mul(4))
                .cast::<u32>()
                .write_unaligned(u2);
            i_var5 = i_var5.wrapping_add(5);
            let u2 = p7.byte_add(12).cast::<u32>().read_unaligned();
            b_e4.byte_offset((i_var5 as isize).wrapping_mul(4))
                .cast::<u32>()
                .write_unaligned(u3);
            let u3 = p7.byte_add(16).cast::<u32>().read_unaligned();
            b_e8.byte_offset((i_var5 as isize).wrapping_mul(4))
                .cast::<u32>()
                .write_unaligned(u2);
            b_ec.byte_offset((i_var5 as isize).wrapping_mul(4))
                .cast::<u32>()
                .write_unaligned(u3);
            p7 = p6;
            if !((p6 as usize) < limit) {
                break;
            }
        }

        let mut p9 = base.byte_offset((i_var5 as isize).wrapping_mul(4));
        let mut src = (&raw mut DAT_00aa5ac0).cast::<u8>() as *const u8;
        let mut i_var10: i32 = 10;
        loop {
            let mut i_var4: i32 = 8;
            loop {
                p9.write(src.read());
                p9 = p9.byte_add(1);
                src = src.byte_add(1);
                i_var4 -= 1;
                if i_var4 == 0 {
                    break;
                }
            }
            i_var10 -= 1;
            if i_var10 == 0 {
                break;
            }
        }

        FUN_004cef70(
            0,
            &raw mut DAT_00aa6af0,
            i_var5.wrapping_mul(4).wrapping_add(0x50) as u32,
        );
        1
    }
}
