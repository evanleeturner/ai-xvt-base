// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00534360: i8;
    static mut DAT_00654d68: i8;
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081d402: u8;
    static mut DAT_0081d40a: u8;
    static mut DAT_0081d40e: u8;
    static mut DAT_0081fb96: u8;
    static mut DAT_0081fc4e: u8;
    static mut DAT_0081b740: i8;
    static mut DAT_005319c4: i8;
    static mut s__c_s___c_s_005319cc: i8;
    static mut s__s__s____s_00534354: i8;
    static mut s_medlvl_d_005321f8: i8;
    static mut s__c_s__c_d_005333b4: i8;
    static mut s__c_s__c_d__c_s__c_d_0053433c: i8;
    static mut DAT_00757cc0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dbe00(a: i32) -> u8;
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004da920(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004dd030(a: *mut c_void, b: i32, c: i32);
    #[link_name = "sprintf"]
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_004db2f0(a: *mut i8, b: i32) -> i32;
    fn FUN_004d0500(a: *mut i8, b: *mut i32);
    fn FUN_004da7c0(a: i32, b: *mut i8, c: i32, d: i32, e: u32);
}

#[inline(always)]
unsafe fn rd(p: *const u8) -> i32 {
    // SAFETY: p addresses packed game memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn level_block(l: *mut i32, l34: *mut u8) {
    // SAFETY: game globals and callees; l points at the caller's 21-int frame.
    unsafe {
        FUN_004dd030(l.cast(), 0, 0xf);
        let idx = rd(&raw const DAT_0081fb96) as u32;
        let val = rd((&raw const DAT_0081fc4e).wrapping_add(idx.wrapping_mul(0xe824) as usize));
        let pc2 = FUN_004dcfa0((val as u32).wrapping_add(0x170) as i32);
        let pc3 = FUN_004dcfa0(0x170);
        let pc4 = FUN_004dcfa0(0x16c);
        _sprintf(
            &raw mut DAT_0081b740,
            &raw const s__s__s____s_00534354,
            pc4,
            pc3,
            pc2,
        );
        let i5 = FUN_004db2f0(&raw mut DAT_0081b740, 0xc);
        let idx = rd(&raw const DAT_0081fb96) as u32;
        let val = rd((&raw const DAT_0081fc4e).wrapping_add(idx.wrapping_mul(0xe824) as usize));
        _sprintf(l34.cast::<i8>(), &raw const s_medlvl_d_005321f8, val);
        FUN_004d0500(l34.cast::<i8>(), l.add(4));
        let a = ((l.add(2).read() as u32).wrapping_sub(l.read() as u32) as i32 >> 1) as u32;
        let b = (((i5 as u32).wrapping_add(10))
            .wrapping_sub(l.add(4).read() as u32)
            .wrapping_add(l.add(6).read() as u32)
            .wrapping_add(0xb))
            >> 1;
        let i6 = a.wrapping_sub(b).wrapping_add(l.read() as u32) as i32;
        FUN_004da7c0(0xc, &raw mut DAT_0081b740, i6, l.add(1).read(), 0xffff);
        FUN_004d0e60(
            l34.cast::<i8>(),
            (i6 as u32)
                .wrapping_add(i5 as u32)
                .wrapping_add(10)
                .wrapping_add(10) as i32,
            l.add(1).read(),
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fca40() -> u32 {
    let mut frame: [i32; 21] = [0; 21];
    let l = frame.as_mut_ptr();
    // SAFETY: the frame has 21 ints; index 8 starts the 52-byte byte area.
    let l34 = unsafe { l.add(8) } as *mut u8;
    let mut i7: i32 = 0;
    let mut i9: i32 = 0;

    // SAFETY: callees and globals are the game's own; l/l34 point into frame.
    unsafe {
        if FUN_004dbe00(0x12) != 0 && FUN_004dbe00(0x10) != 0 && FUN_004dbe00(0x78) != 0 {
            FUN_004d0e60(&raw mut DAT_00534360, 0xaf, 0x156);
        }
        FUN_004dcfd0(l, 0x54, 0x5a, 0x1b2, 0x6a);
        let pc2 = FUN_004dcfa0(0x12d);
        FUN_004da920(0xf, pc2, l, 0xffff);
        FUN_004dcfd0(l, 0x54, 0x6e, 0x1b2, 0x73);
        FUN_004da920(0xc, &raw mut DAT_00654d68, l, DAT_0081bcd0);
        FUN_004dd030(l.cast(), 0, 0x14);

        let mut u13: u32 = 0;
        loop {
            if 0xff < u13 {
                break;
            }
            let t = rd((&raw const DAT_0081d40e).wrapping_add((u13 as usize).wrapping_mul(4)));
            if t == 0 {
                i7 = i7.wrapping_add(1);
            } else {
                let t = rd((&raw const DAT_0081d40e).wrapping_add((u13 as usize).wrapping_mul(4)));
                if t == 1 {
                    i9 = i9.wrapping_add(1);
                }
            }
            u13 = u13.wrapping_add(1);
            let tu = rd(&raw const DAT_0081d402) as u32;
            if !(u13 <= tu) {
                break;
            }
        }

        if rd(&raw const DAT_0081d40a) == i7 {
            let pc2 = FUN_004dcfa0(0x14c);
            let pc3 = FUN_004dcfa0(0x2b7);
            _sprintf(
                &raw mut DAT_0081b740,
                &raw const s__c_s___c_s_005319cc,
                4i32,
                pc3,
                5u32,
                pc2,
            );
            FUN_004da920(0xc, &raw mut DAT_0081b740, l, 0xffff);
            let idx = rd(&raw const DAT_0081fb96) as u32;
            let val = rd((&raw const DAT_0081fc4e).wrapping_add(idx.wrapping_mul(0xe824) as usize));
            if val != 0 {
                level_block(l, l34);
            }
        } else if rd(&raw const DAT_0081d40a) == i9 {
            let pc2 = FUN_004dcfa0(0x14d);
            let pc3 = FUN_004dcfa0(0x2b7);
            _sprintf(
                &raw mut DAT_0081b740,
                &raw const s__c_s___c_s_005319cc,
                4i32,
                pc3,
                3u32,
                pc2,
            );
            FUN_004da920(0xc, &raw mut DAT_0081b740, l, 0xffff);
            let idx = rd(&raw const DAT_0081fb96) as u32;
            let val = rd((&raw const DAT_0081fc4e).wrapping_add(idx.wrapping_mul(0xe824) as usize));
            if val != 0 {
                level_block(l, l34);
            }
        } else {
            let i5 = rd(&raw const DAT_0081d40a);
            let pc2 = FUN_004dcfa0(0x148);
            _sprintf(
                &raw mut DAT_0081b740,
                &raw const s__c_s__c_d_005333b4,
                4i32,
                pc2,
                1u32,
                i5,
            );
            FUN_004da920(0xc, &raw mut DAT_0081b740, l, 0xffff);
        }

        FUN_004dd030(l.cast(), 0, 0x14);
        let pc2 = FUN_004dcfa0(0x14a);
        let pc3 = FUN_004dcfa0(0x149);
        _sprintf(
            &raw mut DAT_0081b740,
            &raw const s__c_s__c_d__c_s__c_d_0053433c,
            5i32,
            pc3,
            1u32,
            i7,
            3u32,
            pc2,
            1u32,
            i9,
        );
        FUN_004da920(0xc, &raw mut DAT_0081b740, l, 0xffff);
        let mut u13: u32 = 0;
        FUN_004dd030(l.cast(), 0, 0x14);
        let mut p8: *const u8 = &raw const DAT_0081d40e;
        loop {
            if (p8 as usize) > 0x81d80d {
                return 1;
            }
            let v = rd(p8);
            i7 = v;
            if v == 0 {
                l34.write(5);
            } else if v == 1 {
                l34.write(3);
            } else if v == 2 {
                l34.write(1);
            }
            u13 = u13.wrapping_add(1);
            let ta = rd(p8.wrapping_add(0x800));
            _sprintf(
                &raw mut DAT_0081b740,
                &raw const DAT_005319c4,
                4i32,
                (ta as u32)
                    .wrapping_mul(0x144)
                    .wrapping_add(DAT_00757cc0 as u32)
                    .wrapping_add(0x40) as i32,
            );
            FUN_004da920(0xc, &raw mut DAT_0081b740, l, 0xffff);
            FUN_004dd030(l.cast(), 0, 0xf);
            let ta = rd(p8);
            let pc2 = FUN_004dcfa0((ta as u32).wrapping_add(0x14c) as i32);
            _sprintf(
                &raw mut DAT_0081b740,
                &raw const DAT_005319c4,
                l34.read() as u32,
                pc2,
            );
            FUN_004da920(0xc, &raw mut DAT_0081b740, l, 0xffff);
            FUN_004dd030(l.cast(), 0, 0xf);
            p8 = p8.wrapping_add(4);
            let tu = rd(&raw const DAT_0081d402) as u32;
            if !(u13 <= tu) {
                break;
            }
        }
        let _ = i7;
        1
    }
}
