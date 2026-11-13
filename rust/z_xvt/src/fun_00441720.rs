// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b730: u32;
    static mut DAT_006d4988: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00446fa0(param_1: u32) -> i16;
}

unsafe fn chain(v: u32) -> u32 {
    // SAFETY: the pointers walked here are the game's own structures, read
    // exactly as the original does.
    unsafe {
        let base = v
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32);
        let inner = (base as usize as *const u32).read_unaligned();
        (inner.wrapping_add(0xa7) as usize as *const u32).read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00441720(param_1: u32, param_2: u32, param_3: i32) {
    // SAFETY: all addresses are the game's own memory, accessed in the
    // original's order.
    unsafe {
        let p2 = (&raw mut DAT_0071b730)
            .cast::<u8>()
            .byte_offset(param_3.wrapping_mul(0x5bd) as isize)
            .cast::<u32>();
        let i_var1 = (param_2 & 0xffff).wrapping_mul(2);

        let v = p2.read_unaligned();
        let a = chain(v).wrapping_add(0x20b).wrapping_add(i_var1);
        if 0 < (a as usize as *const i16).read_unaligned() {
            let mut s_var7 = FUN_00446fa0(p2.read_unaligned());
            let i_var6 = chain(p2.read_unaligned());
            let psv3 = i_var6
                .wrapping_add(0x20b)
                .wrapping_add((param_1 & 0xffff).wrapping_mul(2)) as usize
                as *mut i16;
            let s_var4 = psv3.read_unaligned();
            s_var7 = s_var7.wrapping_sub(s_var4);
            if 0 < s_var7 {
                let s_var5 = (i_var1.wrapping_add(0x20b).wrapping_add(i_var6) as usize
                    as *const i16)
                    .read_unaligned();
                if s_var7 < s_var5 {
                    psv3.write_unaligned(s_var4.wrapping_add(s_var7));
                    let q = chain(p2.read_unaligned())
                        .wrapping_add(0x20b)
                        .wrapping_add(i_var1) as usize as *mut i16;
                    q.write_unaligned(q.read_unaligned().wrapping_sub(s_var7));
                    return;
                }
                psv3.write_unaligned(s_var4.wrapping_add(s_var5));
                let q = chain(p2.read_unaligned())
                    .wrapping_add(0x20b)
                    .wrapping_add(i_var1) as usize as *mut i16;
                q.write_unaligned(0);
            }
        }
    }
}
