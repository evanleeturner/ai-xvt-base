// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523430: i32;
    static mut DAT_009e9670: u8;
    static mut DAT_009ec458: u8;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_00550a08: u8;
    static mut DAT_00550a0e: u8;
    static mut DAT_00550a12: u8;
    static mut DAT_00550a16: u8;
    static mut DAT_00550a26: u8;
    static mut DAT_00550a28: u8;
    static mut DAT_00550a2a: u8;
    static mut DAT_00550888: u8;
    static mut DAT_0055088c: u8;
    static mut DAT_0055088e: u8;
    static mut DAT_00550892: u8;
    static mut DAT_00550896: u8;
    static mut DAT_0055089a: u8;
    static mut DAT_0055089e: u8;
    static mut DAT_005508a2: u8;
    static mut DAT_005508a6: u8;
    static mut DAT_005508a8: u8;
    static mut DAT_005508aa: u8;
    static mut DAT_005508ac: u8;
    static mut DAT_005508ae: u8;
    static mut DAT_005508b0: u8;
    static mut DAT_005508b2: u8;
    static mut DAT_005508b4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, n: u32, args: *const u32);
}

unsafe fn rd32(p: *const u8) -> i32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn rd16(p: *const u8) -> i16 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<i16>().read_unaligned() }
}

