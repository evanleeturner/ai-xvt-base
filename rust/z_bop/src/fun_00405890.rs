// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9698: i8;
    static mut DAT_009ec5fe: i16;
    static mut DAT_009e9697: i8;
    static mut DAT_0051c5aa: i8;
    static mut DAT_009e9bfc: i32;
    static mut DAT_009ecc34: i32;
    static mut _DAT_00a07be0: i16;
    static mut _DAT_00a07be2: i16;
    static mut DAT_00523424: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00450650(param_1: i32, param_2: i32);
    fn FUN_00406030(param_1: i32, param_2: u32);
    fn FUN_00405ac0(param_1: i32, param_2: u32);
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}
unsafe fn r16(a: i32) -> i16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as u32 as usize as *const i16).read_unaligned() }
}
unsafe fn r8(a: i32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as u32 as usize as *const u8).read() }
}
unsafe fn w32(a: i32, v: i32) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as u32 as usize as *mut i32).write_unaligned(v) }
}
unsafe fn w16(a: i32, v: i16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as u32 as usize as *mut i16).write_unaligned(v) }
}
unsafe fn w8(a: i32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as u32 as usize as *mut u8).write(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405890(param_1: i32) {
    // SAFETY: all globals and addresses are the game's own memory, accessed
    // where the original accesses them.
    unsafe {
        let i_var3 = param_1.wrapping_mul(0x5bd);
        let a9670 = (&raw mut DAT_009e9670) as usize as i32;
        let a97 = ((&raw mut DAT_009e9697) as usize as i32).wrapping_add(i_var3);
        let a98 = ((&raw mut DAT_009e9698) as usize as i32).wrapping_add(i_var3);
        let abfc = ((&raw mut DAT_009e9bfc) as usize as i32).wrapping_add(i_var3);
        let a51 = (&raw mut DAT_0051c5aa) as usize as i32;

        let mut i_var1 = r32(a9670.wrapping_add(i_var3));
        if i_var1 == -1 {
            return;
        }
        let i_var2 = r32(r32(i_var1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8))
        .wrapping_add(0xa9));
        let _ = i_var2;
        let i_var2 = r32(r32(i_var1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8))
        .wrapping_add(0xa9));
        let _ = i_var2;
        let i_var2 = {
            let p = r32(i_var1
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8));
            p.wrapping_add(0xa9)
        };
        let i_var2 = r32(i_var2.wrapping_sub(0xa9).wrapping_add(0xa9));
        if r32(i_var2.wrapping_add(0x1d)) != 0
            && (r8(i_var2.wrapping_add(0x246)) != 1 || r16(i_var2.wrapping_add(0x248)) == 0)
        {
            FUN_00450650(0x169, param_1);
            return;
        }
        let d = DAT_009ec5fe;
        let thr = ((d >> 1) as i32).wrapping_add(d as i32) as i16;
        if r8(a98) != 0 {
            let b = r8(a97) as i32;
            if thr <= r16(i_var2.wrapping_add(0x237).wrapping_add(b.wrapping_mul(2))) {
                return;
            }
            if r8(i_var2.wrapping_add(0x1e5)) & 8 != 0 {
                FUN_00406030(i_var1, r8(a97) as u32);
                i_var1 = r32(r32(i_var1
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8))
                .wrapping_add(0xa9));
                let idx = r8(a51
                    .wrapping_add(r8(a97) as i32)
                    .wrapping_add((r8(i_var1.wrapping_add(4)) as i32).wrapping_mul(0x101)))
                    as i32;
                let s = (r8(i_var1.wrapping_add(0x372).wrapping_add(idx.wrapping_mul(4))) as u32)
                    .wrapping_add(
                        r8(i_var1.wrapping_add(0x376).wrapping_add(idx.wrapping_mul(4))) as u32,
                    );
                if s != 0 {
                    return;
                }
                w8(a98, 0);
                w8(a97, 0);
                w16(i_var1.wrapping_add(0x226), 0x76);
                w32(i_var1.wrapping_add(0x22a), r32(abfc).wrapping_add(0x76));
                return;
            }
            if param_1 != DAT_009ecc34 {
                return;
            }
            _DAT_00a07be0 = 0x5e;
            _DAT_00a07be2 = 0x57;
            FUN_00450650(0x56, param_1);
            return;
        }
        let u4 = r8(a97) as i32;
        let mut s_var5 = r16(i_var2.wrapping_add(0x226).wrapping_add(u4.wrapping_mul(2)));
        'lab: {
            if DAT_00523424 != 0 {
                if s_var5 != 0 {
                    if r32(abfc) <= r32(i_var2.wrapping_add(0x22a).wrapping_add(u4.wrapping_mul(4)))
                    {
                        s_var5 = DAT_009ec5fe.wrapping_mul(2);
                        break 'lab;
                    }
                    s_var5 = 0;
                }
                w32(
                    i_var2.wrapping_add(0x22a).wrapping_add(u4.wrapping_mul(4)),
                    r32(abfc),
                );
            }
        }
        let d = DAT_009ec5fe;
        let thr = ((d >> 1) as i32).wrapping_add(d as i32) as i16;
        if s_var5 < thr {
            if r8(i_var2.wrapping_add(0x1e5)) & 0x10 != 0 {
                FUN_00405ac0(r32(a9670.wrapping_add(i_var3)), r8(a97) as u32);
                return;
            }
            if param_1 == DAT_009ecc34 {
                _DAT_00a07be2 = 0x57;
                _DAT_00a07be0 = (r8(a97) as i32).wrapping_add(0x5c) as i16;
                FUN_00450650(0x56, param_1);
                return;
            }
        }
    }
}
