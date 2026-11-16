// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e8f: i8;
    static mut DAT_005f1d44: i32;
    static mut DAT_006db7bc: i32;
    static mut DAT_00525518: i32;
    static mut DAT_006da4fc: i32;
    static mut DAT_0070acc8: i32;
    static mut DAT_00525548: u8;
    static mut DAT_0052ae50: i32;
    static mut DAT_006da540: [u8; 0];
    static mut DAT_006da541: [u8; 0];
    static mut DAT_006da542: [u8; 0];
    static mut DAT_006d4984: u16;
    static mut DAT_007091ae: u16;
    static mut DAT_006a888e: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004a6a80();
    fn FUN_004a6f90();
}

unsafe fn fill16(start: u32, end: u32, v: u16) {
    let mut p = start;
    if p < end {
        let n = end.wrapping_add(1u32.wrapping_sub(p));
        let mut cnt = n >> 2;
        let dv = ((v as u32) << 16) | (v as u32);
        while cnt != 0 {
            // SAFETY: writes into the game's own frame buffer, as the original does.
            unsafe { (p as usize as *mut u32).write_unaligned(dv) };
            p = p.wrapping_add(4);
            cnt = cnt.wrapping_sub(1);
        }
        if ((n >> 1) & 1) != 0 {
            // SAFETY: same frame buffer write as above.
            unsafe { (p as usize as *mut u16).write_unaligned(v) };
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00478040() {
    // SAFETY: all globals, callees and pointers are the game's own memory;
    // reads and writes happen in the original's order.
    unsafe {
        if DAT_00732e8f == 0 {
            FUN_004a6a80();
        }
        DAT_005f1d44 = (DAT_006db7bc as u32)
            .wrapping_mul(DAT_00525518 as u32)
            .wrapping_add((DAT_006da4fc as u32).wrapping_mul(DAT_0070acc8 as u32))
            as i32;
        if DAT_00525518 == 2 {
            let idx = (DAT_00525548 as usize).wrapping_mul(3);
            let r = ((&raw mut DAT_006da540) as *mut u8).add(idx).read() as u16;
            let g = ((&raw mut DAT_006da541) as *mut u8).add(idx).read() as u16;
            let b = ((&raw mut DAT_006da542) as *mut u8).add(idx).read() as u16;
            let s9: u16 = ((r >> 1).wrapping_mul(0x20).wrapping_add(g >> 1))
                .wrapping_mul(0x40)
                .wrapping_add(b >> 1);
            let mut ivar7: i32 = 0;
            let mut istack4: i32 = 0;
            if DAT_006d4984 != 0 {
                loop {
                    let mut ivar10: i32 = 0;
                    let ibase = (&raw const DAT_006a888e).read_unaligned();
                    let mut pvar8: *mut i32 = ((ibase.wrapping_add(ivar7)) as u32 as usize
                        as *const *mut i32)
                        .read_unaligned();
                    loop {
                        if pvar8.is_null() {
                            break;
                        }
                        let base = (DAT_005f1d44 as u32).wrapping_add(DAT_0052ae50 as u32);
                        let p12 = base.wrapping_add((ivar10 as u32).wrapping_mul(2));
                        let p1 = base
                            .wrapping_add((pvar8.add(1).read_unaligned() as u32).wrapping_mul(2));
                        fill16(p12, p1, s9);
                        ivar10 = pvar8.add(2).read_unaligned();
                        pvar8 = (pvar8 as *const *mut i32).read_unaligned();
                        if pvar8.is_null() {
                            break;
                        }
                        loop {
                            if ivar10 < pvar8.add(1).read_unaligned() {
                                break;
                            }
                            if ivar10 < pvar8.add(2).read_unaligned() {
                                ivar10 = pvar8.add(2).read_unaligned();
                            }
                            pvar8 = (pvar8 as *const *mut i32).read_unaligned();
                            if pvar8.is_null() {
                                break;
                            }
                        }
                    }
                    let base = (DAT_005f1d44 as u32).wrapping_add(DAT_0052ae50 as u32);
                    let p12 = base.wrapping_add((ivar10 as u32).wrapping_mul(2));
                    let p1 = base.wrapping_add((DAT_007091ae as u32).wrapping_mul(2));
                    fill16(p12, p1, s9);
                    DAT_005f1d44 = (DAT_005f1d44 as u32).wrapping_add(DAT_0070acc8 as u32) as i32;
                    ivar7 = ivar7.wrapping_add(4);
                    istack4 = istack4.wrapping_add(1);
                    if !(istack4 < DAT_006d4984 as i32) {
                        break;
                    }
                }
            }
        } else {
            let mut ivar7: i32 = 0;
            let mut ivar10: i32 = 0;
            if DAT_006d4984 != 0 {
                loop {
                    let mut ivar11: i32 = 0;
                    let ibase = (&raw const DAT_006a888e).read_unaligned();
                    let mut pvar8: *mut i32 = ((ibase.wrapping_add(ivar7)) as u32 as usize
                        as *const *mut i32)
                        .read_unaligned();
                    loop {
                        if pvar8.is_null() {
                            break;
                        }
                        let ivar2 = (DAT_005f1d44 as u32).wrapping_add(DAT_0052ae50 as u32);
                        let ivar4 = pvar8.add(1).read_unaligned() as u32;
                        let mut p = ivar2.wrapping_add(ivar11 as u32);
                        let e = ivar2.wrapping_add(ivar4);
                        while p < e {
                            (p as usize as *mut u8).write(DAT_00525548);
                            p = p.wrapping_add(1);
                        }
                        ivar11 = pvar8.add(2).read_unaligned();
                        pvar8 = (pvar8 as *const *mut i32).read_unaligned();
                        if pvar8.is_null() {
                            break;
                        }
                        loop {
                            if ivar11 < pvar8.add(1).read_unaligned() {
                                break;
                            }
                            if ivar11 < pvar8.add(2).read_unaligned() {
                                ivar11 = pvar8.add(2).read_unaligned();
                            }
                            pvar8 = (pvar8 as *const *mut i32).read_unaligned();
                            if pvar8.is_null() {
                                break;
                            }
                        }
                    }
                    let ivar4 = (DAT_005f1d44 as u32).wrapping_add(DAT_0052ae50 as u32);
                    let uvar5 = DAT_007091ae as u32;
                    let mut p = ivar4.wrapping_add(ivar11 as u32);
                    let e = ivar4.wrapping_add(uvar5);
                    while p < e {
                        (p as usize as *mut u8).write(DAT_00525548);
                        p = p.wrapping_add(1);
                    }
                    DAT_005f1d44 = (DAT_005f1d44 as u32).wrapping_add(DAT_0070acc8 as u32) as i32;
                    ivar7 = ivar7.wrapping_add(4);
                    ivar10 = ivar10.wrapping_add(1);
                    if !(ivar10 < DAT_006d4984 as i32) {
                        break;
                    }
                }
            }
        }
        if DAT_00732e8f == 0 {
            FUN_004a6f90();
        }
    }
}
