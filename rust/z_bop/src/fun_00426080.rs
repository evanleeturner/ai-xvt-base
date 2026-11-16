// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00488: i32;
    static mut DAT_00a00490: i32;
    static mut DAT_00a00480: i32;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00a004b8: i32;
    static mut DAT_00a004cc: i32;
    static mut DAT_00a004b0: i32;
    static mut DAT_00a004a4: i32;
    static mut DAT_00a004a8: i32;
    static mut DAT_009d80d0: [i32; 16];
    static mut DAT_009a8dd0: [i32; 16];
    static mut DAT_009d1270: [i32; 16];
    static mut DAT_009fe740: [i32; 16];
    static mut DAT_009e9600: [i32; 16];
    static mut DAT_009ec480: [i32; 16];
    static mut DAT_00a08250: [i32; 16];
    static mut DAT_00a004e0: [i32; 16];
    static mut DAT_00a07c80: [i32; 16];
    static mut DAT_009a7810: [i32; 128];
    static mut DAT_009a7400: [i32; 128];
    static mut DAT_009a7600: [i32; 128];
    static mut DAT_00a080fc: i8;
    static mut DAT_00523530: [u8; 0];
    static mut DAT_00523570: u16;
    static mut DAT_00523574: u16;
    static mut DAT_00523578: u16;
    static mut DAT_0052357c: u16;
    static mut DAT_00523580: u16;
    static mut DAT_00523584: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046abf0(a: i32, b: i32) -> i32;
    fn FUN_00426860(a: i32, b: i32, c: u32, d: i16, e: u32);
}

macro_rules! m {
    ($n:ident) => {
        (&raw mut $n).cast::<i32>()
    };
}

macro_rules! c {
    ($n:ident) => {
        (&raw const $n).cast::<i32>()
    };
}

unsafe fn wr(base: *mut i32, off: u32, v: i32) {
    // SAFETY: caller passes a game array and an in-range byte offset.
    unsafe {
        base.byte_add(off as usize).write_unaligned(v);
    }
}

unsafe fn tab(i: u32) -> u8 {
    // SAFETY: reads the game's byte table at the given index.
    unsafe {
        (&raw const DAT_00523530)
            .cast::<u8>()
            .add(i as usize)
            .read()
    }
}

