// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d6950: u16;
    static mut DAT_009d6944: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d91ea: i16;
    static mut DAT_00a08104: i32;
    static mut DAT_00a07bd0: i32;
    static mut DAT_009a8d80: i32;
    static mut DAT_009a8d60: i32;
    static mut DAT_009a8d8c: i32;
    static mut DAT_009d12e8: i32;
    static mut DAT_009d1264: i32;
    static mut DAT_009d1154: i32;
    static mut DAT_009a8e20: i32;
    static mut DAT_009a8e1c: i32;
    static mut DAT_009a8e28: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00427d30(a: i16, b: i32, c: i32);
    fn FUN_004ae080(a: u32) -> i64;
    fn FUN_004ae120(a: u32) -> i64;
    fn FUN_004adff0(a: u32) -> i64;
    fn FUN_004ae020(a: u32) -> i64;
    fn FUN_004ae160(a: u32) -> i64;
}

unsafe fn put8(a: u32, v: u8) {
    // SAFETY: address is the game's own memory, written where the original does.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn put16(a: u32, v: u16) {
    // SAFETY: address is the game's own memory, written where the original does.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
unsafe fn put32(a: u32, v: u32) {
    // SAFETY: address is the game's own memory, written where the original does.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}
unsafe fn get8(a: u32) -> u8 {
    // SAFETY: address is the game's own memory, read where the original does.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn get16(a: u32) -> u16 {
    // SAFETY: address is the game's own memory, read where the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
unsafe fn get32(a: u32) -> u32 {
    // SAFETY: address is the game's own memory, read where the original does.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

fn fx(a: i32, b: i32) -> u32 {
    let pr = (a as i64).wrapping_mul(b as i64);
    ((pr as u32) >> 0xf) | (((pr as u64 >> 0x20) as u32) << 0x11)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042b550() {
    let a124: [u16; 14] = [
        0, 0x62, 99, 0x62, 99, 0x62, 99, 0x62, 99, 0x62, 99, 0x62, 99, 0,
    ];
    let d0: [u16; 16] = [
        0, 0x4000, 0x4000, 0, 0, 0x4000, 0x4000, 0x8000, 0x8000, 0x4000, 0x4000, 0x4000, 0x4000, 0,
        0, 0,
    ];
    let ec: [u16; 14] = [
        0, 0, 0, 0, 0x4000, 0xc000, 0xc000, 0, 0x8000, 0x8000, 0x8000, 0x4000, 0x4000, 0,
    ];
    let a108: [u16; 14] = [0, 0, 0, 0, 0, 0, 0x8000, 0, 0, 0, 0x4000, 0, 0x4000, 0];

    // SAFETY: the globals and callees are the game's own; all addresses
    // written are the game's own structures, accessed as the original does.
    unsafe {
        DAT_009d6950 = 0;
        DAT_009d6944 = 0;
        let mut local_1c: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_14: i32 = 0;
        let mut local_6: u16 = 1;
        loop {
            let u_var4 = local_6 as u32;
            let u_var1 = a124[u_var4 as usize];
            let i_var3 = u_var4.wrapping_mul(0x23);
            let e = (DAT_009a1fe8 as u32).wrapping_add(i_var3);

            put8(e.wrapping_add(3), u_var1 as u8);
            put16(e, 1);
            let p = get32(e.wrapping_add(0x1f));
            put8(p, 6);
            put8(e.wrapping_add(2), 0xe);
            let p = get32(e.wrapping_add(0x1f));
            put16(p.wrapping_add(0x77), 0);
            let p = get32(e.wrapping_add(0x1f));
            put16(p.wrapping_add(0x79), 0);
            let p = get32(e.wrapping_add(0x1f));
            put16(p.wrapping_add(0x7b), 0);
            let p = get32(e.wrapping_add(0x1f));
            put32(p.wrapping_add(0x7d), 0x7fff);
            let p = get32(e.wrapping_add(0x1f));
            put16(p.wrapping_add(0x81), 0);
            let p = get32(e.wrapping_add(0x1f));
            put16(p.wrapping_add(0x83), 0);
            let p = get32(e.wrapping_add(0x1f));
            put16(p.wrapping_add(0x85), 0);
            let p = get32(e.wrapping_add(0x1f));
            put8(p.wrapping_add(0x87), 0);
            let p = get32(e.wrapping_add(0x1f));
            put8(p.wrapping_add(0x88), 0);
            let p = get32(e.wrapping_add(0x1f));
            put8(p.wrapping_add(0x92), 1);
            let i_var6 = get32(e.wrapping_add(0x1f));
            put8(i_var6.wrapping_add(0x8b), get8(i_var6.wrapping_add(0x92)));
            put8(e.wrapping_add(0x16), 1);
            DAT_009d91ea = 5;
            DAT_00a08104 = u_var4.wrapping_mul(0x462).wrapping_add(DAT_00a07bd0 as u32) as i32;
            let p = get32(e.wrapping_add(0x1f));
            put32(p.wrapping_add(0xa9), DAT_00a08104 as u32);

            let mut u_var7: u16 = 0;
            loop {
                let u_var8 = u_var7.wrapping_add(1);
                put8(
                    (u_var7 as u32)
                        .wrapping_add(DAT_009a1fe8 as u32)
                        .wrapping_add(0x19)
                        .wrapping_add(i_var3),
                    0,
                );
                u_var7 = u_var8;
                if !(u_var8 < 2) {
                    break;
                }
            }

            let d = DAT_00a08104 as u32;
            u_var7 = 0;
            loop {
                let u_var8 = u_var7.wrapping_add(1);
                let u_var5 = u_var7 as u32;
                put8(u_var5.wrapping_add(0x2d8).wrapping_add(d), 0);
                put8(u_var5.wrapping_add(0x30a).wrapping_add(d), 0);
                put8(u_var5.wrapping_add(0x33c).wrapping_add(d), 0xff);
                u_var7 = u_var8;
                if !(u_var8 < 0x32) {
                    break;
                }
            }

            put32(d.wrapping_add(0xcb), 0x7fff);
            put32(d.wrapping_add(199), 0x7fff);
            put32(d.wrapping_add(0xc3), 0);
            put16(d.wrapping_add(0xd1), 0);
            put8(d.wrapping_add(0x1e9), 0);
            put8(d.wrapping_add(0x1ec), 0);
            put8(d.wrapping_add(0x1ea), 0);
            put8(d.wrapping_add(0x1eb), 0);
            put8(d.wrapping_add(0x29), 0);

            u_var7 = 0;
            loop {
                let u_var8 = u_var7.wrapping_add(1);
                put32(
                    d.wrapping_add(0x15)
                        .wrapping_add((u_var7 as u32).wrapping_mul(4)),
                    0,
                );
                u_var7 = u_var8;
                if !(u_var8 < 5) {
                    break;
                }
            }

            put16(d.wrapping_add(0x1e3), 1);
            let u_var2 = get16(d.wrapping_add(0x1e3));
            put16(d.wrapping_add(0x1e5), u_var2);
            put8(d.wrapping_add(0x21a), 0);
            let u_var2 = a108[u_var4 as usize];
            put32(d.wrapping_add(0x211), 0x7fff);
            put32(d.wrapping_add(0x215), 0);
            put16(e.wrapping_add(0x14), u_var2);
            let u_var2 = d0[u_var4 as usize];
            put16(e.wrapping_add(0x10), ec[u_var4 as usize]);
            put16(e.wrapping_add(0x12), u_var2);

            let a12 = get16(e.wrapping_add(0x12));
            let a10 = get16(e.wrapping_add(0x10));
            FUN_00427bd0(a12 as i16, a10 as i16, e as i32);
            let a14 = get16(e.wrapping_add(0x14));
            FUN_00427d30(a14 as i16, 0, e as i32);

            let mut local_c: i16 = 0;
            let mut local_a: i16 = 0;
            let mut local_8: i16 = 0;
            let lvar9 = FUN_004ae080(u_var1 as u32);
            let local_e: i16 = (lvar9 as i16).wrapping_neg();
            if u_var1 == 0x62 {
                local_c = 0;
                let l9 = FUN_004ae120(0x62);
                local_a = l9 as i16;
                local_8 = 0;
            } else if u_var1 == 99 {
                local_c = 0;
                let l9 = FUN_004adff0(99);
                let l10 = FUN_004ae120(99);
                local_a = (l10 as i16).wrapping_add(l9 as i16);
                let l9 = FUN_004ae020(99);
                let l10 = FUN_004ae160(99);
                local_8 = (l10 as i16).wrapping_add(l9 as i16);
            }

            let i6 = local_e as i32;
            let local_68 = fx(i6, DAT_009d12e8);
            let local_60 = fx(i6, DAT_009d1264);
            let local_54 = fx(i6, DAT_009d1154);

            put32(
                e.wrapping_add(4),
                (local_1c as u32).wrapping_add(local_68.wrapping_mul(0xFFFFFFFE)),
            );
            put32(
                e.wrapping_add(8),
                (local_18 as u32).wrapping_add(local_60.wrapping_mul(0xFFFFFFFE)),
            );
            put32(
                e.wrapping_add(0xc),
                (local_14 as u32).wrapping_add(local_54.wrapping_mul(0xFFFFFFFE)),
            );

            let i6 = local_c as i32;
            let local_44 = fx(i6, DAT_009a8d80);
            let local_38 = fx(i6, DAT_009a8d60);
            let local_34 = fx(i6, DAT_009a8d8c);
            let i6 = local_a as i32;
            let local_40 = fx(i6, DAT_009d12e8);
            let local_3c = fx(i6, DAT_009d1264);
            let local_30 = fx(i6, DAT_009d1154);
            let i6 = local_8 as i32;
            let local_2c = fx(i6, DAT_009a8e20);
            let local_28 = fx(i6, DAT_009a8e1c);
            let local_20 = fx(i6, DAT_009a8e28);

            local_1c = (local_1c as u32).wrapping_add(
                local_44
                    .wrapping_add(local_40)
                    .wrapping_add(local_2c)
                    .wrapping_mul(2),
            ) as i32;
            local_18 = (local_18 as u32).wrapping_add(
                local_3c
                    .wrapping_add(local_38)
                    .wrapping_add(local_28)
                    .wrapping_mul(2),
            ) as i32;
            local_14 = (local_14 as u32).wrapping_add(
                local_34
                    .wrapping_add(local_30)
                    .wrapping_add(local_20)
                    .wrapping_mul(2),
            ) as i32;

            let v = get32(e.wrapping_add(4));
            let p = get32(e.wrapping_add(0x1f));
            put32(p.wrapping_add(6), v);
            local_6 = local_6.wrapping_add(1);
            let v = get32(e.wrapping_add(8));
            let p = get32(e.wrapping_add(0x1f));
            put32(p.wrapping_add(10), v);
            let v = get32(e.wrapping_add(0xc));
            let p = get32(e.wrapping_add(0x1f));
            put32(p.wrapping_add(0xe), v);

            if !(local_6 < 0xd) {
                break;
            }
        }
    }
}
