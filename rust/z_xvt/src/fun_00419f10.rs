// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005200ac: f32;
}

#[allow(non_snake_case)]
unsafe fn ld(base: i32, idx: u32) -> f32 {
    // SAFETY: the caller's table lives at base; the offset is computed as the original does.
    unsafe {
        (base.wrapping_add(idx.wrapping_mul(4) as i32) as u32 as usize as *const f32)
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00419f10(
    param_1: *mut f32,
    param_2: i32,
    param_3: *mut i32,
    param_4: *mut u32,
) -> u32 {
    // SAFETY: all pointers belong to the caller and the global is the game's own;
    // reads and writes follow the original's order.
    unsafe {
        let mut local_4 = param_1.read_unaligned();
        let mut local_14 = param_1.add(1).read_unaligned();
        let mut local_10 = param_1.add(2).read_unaligned();

        if 0x8000_0000u32 < local_4.to_bits() {
            local_4 = -local_4;
        }
        if 0x8000_0000u32 < local_14.to_bits() {
            local_14 = -local_14;
        }
        if 0x8000_0000u32 < local_10.to_bits() {
            local_10 = -local_10;
        }

        let i_var9: u32;
        let i_var10: u32;
        if (local_10 < local_14) || (local_10 < local_4) {
            if (local_14 < local_4) || (local_14 < local_10) {
                i_var9 = 1;
                i_var10 = 2;
            } else {
                i_var9 = 0;
                i_var10 = 2;
                param_4.add(1).write_unaligned(param_4.read_unaligned());
            }
        } else {
            i_var9 = 0;
            i_var10 = 1;
            param_4
                .add(2)
                .write_unaligned(param_4.add(1).read_unaligned());
            param_4.add(1).write_unaligned(param_4.read_unaligned());
        }

        let f1 = f32::from_bits(param_4.add(1).read_unaligned());
        let f2 = f32::from_bits(param_4.add(2).read_unaligned());

        let p0 = param_3.read_unaligned() as u32;
        let p1 = param_3.add(1).read_unaligned() as u32;
        let p2 = param_3.add(2).read_unaligned() as u32;

        let i_var1 = p0.wrapping_mul(3).wrapping_add(i_var9);
        let mut f_var3 = ld(param_2, i_var1);
        let idx2 = p0.wrapping_mul(3).wrapping_add(i_var10);
        let mut f_var4 = ld(param_2, idx2);
        let mut f_var5 = ld(param_2, p1.wrapping_mul(3).wrapping_add(i_var9));
        let f_var6 = ld(param_2, p1.wrapping_mul(3).wrapping_add(i_var10));
        let b_var7: u8 = (((f1 - f_var3) * (f_var6 - f_var4) - (f2 - f_var4) * (f_var5 - f_var3))
            < DAT_005200ac) as u8;

        f_var3 = ld(param_2, p2.wrapping_mul(3).wrapping_add(i_var9));
        f_var4 = ld(param_2, p2.wrapping_mul(3).wrapping_add(i_var10));
        f_var5 = (f1 - f_var5) * (f_var4 - f_var6) - (f2 - f_var6) * (f_var3 - f_var5);

        if (f_var5 < DAT_005200ac) && b_var7 == 0 {
            return 0;
        }
        if f_var5.to_bits() < 0x8000_0001u32 && b_var7 != 0 {
            return 0;
        }

        let mut local_c = f_var4;
        let mut local_8 = f_var3;
        let p3 = param_3.add(3).read_unaligned();
        if p3 != -1 {
            let i_var8 = (p3 as u32).wrapping_mul(3);
            local_8 = ld(param_2, i_var9.wrapping_add(i_var8));
            local_c = ld(param_2, i_var8.wrapping_add(i_var10));
            f_var3 = (f1 - f_var3) * (local_c - f_var4) - (f2 - f_var4) * (local_8 - f_var3);
            if (f_var3 < DAT_005200ac) && b_var7 == 0 {
                return 0;
            }
            if f_var3.to_bits() < 0x8000_0001u32 && b_var7 != 0 {
                return 0;
            }
        }

        f_var3 = (f1 - local_8) * (ld(param_2, idx2) - local_c)
            - (ld(param_2, i_var1) - local_8) * (f2 - local_c);
        if (f_var3 < DAT_005200ac) && b_var7 == 0 {
            return 0;
        }
        if f_var3.to_bits() < 0x8000_0001u32 && b_var7 != 0 {
            return 0;
        }
        1
    }
}