unsafe fn comp(
    b: u8,
    s1: *const i32,
    s2: *const i32,
    s3: *const i32,
    t1: *const i32,
    t2: *const i32,
    t3: *const i32,
) -> (u32, u32, u32, u32, u32, u32) {
    let k = (b & 7) as usize;
    let o = ((b & 0x70) >> 2) as usize;
    // SAFETY: arrays are the game's; indices come from masked bits.
    unsafe {
        let mut p1 = s1.add(k).read() as u32;
        let mut p2 = s2.add(k).read() as u32;
        let mut p3 = s3.add(k).read() as u32;
        if (b & 8) != 0 {
            p1 = p1.wrapping_neg();
            p2 = p2.wrapping_neg();
            p3 = p3.wrapping_neg();
        }
        let mut q1 = t1.byte_add(o).read_unaligned() as u32;
        let mut q2 = t2.byte_add(o).read_unaligned() as u32;
        let mut q3 = t3.byte_add(o).read_unaligned() as u32;
        if (b & 0x80) != 0 {
            q1 = q1.wrapping_neg();
            q2 = q2.wrapping_neg();
            q3 = q3.wrapping_neg();
        }
        (p1, p2, p3, q1, q2, q3)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00426080() {
    // SAFETY: all globals and callees are the game's own memory and code.
    unsafe {
        let i13: u32 = DAT_00a00488 as u32;
        let mut i9: u32 = 0;
        let mut i12: u32 = 0;
        let mut i8: u32 = 0;
        let mut l20: u32 = 0;
        let mut i11: u32 = 0;
        let mut l10: u32 = 0;
        let mut l1c: u32 = 0;
        let mut l18: u32 = 0;
        let mut l14: u32 = 0;
        let mut i7: u32 = 0;
        loop {
            let i2 = i7.wrapping_add(4);
            wr(m!(DAT_009d80d0), i7, (i11 as i32) >> 5);
            wr(m!(DAT_009a8dd0), i7, (l14 as i32) >> 5);
            wr(m!(DAT_009d1270), i7, (l18 as i32) >> 5);
            wr(m!(DAT_009fe740), i7, (i8 as i32) >> 5);
            wr(m!(DAT_009e9600), i7, (l1c as i32) >> 5);
            wr(m!(DAT_009ec480), i7, (l10 as i32) >> 5);
            let i6 = (i12 as i32) >> 5;
            i12 = i12.wrapping_add(i13);
            wr(m!(DAT_00a08250), i7, i6);
            wr(m!(DAT_00a004e0), i7, (l20 as i32) >> 5);
            wr(m!(DAT_00a07c80), i7, (i9 as i32) >> 5);
            i9 = i9.wrapping_add(DAT_00a00490 as u32);
            l20 = l20.wrapping_add(DAT_00a00480 as u32);
            l10 = l10.wrapping_add(DAT_009fe7f4 as u32);
            l1c = l1c.wrapping_add(DAT_00a004b8 as u32);
            i8 = i8.wrapping_add(DAT_00a004cc as u32);
            l18 = l18.wrapping_add(DAT_00a004b0 as u32);
            l14 = l14.wrapping_add(DAT_00a004a4 as u32);
            i11 = i11.wrapping_add(DAT_00a004a8 as u32);
            i7 = i2;
            if !((i2 as i32) < 0x40) {
                break;
            }
        }
        i7 = 0;
        let mut l_c: u32 = (DAT_009fe7f4 as u32)
            .wrapping_add(DAT_00a00490 as u32)
            .wrapping_add(DAT_00a004b0 as u32)
            .wrapping_mul(0xfffffffe);
        let mut l_8: u32 = (DAT_00a004b8 as u32)
            .wrapping_add(DAT_00a00480 as u32)
            .wrapping_add(DAT_00a004a4 as u32)
            .wrapping_mul(0xfffffffe);
        l10 = 5;
        let mut l_4: u32 = (DAT_00a004a8 as u32)
            .wrapping_add(i13)
            .wrapping_add(DAT_00a004cc as u32)
            .wrapping_mul(0xfffffffe);
        loop {
            l1c = l_4;
            l18 = l_8;
            l14 = l_c;
            l20 = 5;
            loop {
                let mut i2: u32 = 5;
                let mut i8b = i7;
                let mut i9b = l18;
                let mut i11b = l1c;
                let mut i12b = l14;
                loop {
                    i7 = i8b.wrapping_add(4);
                    let i6 = (i11b as i32) >> 7;
                    i11b = i11b.wrapping_add(i13);
                    wr(m!(DAT_009a7810), i8b, i6);
                    wr(m!(DAT_009a7400), i8b, (i9b as i32) >> 7);
                    wr(m!(DAT_009a7600), i8b, (i12b as i32) >> 7);
                    i12b = i12b.wrapping_add(DAT_00a00490 as u32);
                    i9b = i9b.wrapping_add(DAT_00a00480 as u32);
                    i2 = i2.wrapping_sub(1);
                    i8b = i7;
                    if i2 == 0 {
                        break;
                    }
                }
                l1c = l1c.wrapping_add(DAT_00a004cc as u32);
                l18 = l18.wrapping_add(DAT_00a004b8 as u32);
                l14 = l14.wrapping_add(DAT_009fe7f4 as u32);
                l20 = l20.wrapping_sub(1);
                if l20 == 0 {
                    break;
                }
            }
            l_c = l_c.wrapping_add(DAT_00a004b0 as u32);
            l_8 = l_8.wrapping_add(DAT_00a004a4 as u32);
            l_4 = l_4.wrapping_add(DAT_00a004a8 as u32);
            l10 = l10.wrapping_sub(1);
            if l10 == 0 {
                break;
            }
        }
        if DAT_00a080fc == 0 {
            return;
        }

        let mut pos: u32;
        let mut cnt: u32;

        let u3 = FUN_0046abf0(DAT_00a004a4, DAT_00a004a8);
        let d3 = (u3 as i16).wrapping_neg();
        if DAT_009fe7f4 < 0 {
            pos = DAT_00523570 as u32;
            cnt = DAT_00523574 as u32;
            while cnt != 0 {
                cnt -= 1;
                let b = tab(pos);
                pos = pos.wrapping_add(1);
                let (p1, p2, p3, q1, q2, q3) = comp(
                    b,
                    c!(DAT_009d80d0),
                    c!(DAT_009a8dd0),
                    c!(DAT_009d1270),
                    c!(DAT_00a08250),
                    c!(DAT_00a004e0),
                    c!(DAT_00a07c80),
                );
                let z = p3.wrapping_add(q3).wrapping_sub((DAT_009fe7f4 >> 2) as u32);
                if (z as i32) >= 0 {
                    FUN_00426860(
                        p1.wrapping_add(q1).wrapping_sub((DAT_00a004cc >> 2) as u32) as i32,
                        p2.wrapping_add(q2).wrapping_sub((DAT_00a004b8 >> 2) as u32) as i32,
                        z,
                        d3,
                        pos,
                    );
                }
            }
        } else {
            pos = 0;
            cnt = DAT_00523570 as u32;
            while cnt != 0 {
                cnt -= 1;
                let b = tab(pos);
                pos = pos.wrapping_add(1);
                let (p1, p2, p3, q1, q2, q3) = comp(
                    b,
                    c!(DAT_009d80d0),
                    c!(DAT_009a8dd0),
                    c!(DAT_009d1270),
                    c!(DAT_00a08250),
                    c!(DAT_00a004e0),
                    c!(DAT_00a07c80),
                );
                let z = p3.wrapping_add(q3).wrapping_add((DAT_009fe7f4 >> 2) as u32);
                if (z as i32) >= 0 {
                    FUN_00426860(
                        p1.wrapping_add(q1).wrapping_add((DAT_00a004cc >> 2) as u32) as i32,
                        p2.wrapping_add(q2).wrapping_add((DAT_00a004b8 >> 2) as u32) as i32,
                        z,
                        d3,
                        pos,
                    );
                }
            }
            pos = pos.wrapping_add(DAT_00523574 as u32);
        }

        let u3 = FUN_0046abf0(DAT_00a004b8, DAT_00a004cc);
        let d3 = (u3 as i16).wrapping_neg();
        if DAT_00a004b0 < 0 {
            pos = pos.wrapping_add(DAT_00523580 as u32);
            cnt = DAT_00523584 as u32;
            while cnt != 0 {
                cnt -= 1;
                let b = tab(pos);
                pos = pos.wrapping_add(1);
                let (p1, p2, p3, q1, q2, q3) = comp(
                    b,
                    c!(DAT_009fe740),
                    c!(DAT_009e9600),
                    c!(DAT_009ec480),
                    c!(DAT_00a08250),
                    c!(DAT_00a004e0),
                    c!(DAT_00a07c80),
                );
                let z = p3.wrapping_add(q3).wrapping_sub((DAT_00a004b0 >> 2) as u32);
                if (z as i32) >= 0 {
                    FUN_00426860(
                        p1.wrapping_add(q1).wrapping_sub((DAT_00a004a8 >> 2) as u32) as i32,
                        p2.wrapping_add(q2).wrapping_sub((DAT_00a004a4 >> 2) as u32) as i32,
                        z,
                        d3,
                        pos,
                    );
                }
            }
        } else {
            cnt = DAT_00523580 as u32;
            while cnt != 0 {
                cnt -= 1;
                let b = tab(pos);
                pos = pos.wrapping_add(1);
                let (p1, p2, p3, q1, q2, q3) = comp(
                    b,
                    c!(DAT_009fe740),
                    c!(DAT_009e9600),
                    c!(DAT_009ec480),
                    c!(DAT_00a08250),
                    c!(DAT_00a004e0),
                    c!(DAT_00a07c80),
                );
                let z = p3.wrapping_add(q3).wrapping_add((DAT_00a004b0 >> 2) as u32);
                if (z as i32) >= 0 {
                    FUN_00426860(
                        p1.wrapping_add(q1).wrapping_add((DAT_00a004a8 >> 2) as u32) as i32,
                        p2.wrapping_add(q2).wrapping_add((DAT_00a004a4 >> 2) as u32) as i32,
                        z,
                        d3,
                        pos,
                    );
                }
            }
            pos = pos.wrapping_add(DAT_00523584 as u32);
        }

        let u3 = FUN_0046abf0(DAT_00a004a4, DAT_00a004a8);
        let d3 = (u3 as i16).wrapping_neg();
        if DAT_00a00490 < 0 {
            pos = pos.wrapping_add(DAT_00523578 as u32);
            cnt = DAT_0052357c as u32;
            while cnt != 0 {
                cnt -= 1;
                let b = tab(pos);
                pos = pos.wrapping_add(1);
                let (p1, p2, p3, q1, q2, q3) = comp(
                    b,
                    c!(DAT_009fe740),
                    c!(DAT_009e9600),
                    c!(DAT_009ec480),
                    c!(DAT_009d80d0),
                    c!(DAT_009a8dd0),
                    c!(DAT_009d1270),
                );
                let z = p3.wrapping_add(q3).wrapping_sub((DAT_00a00490 >> 2) as u32);
                if (z as i32) >= 0 {
                    FUN_00426860(
                        p1.wrapping_add(q1).wrapping_sub((DAT_00a00488 >> 2) as u32) as i32,
                        p2.wrapping_add(q2).wrapping_sub((DAT_00a00480 >> 2) as u32) as i32,
                        z,
                        d3,
                        pos,
                    );
                }
            }
        } else {
            cnt = DAT_00523578 as u32;
            while cnt != 0 {
                cnt -= 1;
                let b = tab(pos);
                pos = pos.wrapping_add(1);
                let (p1, p2, p3, q1, q2, q3) = comp(
                    b,
                    c!(DAT_009fe740),
                    c!(DAT_009e9600),
                    c!(DAT_009ec480),
                    c!(DAT_009d80d0),
                    c!(DAT_009a8dd0),
                    c!(DAT_009d1270),
                );
                let z = p3.wrapping_add(q3).wrapping_add((DAT_00a00490 >> 2) as u32);
                if (z as i32) >= 0 {
                    FUN_00426860(
                        p1.wrapping_add(q1).wrapping_add((DAT_00a00488 >> 2) as u32) as i32,
                        p2.wrapping_add(q2).wrapping_add((DAT_00a00480 >> 2) as u32) as i32,
                        z,
                        d3,
                        pos,
                    );
                }
            }
        }
    }
}
