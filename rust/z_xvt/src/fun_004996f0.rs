// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645a80: *mut u8;
    static mut DAT_00703c88: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_006db7a4: i32;
    static mut DAT_00703ac0: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_007091a0: i32;
    static mut DAT_00732940: i32;
    static mut DAT_006db798: i32;
    static mut DAT_006db71c: i32;
    static mut DAT_0071f2d8: u32;
    static mut _DAT_0071f2dc: u32;
    static mut DAT_0070ac30: u32;
    static mut _DAT_0070ac34: u32;
    static mut DAT_006d49a0: i16;
    static mut DAT_006d4a60: [u8; 1];
    static mut DAT_0070acf0: [u8; 1];
    static mut DAT_007092c0: [u8; 1];
    static mut DAT_0071e6a0: [u8; 1];
    static mut DAT_00732fe4: u16;
    static mut DAT_00732e48: u32;
    static mut DAT_0070ace0: u8;
    static mut DAT_0071eae8: u32;
    static mut DAT_006da530: u32;
    static mut DAT_0072f4e0: u32;
    static mut DAT_00732e9c: u32;
    static mut DAT_006da4f0: u32;
    static mut DAT_0070a000: u32;
    static mut _DAT_00732a3c: u32;
    static mut DAT_006d4980: u32;
    static mut DAT_00703acc: u32;
    static mut DAT_0073088c: u32;
    static mut DAT_0071b704: u32;
    static mut DAT_00732904: u32;
    static mut DAT_006da4f4: u32;
    static mut DAT_00703ac8: u32;
    static mut DAT_00703c90: u32;
    static mut DAT_0071e53c: u32;
    static mut DAT_00703ca0: [u8; 1];
    static mut DAT_006da9e8: u32;
    static mut DAT_006d9d44: u32;
    static mut DAT_006da3e0: [u8; 1];
    static mut DAT_00703abc: u16;
    static mut DAT_006db6c0: u16;
    static mut DAT_00525564: u32;
    static mut DAT_0071b730: [u8; 1];
}

