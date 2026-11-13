// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ed664: u16;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009e9a5f: i8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9a5b: i8;
    static mut DAT_009e9a63: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046abf0(param_1: u32, param_2: u32) -> u32;
    fn FUN_00427a60(param_1: i16, param_2: i16, param_3: i16, param_4: i16, param_5: i32);
    fn FUN_00472400(param_1: i32);
}

fn q(a: i16, b: i32) -> u32 {
    let p = (a as i32 as i64).wrapping_mul(b as i64);
    ((p as u32) >> 0xf) | ((((p as u64) >> 0x20) as i32).wrapping_shl(0x11) as u32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041ff70(param_1: u16) {
    // SAFETY: the globals and callees are the game's own; the pointers
    // computed from them address the game's records, accessed as the original does.
    unsafe {
        DAT_009ed664 = param_1;
        let i_var11: i32 = (param_1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8 as u32) as i32;
        let base = |addr: u32| -> i32 {
            let off = DAT_009ecc34.wrapping_mul(0x5bd) as u32;
            (addr.wrapping_add(off) as *const i32).read_unaligned()
        };
        let rec = |off: u32| -> u32 { (i_var11 as u32).wrapping_add(off) };
        let i_var12 = base((&raw mut DAT_009e9a5f) as u32)
            .wrapping_sub((rec(8) as *const i32).read_unaligned());
        let i_var9 = base((&raw mut DAT_009e9a5b) as u32)
            .wrapping_sub((rec(4) as *const i32).read_unaligned());
        let i_var10 = base((&raw mut DAT_009e9a63) as u32)
            .wrapping_sub((rec(0xc) as *const i32).read_unaligned());
        let p = (rec(0x1f) as *const i32).read_unaligned() as u32;
        let rd = |off: u32| -> i16 { (p.wrapping_add(off) as *const i16).read_unaligned() };
        let l2 = q(rd(0x99), i_var9);
        let l3 = q(rd(0x9b), i_var12);
        let l4 = q(rd(0x9d), i_var10);
        let l5 = q(rd(0x9f), i_var9);
        let l6 = q(rd(0xa1), i_var12);
        let l7 = q(rd(0xa3), i_var10);
        let s_var1 = (rec(0x14) as *const i16).read_unaligned();
        let u_var8 = FUN_0046abf0(
            l7.wrapping_add(l5).wrapping_add(l6),
            l4.wrapping_add(l2).wrapping_add(l3),
        );
        (rec(0x14) as *mut i16).write_unaligned((u_var8 as i16).wrapping_add(s_var1));
        let cur = (rec(0x14) as *const i16).read_unaligned();
        (rec(0x14) as *mut i16).write_unaligned(cur.wrapping_add(-0x4000));
        let p2 = (rec(0x1f) as *const i32).read_unaligned() as u32;
        (p2.wrapping_add(0x92) as *mut u8).write(1);
        FUN_00427a60(
            (rec(0x14) as *const i16).read_unaligned(),
            (rec(0x12) as *const i16).read_unaligned(),
            (rec(0x10) as *const i16).read_unaligned(),
            0,
            i_var11,
        );
        FUN_00472400(i_var11);
        (rec(0x14) as *mut i16).write_unaligned(s_var1);
        let p3 = (rec(0x1f) as *const i32).read_unaligned() as u32;
        (p3.wrapping_add(0x92) as *mut u8).write(1);
    }
}
