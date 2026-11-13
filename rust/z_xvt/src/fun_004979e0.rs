// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d50: i16;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d56: i32;
    static mut DAT_006da9f0: i16;
    static mut DAT_00732e40: i16;
    static mut DAT_00732e54: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00418cd0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0042b250(param_1: i32) -> u32;
}

#[inline(always)]
fn p(a: i32) -> *mut u8 {
    a as u32 as usize as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004979e0(param_1: i16, param_2: i32) {
    // SAFETY: all globals and the memory they point to belong to the game;
    // the accesses are made at the same offsets and in the same order as the original.
    unsafe {
        let idx = (DAT_006d9d50 as i32 as u32).wrapping_mul(0x23);
        let i_var4 = (idx as i32).wrapping_add(DAT_006d4988);

        let v1f = p(DAT_006d9d56.wrapping_add(0x1f))
            .cast::<i32>()
            .read_unaligned();
        let v23 = p(DAT_006d9d56.wrapping_add(0x23))
            .cast::<i32>()
            .read_unaligned();
        let v27 = p(DAT_006d9d56.wrapping_add(0x27))
            .cast::<i32>()
            .read_unaligned();
        let v4 = p(i_var4.wrapping_add(4)).cast::<i32>().read_unaligned();
        let v8 = p(i_var4.wrapping_add(8)).cast::<i32>().read_unaligned();
        let vc = p(i_var4.wrapping_add(0xc)).cast::<i32>().read_unaligned();

        FUN_00418cd0(
            v1f.wrapping_sub(v4),
            v23.wrapping_sub(v8),
            v27.wrapping_sub(vc),
        );

        p(DAT_006d9d56.wrapping_add(0x32))
            .cast::<i16>()
            .write_unaligned(DAT_006da9f0.wrapping_add(param_1));
        let u_var2 = FUN_0042b250(DAT_00732e54);
        let ps = p(((DAT_006d9d50 as i32 as u32).wrapping_mul(0x23) as i32)
            .wrapping_add(0x10)
            .wrapping_add(DAT_006d4988))
        .cast::<u16>();
        let cur = ps.read_unaligned();
        let other = p(DAT_006d9d56.wrapping_add(0x32))
            .cast::<i16>()
            .read_unaligned();
        let mut u_var3: u16 = cur.wrapping_sub(other as u16);
        if 0x7fff < u_var3 {
            u_var3 = u_var3.wrapping_neg();
        }
        if u_var3 < 0x301 {
            let nv = p(DAT_006d9d56.wrapping_add(0x32))
                .cast::<i16>()
                .read_unaligned();
            ps.write_unaligned(nv as u16);
            let base = p(((DAT_006d9d50 as i32 as u32).wrapping_mul(0x23) as i32)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_006d4988))
            .cast::<i32>()
            .read_unaligned();
            *p(base.wrapping_add(0x90)) = 1;
            let b = *p(base.wrapping_add(0x90));
            *p(base.wrapping_add(0x89)) = b;
            *p(DAT_00732e54.wrapping_add(0xab)) = 3;
        } else {
            *p(DAT_00732e54.wrapping_add(0xab)) = 2;
            p(DAT_00732e54.wrapping_add(0xac))
                .cast::<u16>()
                .write_unaligned((((u_var2 >> 1) & 0x7fff) as u16).wrapping_add(0x4000));
        }
        if param_2 != 0 {
            p(DAT_006d9d56.wrapping_add(0x2e))
                .cast::<u16>()
                .write_unaligned(DAT_00732e40 as u16);
            p(DAT_00732e54.wrapping_add(0x9e))
                .cast::<u16>()
                .write_unaligned(0xffff);
            *p(DAT_00732e54.wrapping_add(0x9d)) = 0;
            let v0d = p(DAT_00732e54.wrapping_add(0xd))
                .cast::<u16>()
                .read_unaligned();
            let v2e = p(DAT_006d9d56.wrapping_add(0x2e))
                .cast::<u16>()
                .read_unaligned();
            if v0d < v2e {
                *p(DAT_00732e54.wrapping_add(0x9c)) = 2;
            } else {
                *p(DAT_00732e54.wrapping_add(0x9c)) = 1;
            }
            *p(DAT_00732e54.wrapping_add(0x96)) = 0;
            *p(DAT_00732e54.wrapping_add(0x97)) = 0;
        }
    }
}
