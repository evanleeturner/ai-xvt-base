// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_009a8c06: u16;
    static mut DAT_009cd274: i32;
    static mut DAT_009a8e24: i32;
    static mut DAT_009a1fe8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041f3d0(a: u32, b: u32, c: u32) -> u32;
    fn FUN_00425d10() -> u16;
    fn FUN_00427bd0(a: i16, b: i16, c: i32);
    fn FUN_00427d30(a: i16, b: i32, c: i32);
}

#[inline(always)]
fn p(a: u32) -> *mut u8 {
    a as usize as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00459850() {
    // SAFETY: all globals and callees are the game's own memory; the object
    // pointers are computed from the game's table exactly as the original does.
    unsafe {
        let idx = (DAT_009ecc34 as u32).wrapping_mul(0x5bd);
        let mut iv12: i32 = (&raw mut DAT_009e9670)
            .cast::<u8>()
            .byte_add(idx as usize)
            .cast::<i32>()
            .read_unaligned();
        if iv12 == -1 {
            return;
        }
        let u9 = DAT_009a8c06 as u32;
        DAT_009a8c06 = DAT_009a8c06.wrapping_add(1);
        if DAT_009a8c06 as i32 == DAT_009cd274 {
            DAT_009a8c06 = DAT_009a8e24 as u16;
        }
        let iv5: u32 = u9.wrapping_mul(0x23);
        let iv10: u32 = (DAT_009a1fe8 as u32).wrapping_add(iv5);
        let o12: u32 = (iv12 as u32).wrapping_mul(0x23);
        iv12 = o12 as i32;
        let iv8: u32 = (DAT_009a1fe8 as u32).wrapping_add(o12);

        let mut d = [0u32; 3];
        let mut k = 0;
        while k < 3 {
            let off = 4 + 4 * k as u32;
            let a = p(iv10.wrapping_add(off)).cast::<i32>().read_unaligned();
            let b = p(iv8.wrapping_add(off)).cast::<i32>().read_unaligned();
            let mut v = (a as u32).wrapping_sub(b as u32);
            if (v as i32) < 0 {
                v = 0u32.wrapping_sub(v);
            }
            d[k] = v;
            k += 1;
        }
        let r = FUN_0041f3d0(d[0], d[1], d[2]);
        if 0x800 < r {
            let b6 = FUN_00425d10() as u8;
            p((DAT_009a1fe8 as u32).wrapping_add(3).wrapping_add(iv5))
                .write(((b6 & 3) as u32 + 0x6e) as u8);
            p((DAT_009a1fe8 as u32).wrapping_add(2).wrapping_add(iv5)).write(0xb);
            let obj = p((DAT_009a1fe8 as u32).wrapping_add(0x1f).wrapping_add(iv5))
                .cast::<u32>()
                .read_unaligned();
            p(obj).write(3);
            p((DAT_009a1fe8 as u32).wrapping_add(0x16).wrapping_add(iv5)).write(0xff);

            let iv8b: u32 = (DAT_009a1fe8 as u32).wrapping_add(iv12 as u32);
            let obj = p((DAT_009a1fe8 as u32)
                .wrapping_add(0x1f)
                .wrapping_add(iv12 as u32))
            .cast::<u32>()
            .read_unaligned();
            if p(obj).byte_add(0x92).read() != 0 {
                let sa = p(iv8b.wrapping_add(0x12)).cast::<i16>().read_unaligned();
                let sb = p(iv8b.wrapping_add(0x10)).cast::<i16>().read_unaligned();
                FUN_00427bd0(sa, sb, iv8b as i32);
                let sc = p((DAT_009a1fe8 as u32)
                    .wrapping_add(iv12 as u32)
                    .wrapping_add(0x14))
                .cast::<i16>()
                .read_unaligned();
                FUN_00427d30(
                    sc,
                    0,
                    (DAT_009a1fe8 as u32).wrapping_add(iv12 as u32) as i32,
                );
            }

            let u7 = FUN_00425d10();
            let obj = p((DAT_009a1fe8 as u32)
                .wrapping_add(0x1f)
                .wrapping_add(iv12 as u32))
            .cast::<u32>()
            .read_unaligned();
            let m = (((u7 & 0x3ff) as i32) - 0x200) as i16 as i32;
            let sa = p(obj).byte_add(0x99).cast::<i16>().read_unaligned();
            let local_20 = ((m.wrapping_mul(sa as i32) as u32) >> 15) as i16;
            let sa = p(obj).byte_add(0x9b).cast::<i16>().read_unaligned();
            let local_1c = ((m.wrapping_mul(sa as i32) as u32) >> 15) as i16;
            let sa = p(obj).byte_add(0x9d).cast::<i16>().read_unaligned();
            let local_18 = ((m.wrapping_mul(sa as i32) as u32) >> 15) as i16;

            let u7 = FUN_00425d10();
            let b12: u32 = (iv12 as u32).wrapping_add(DAT_009a1fe8 as u32);
            let m = (((u7 & 0x3ff) as i32) - 0x200) as i16 as i32;
            let obj = p(b12.wrapping_add(0x1f)).cast::<u32>().read_unaligned();
            let sa = p(obj).byte_add(0x9f).cast::<i16>().read_unaligned();
            let local_14 = ((m.wrapping_mul(sa as i32) as u32) >> 15) as i16;
            let obj = p(b12.wrapping_add(0x1f)).cast::<u32>().read_unaligned();
            let sa = p(obj).byte_add(0xa1).cast::<i16>().read_unaligned();
            let local_10 = ((m.wrapping_mul(sa as i32) as u32) >> 15) as i16;
            let obj = p(b12.wrapping_add(0x1f)).cast::<u32>().read_unaligned();
            let sa = p(obj).byte_add(0xa3).cast::<i16>().read_unaligned();
            let local_c = ((m.wrapping_mul(sa as i32) as u32) >> 15) as i16;

            let obj = p(b12.wrapping_add(0x1f)).cast::<u32>().read_unaligned();
            let s2 = p(obj).byte_add(0x95).cast::<i16>().read_unaligned();
            let i10 = p(b12.wrapping_add(0xc)).cast::<i32>().read_unaligned();
            let s3 = p(obj).byte_add(0x97).cast::<i16>().read_unaligned();
            let i4 = p(b12.wrapping_add(8)).cast::<i32>().read_unaligned();
            let sb = p(obj).byte_add(0x93).cast::<i16>().read_unaligned();
            let ia = p(b12.wrapping_add(4)).cast::<i32>().read_unaligned();

            let ib = (ia as u32)
                .wrapping_add(local_20.wrapping_add(local_14).wrapping_add(sb >> 4) as i32 as u32);
            p((DAT_009a1fe8 as u32).wrapping_add(4).wrapping_add(iv5))
                .cast::<u32>()
                .write_unaligned(ib);

            let ib = (local_1c.wrapping_add(local_10).wrapping_add(s2 >> 4) as i32 as u32)
                .wrapping_add(i4 as u32);
            p((DAT_009a1fe8 as u32).wrapping_add(8).wrapping_add(iv5))
                .cast::<u32>()
                .write_unaligned(ib);

            let ib = (i10 as u32)
                .wrapping_add(local_18.wrapping_add(local_c).wrapping_add(s3 >> 4) as i32 as u32);
            p((DAT_009a1fe8 as u32).wrapping_add(0xc).wrapping_add(iv5))
                .cast::<u32>()
                .write_unaligned(ib);

            p((DAT_009a1fe8 as u32).wrapping_add(0x19).wrapping_add(iv5)).write(2);
        }
    }
}