#[inline(always)]
unsafe fn rd32(p: *const u8) -> u32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(p: *const u8) -> u16 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<u16>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(p: *mut u8, v: u32) {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

#[inline(always)]
unsafe fn cp(d: *mut u8, s: *const u8, n: usize) {
    let mut i = 0usize;
    while i < n {
        // SAFETY: caller supplies valid game buffers; forward byte copy as the original.
        unsafe { d.add(i).write(s.add(i).read()) };
        i += 1;
    }
}

#[inline(always)]
unsafe fn zr(d: *mut u8, n: usize) {
    let mut i = 0usize;
    while i < n {
        // SAFETY: caller supplies a valid game buffer.
        unsafe { d.add(i).write(0) };
        i += 1;
    }
}

#[inline(always)]
unsafe fn adj(base: *mut u8, off: usize, add: u32) {
    // SAFETY: caller supplies a valid game address.
    unsafe {
        let a = base.add(off);
        let pv = rd32(a);
        if pv != 0 {
            wr32(a, pv.wrapping_sub(1));
            let pv = rd32(a);
            wr32(a, pv.wrapping_add(add));
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004996f0() {
    // SAFETY: all globals and the buffers they point at are the game's own
    // memory; reads and writes follow the original's order and extents.
    unsafe {
        let mut i_var9: i32 = 0;
        let mut pu7: *mut u8 = DAT_00645a80;
        if 0 < DAT_00703c88.wrapping_add(DAT_00732a38) {
            let mut i_var8: i32 = 0;
            let mut pu6: *mut u8 = DAT_00645a80;
            loop {
                let mut run = false;
                if i_var9 < DAT_006db7a4 {
                    run = true;
                } else {
                    pu7 = pu6;
                    if DAT_00703ac0 <= i_var9 {
                        run = true;
                    }
                }
                if run {
                    let e: *mut u8 =
                        (DAT_006d4988 as u32).wrapping_add(i_var8 as u32) as usize as *mut u8;
                    pu7 = pu6.add(1);
                    e.add(3).write(pu6.read());
                    if e.add(3).read() == 0 {
                        zr(e, 28);
                        zr(e.add(28), 3);
                        wr32(e.add(0x1b), 0xffff_ffff);
                        let p = rd32(e.add(0x1f)) as usize as *mut u8;
                        if !p.is_null() {
                            zr(p, 0x22 * 4 + 1);
                            p.add(0x86).write(0xff);
                            let q = rd32(p.add(0xa7)) as usize as *mut u8;
                            if !q.is_null() {
                                zr(q, 0xf9 * 4 + 2);
                            }
                            let q = rd32(p.add(0xa3)) as usize as *mut u8;
                            if !q.is_null() {
                                zr(q, 8);
                            }
                            let q = rd32(p.add(0xab)) as usize as *mut u8;
                            if !q.is_null() {
                                zr(q, 0x13 * 4);
                            }
                        }
                    } else {
                        cp(e, pu7, 35);
                        pu7 = pu6.add(36);
                        adj(e, 0x1f, DAT_007091a0 as u32);
                        let p = rd32(e.add(0x1f)) as usize as *mut u8;
                        if !p.is_null() {
                            cp(p, pu7, 0x2b * 4 + 3);
                            let mut pu10 = pu6.add(0xd3);

                            adj(p, 0xa7, DAT_00732940 as u32);
                            adj(p, 0xa3, DAT_006db798 as u32);
                            adj(p, 0xab, DAT_006db71c as u32);

                            let q = rd32(p.add(0xa7)) as usize as *mut u8;
                            if !q.is_null() {
                                cp(q, pu10, 0x10d * 4 + 2);
                                pu10 = pu6.add(0x509);
                                let mut i_var3: usize = 0;
                                loop {
                                    let q = rd32(p.add(0xa7)) as usize as *mut u8;
                                    adj(q, 0x3f2 + i_var3, DAT_006d4988 as u32);
                                    i_var3 += 4;
                                    if i_var3 >= 0x40 {
                                        break;
                                    }
                                }
                                let q = rd32(p.add(0xa7)) as usize as *mut u8;
                                adj(q, 0x432, DAT_006d4988 as u32);
                            }

                            let q = rd32(p.add(0xa3)) as usize as *mut u8;
                            pu7 = pu10;
                            if !q.is_null() {
                                pu7 = pu10.add(8);
                                cp(q, pu10, 8);
                            }
                            let r = rd32(p.add(0xab)) as usize as *mut u8;
                            if !r.is_null() {
                                cp(r, pu7, 0x13 * 4);
                                pu7 = pu7.add(0x4c);
                            }
                        }
                    }
                }
                i_var8 = i_var8.wrapping_add(0x23);
                i_var9 = i_var9.wrapping_add(1);
                pu6 = pu7;
                if !(i_var9 < DAT_00703c88.wrapping_add(DAT_00732a38)) {
                    break;
                }
            }
        }

        DAT_0071f2d8 = rd32(pu7);
        _DAT_0071f2dc = rd32(pu7.add(4));
        DAT_0070ac30 = rd32(pu7.add(8));
        _DAT_0070ac34 = rd32(pu7.add(12));
        cp(
            (&raw mut DAT_006d49a0).cast::<u8>(),
            pu7.add(16),
            0x28 * 4 + 2,
        );
        let n1 = (DAT_006d49a0 as i32 as u32).wrapping_mul(0x126);
        cp(
            (&raw mut DAT_006d4a60).cast::<u8>(),
            pu7.add(0xb2),
            n1 as usize,
        );
        pu7 = pu7
            .add(0xb2usize.wrapping_add((DAT_006d49a0 as i32 as u32).wrapping_mul(0x126) as usize));
        let n2 = (DAT_006d49a0 as i32 as u32).wrapping_mul(0x566);
        cp((&raw mut DAT_0070acf0).cast::<u8>(), pu7, n2 as usize);
        pu7 = pu7.add((DAT_006d49a0 as i32 as u32).wrapping_mul(0x566) as usize);

        cp((&raw mut DAT_007092c0).cast::<u8>(), pu7, 0x34c * 4);
        cp(
            (&raw mut DAT_0071e6a0).cast::<u8>(),
            pu7.add(0xd30),
            5 * 4 + 2,
        );
        DAT_00732fe4 = rd16(pu7.add(0xd46));
        DAT_00732e48 = rd32(pu7.add(0xd48));
        DAT_0070ace0 = pu7.add(0xd4c).read();
        DAT_0071eae8 = rd32(pu7.add(0xd4d));
        DAT_006da530 = rd32(pu7.add(0xd51));
        DAT_0072f4e0 = rd32(pu7.add(0xd55));
        DAT_00732e9c = rd32(pu7.add(0xd59));
        DAT_006da4f0 = rd32(pu7.add(0xd5d));
        DAT_0070a000 = rd32(pu7.add(0xd61));
        _DAT_00732a3c = rd32(pu7.add(0xd65));
        DAT_006d4980 = rd32(pu7.add(0xd69));
        DAT_00703acc = rd32(pu7.add(0xd6d));
        DAT_0073088c = rd32(pu7.add(0xd71));
        DAT_0071b704 = rd32(pu7.add(0xd75));
        DAT_00732904 = rd32(pu7.add(0xd79));
        DAT_006da4f4 = rd32(pu7.add(0xd7d));
        DAT_00703ac8 = rd32(pu7.add(0xd81));
        DAT_00703c90 = rd32(pu7.add(0xd85));
        DAT_0071e53c = rd32(pu7.add(0xd89));
        DAT_006db7a4 = rd32(pu7.add(0xd8d)) as i32;
        DAT_00703ac0 = rd32(pu7.add(0xd91)) as i32;
        DAT_00732a38 = rd32(pu7.add(0xd95)) as i32;
        DAT_00703c88 = rd32(pu7.add(0xd99)) as i32;
        cp(
            (&raw mut DAT_00703ca0).cast::<u8>(),
            pu7.add(0xd9d),
            0x1540 * 4,
        );
        DAT_006da9e8 = rd32(pu7.add(0x629d));
        DAT_006d9d44 = rd32(pu7.add(0x62a1));
        cp(
            (&raw mut DAT_006da3e0).cast::<u8>(),
            pu7.add(0x62a5),
            0x40 * 4,
        );
        DAT_00703abc = rd16(pu7.add(0x63a5));
        DAT_006db6c0 = rd16(pu7.add(0x63a7));
        DAT_00525564 = rd32(pu7.add(0x63a9));
        cp(
            (&raw mut DAT_0071b730).cast::<u8>(),
            pu7.add(0x63ad),
            0xb7a * 4,
        );
    }
}
