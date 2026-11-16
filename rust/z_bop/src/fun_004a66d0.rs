// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005181fc: f32;
}

#[inline(always)]
unsafe fn rd(a: u32) -> f32 {
    // SAFETY: the address is inside the caller's float table, as in the original.
    unsafe { (a as *const f32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a66d0(
    param_1: *mut f32,
    param_2: i32,
    param_3: *mut i32,
    param_4: *mut u32,
) -> u32 {
    // SAFETY: all pointers are the caller's buffers and the game's own globals,
    // read and written where the original does.
    unsafe {
        let base = param_2 as u32;
        let mut local_4: f32 = param_1.read_unaligned();
        let mut local_14: f32 = param_1.add(1).read_unaligned();
        let mut local_10: f32 = param_1.add(2).read_unaligned();

        if 0x80000000u32 < local_4.to_bits() {
            local_4 = -local_4;
        }
        if 0x80000000u32 < local_14.to_bits() {
            local_14 = -local_14;
        }
        if 0x80000000u32 < local_10.to_bits() {
            local_10 = -local_10;
        }

        let i9: u32;
        let i10: u32;
        if (local_10 < local_14) || (local_10 < local_4) {
            if (local_14 < local_4) || (local_14 < local_10) {
                i9 = 1;
                i10 = 2;
            } else {
                i9 = 0;
                i10 = 2;
                param_4.add(1).write_unaligned(param_4.read_unaligned());
            }
        } else {
            i9 = 0;
            i10 = 1;
            param_4
                .add(2)
                .write_unaligned(param_4.add(1).read_unaligned());
            param_4.add(1).write_unaligned(param_4.read_unaligned());
        }

        let p3_0 = param_3.read_unaligned() as u32;
        let p3_1 = param_3.add(1).read_unaligned() as u32;
        let p3_2 = param_3.add(2).read_unaligned() as u32;
        let p3_3 = param_3.add(3).read_unaligned();

        let i1 = i9.wrapping_add(p3_0.wrapping_mul(3));
        let f3a = rd(base.wrapping_add(i1.wrapping_mul(4)));
        let a2 = base.wrapping_add(p3_0.wrapping_mul(3).wrapping_add(i10).wrapping_mul(4));
        let f4a = rd(a2);
        let f5 = rd(base.wrapping_add(i9.wrapping_add(p3_1.wrapping_mul(3)).wrapping_mul(4)));
        let f6 = rd(base.wrapping_add(p3_1.wrapping_mul(3).wrapping_add(i10).wrapping_mul(4)));

        let p41 = f32::from_bits(param_4.add(1).read_unaligned());
        let p42 = f32::from_bits(param_4.add(2).read_unaligned());

        let b7: bool = ((f6 - f4a) * (p41 - f3a) - (f5 - f3a) * (p42 - f4a)) < _DAT_005181fc;

        let f3 = rd(base.wrapping_add(i9.wrapping_add(p3_2.wrapping_mul(3)).wrapping_mul(4)));
        let f4 = rd(base.wrapping_add(p3_2.wrapping_mul(3).wrapping_add(i10).wrapping_mul(4)));
        let f5n = (f4 - f6) * (p41 - f5) - (f3 - f5) * (p42 - f6);

        if (f5n < _DAT_005181fc) && !b7 {
            return 0;
        }
        if (f5n.to_bits() < 0x80000001u32) && b7 {
            return 0;
        }

        let mut local_c = f4;
        let mut local_8 = f3;
        if p3_3 != -1 {
            let i8 = (p3_3 as u32).wrapping_mul(3);
            local_8 = rd(base.wrapping_add(i9.wrapping_add(i8).wrapping_mul(4)));
            local_c = rd(base.wrapping_add(i10.wrapping_add(i8).wrapping_mul(4)));
            let f3b = (local_c - f4) * (p41 - f3) - (local_8 - f3) * (p42 - f4);
            if (f3b < _DAT_005181fc) && !b7 {
                return 0;
            }
            if (f3b.to_bits() < 0x80000001u32) && b7 {
                return 0;
            }
        }

        let f3c = (p41 - local_8) * (rd(a2) - local_c)
            - (rd(base.wrapping_add(i1.wrapping_mul(4))) - local_8) * (p42 - local_c);
        if (f3c < _DAT_005181fc) && !b7 {
            return 0;
        }
        if (f3c.to_bits() < 0x80000001u32) && b7 {
            return 0;
        }
        1
    }
}