unsafe fn axis(iv10: *mut u8, field: usize, refp: *const u8, gainp: *const u8, iv4: i32) {
    // SAFETY: all pointers are the game's own memory, accessed as the original does.
    unsafe {
        let a = rd16(iv10.byte_add(field));
        let b = rd16(refp);
        let mut v5: i32 = a.wrapping_sub(b) as i32;
        let t32 = rd32(gainp);
        let reset = if t32 < 1 { t32 < 0 && 0 < v5 } else { v5 < 0 };
        if reset {
            let r = rd16(refp);
            iv10.byte_add(field).cast::<i16>().write_unaligned(r);
            v5 = 0;
        }
        let mut v9 = v5;
        if v9 < 0 {
            v9 = v9.wrapping_neg();
        }
        if iv4 < v9 {
            let r = rd16(refp);
            v5 = (r as i32).wrapping_add(v5);
            let cur = rd16(iv10.byte_add(field));
            let mut d = (cur as i32).wrapping_sub(v5);
            if d < 0 {
                d = d.wrapping_neg();
            }
            let lim = (iv4 as u32).wrapping_mul(8);
            if lim.wrapping_sub(d as u32) != 0 && d <= lim as i32 {
                iv10.byte_add(field)
                    .cast::<i16>()
                    .write_unaligned(v5 as i16);
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00418b70() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read and written at the offsets the original uses.
    unsafe {
        let mut i11: usize = 0;
        if DAT_00523430 == 0 {
            return;
        }
        let mut local_c: *mut u8 = &raw mut DAT_009e9670;
        let mut local_1c: i32 = 0;
        let o08 = &raw mut DAT_00550a08 as *mut u8;
        loop {
            'next: {
                o08.byte_add(i11).cast::<i32>().write_unaligned(0);

                if local_c.byte_add(0x10).read() == 0 {
                    break 'next;
                }
                let t = rd32(local_c);
                if t == -1 {
                    break 'next;
                }
                let iv10: *mut u8 = DAT_009a1fe8.byte_add((t as u32).wrapping_mul(0x23) as usize);
                if iv10.byte_add(3).read() == 0 {
                    break 'next;
                }
                if rd32(iv10.byte_add(0x1f)) == 0 {
                    break 'next;
                }
                if rd32((&raw mut DAT_00550888 as *mut u8).byte_add(i11)) == 0 {
                    break 'next;
                }
                if DAT_009ecc34 == local_1c {
                    break 'next;
                }
                let tu16 = (rd16((&raw mut DAT_0055088c as *mut u8).byte_add(i11)) as u16) as u32;
                if tu16 != rd32(local_c.byte_add(4)) as u32 {
                    break 'next;
                }

                let v = rd16(iv10.byte_add(0x14));
                (&raw mut DAT_00550a26 as *mut u8)
                    .byte_add(i11)
                    .cast::<i16>()
                    .write_unaligned(v);
                let v = rd16(iv10.byte_add(0x12));
                (&raw mut DAT_00550a28 as *mut u8)
                    .byte_add(i11)
                    .cast::<i16>()
                    .write_unaligned(v);
                let v = rd16(iv10.byte_add(0x10));
                (&raw mut DAT_00550a2a as *mut u8)
                    .byte_add(i11)
                    .cast::<i16>()
                    .write_unaligned(v);
                let v = rd32(iv10.byte_add(4));
                (&raw mut DAT_00550a0e as *mut u8)
                    .byte_add(i11)
                    .cast::<i32>()
                    .write_unaligned(v);
                let mut local_18 = rd32((&raw mut DAT_0055088e as *mut u8).byte_add(i11));
                let v = rd32(iv10.byte_add(8));
                (&raw mut DAT_00550a12 as *mut u8)
                    .byte_add(i11)
                    .cast::<i32>()
                    .write_unaligned(v);
                let mut local_14 = rd32((&raw mut DAT_00550892 as *mut u8).byte_add(i11));
                let v = rd32(iv10.byte_add(0xc));
                (&raw mut DAT_00550a16 as *mut u8)
                    .byte_add(i11)
                    .cast::<i32>()
                    .write_unaligned(v);
                let mut local_10 = rd32((&raw mut DAT_00550896 as *mut u8).byte_add(i11));
                o08.byte_add(i11).cast::<i32>().write_unaligned(1);

                let p2 = rd32(iv10.byte_add(0x1f)) as u32 as usize as *const u8;
                let iv5 = rd32(p2.byte_add(2));
                let t32 = rd32((&raw mut DAT_005508b4 as *mut u8).byte_add(i11));
                let mut iv4: i32 = iv5.wrapping_sub(t32);
                if -1 < iv4 {
                    let mut iv9: i32 = 0;
                    let t32 = rd32((&raw mut DAT_005508b4 as *mut u8).byte_add(i11));
                    let tu = rd16((&raw mut DAT_005508b2 as *mut u8).byte_add(i11)) as u16;
                    if iv4 != 0 && t32 <= iv5 && tu != 0 {
                        let a = ((((tu as u32).wrapping_mul(0x1234).wrapping_add(0x80)) as i32)
                            >> 8) as u32;
                        iv9 = (a.wrapping_mul(iv4 as u32) as i32) / 0xec;
                        let g = rd16((&raw mut DAT_005508ac as *mut u8).byte_add(i11)) as i32;
                        let l = (g as i64).wrapping_mul(iv9 as i64);
                        local_18 = local_18.wrapping_add((l >> 15) as i32);
                        let g = rd16((&raw mut DAT_005508ae as *mut u8).byte_add(i11)) as i32;
                        let l = (g as i64).wrapping_mul(iv9 as i64);
                        local_14 = local_14.wrapping_add((l >> 15) as i32);
                        let g = rd16((&raw mut DAT_005508b0 as *mut u8).byte_add(i11)) as i32;
                        let l = (g as i64).wrapping_mul(iv9 as i64);
                        local_10 = local_10.wrapping_add((l >> 15) as i32);
                    }
                    let iv5 = rd32(iv10.byte_add(4)).wrapping_sub(local_18);
                    let iv6 = rd32(iv10.byte_add(8)).wrapping_sub(local_14);
                    let iv7 = rd32(iv10.byte_add(0xc)).wrapping_sub(local_10);
                    let iv8 = FUN_0041f410(iv5, iv6, iv7);
                    if ((iv9 as u32).wrapping_mul(0x20) as i32) < iv8 || iv8 == 0 {
                        iv9 = 0x4000;
                    } else {
                        let divisor = (iv9 as u32).wrapping_mul(0x20);
                        let dividend = (iv8 as u32).wrapping_shl(0xe);
                        if divisor == 0 || (dividend as i32 == i32::MIN && divisor as i32 == -1) {
                            RaiseException(0xC0000094, 0, 0, core::ptr::null());
                        }
                        let q: i32;
                        core::arch::asm!(
                            "idiv {d}",
                            d = in(reg) divisor as i32,
                            inout("eax") dividend as i32 => q,
                            inout("edx") ((dividend as i32) >> 31) => _,
                        );
                        iv9 = q;
                    }
                    let l1 = (iv9 as i64).wrapping_mul(iv5 as i64);
                    let l2 = (iv9 as i64).wrapping_mul(iv6 as i64);
                    let l3 = (iv9 as i64).wrapping_mul(iv7 as i64);
                    iv10.byte_add(4)
                        .cast::<i32>()
                        .write_unaligned(local_18.wrapping_add((l1 >> 15) as i32));
                    iv10.byte_add(8)
                        .cast::<i32>()
                        .write_unaligned(local_14.wrapping_add((l2 >> 15) as i32));
                    iv10.byte_add(0xc)
                        .cast::<i32>()
                        .write_unaligned(local_10.wrapping_add((l3 >> 15) as i32));

                    // axis 0x14 uses the pre-scaled iVar4 only after it is rescaled
                    iv4 = ((iv4 as u32).wrapping_mul(0x1800) as i32) / 0xec;
                    axis(
                        iv10,
                        0x14,
                        (&raw mut DAT_005508a6 as *mut u8).byte_add(i11),
                        (&raw mut DAT_0055089a as *mut u8).byte_add(i11),
                        iv4,
                    );
                    axis(
                        iv10,
                        0x12,
                        (&raw mut DAT_005508a8 as *mut u8).byte_add(i11),
                        (&raw mut DAT_0055089e as *mut u8).byte_add(i11),
                        iv4,
                    );
                    axis(
                        iv10,
                        0x10,
                        (&raw mut DAT_005508aa as *mut u8).byte_add(i11),
                        (&raw mut DAT_005508a2 as *mut u8).byte_add(i11),
                        iv4,
                    );
                }
            }
            i11 = i11.wrapping_add(0x30);
            local_c = local_c.byte_add(0x5bd);
            local_1c = local_1c.wrapping_add(1);
            if !(local_c < (&raw mut DAT_009ec458)) {
                break;
            }
        }
    }
}
