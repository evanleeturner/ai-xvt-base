// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071bb37: u8;
    static mut DAT_0071b730: u8;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00655184: u16;
    static mut DAT_00655a84: u16;
    static mut DAT_00656c90: u8;
    static mut DAT_00655ab0: u8;
    static mut DAT_00655ab2: u8;
    static mut DAT_00655ab4: u8;
    static mut DAT_00655ab6: u8;
    static mut DAT_0071f2f0: u32;
    static mut DAT_0070a184: *const core::ffi::c_void;
    static mut DAT_00656d42: u8;
    static mut DAT_00656d44: u8;
    static mut DAT_00656d3e: u8;
    static mut DAT_00656d0c: u8;
    static mut DAT_00656d0a: u8;
    static mut DAT_00656d04: u8;
    static mut DAT_00656d38: u16;
    static mut DAT_00656cea: u16;
    static mut DAT_00655cd0: u16;
    static mut DAT_00655ccc: u16;
    static mut DAT_00655cce: u16;
    static mut DAT_00655cd2: u16;
}

type CallFn = unsafe extern "C" fn(u32, u16, u16, u16, i32);

unsafe fn rd<T>(a: i32) -> T {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as *const T).read_unaligned() }
}

unsafe fn cls(iv: i32) -> i32 {
    // SAFETY: game-owned table read as the original does.
    unsafe {
        let c: i8 = rd::<i8>(
            DAT_006d4988
                .wrapping_add(3)
                .wrapping_add((iv as u32).wrapping_mul(0x23) as i32),
        );
        if iv == -1 || (c != 1 && c != 2 && c != 3 && c != 0xe && c != 4) {
            0
        } else {
            1
        }
    }
}

unsafe fn flags(iv: i32, off: i32) -> u16 {
    // SAFETY: game-owned pointer chain read as the original does.
    unsafe {
        let p: i32 = rd::<i32>(
            DAT_006d4988
                .wrapping_add(0x1f)
                .wrapping_add((iv as u32).wrapping_mul(0x23) as i32),
        );
        let q: i32 = rd::<i32>(p.wrapping_add(0xa7));
        rd::<u16>(q.wrapping_add(off))
    }
}

unsafe fn call(a: u32, b: u16, c: u16, d: u16) {
    // SAFETY: the pointer is the game's callback with this signature.
    unsafe {
        let f: CallFn = core::mem::transmute(DAT_0070a184);
        f(a, b, c, d, 0);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004afb90() {
    // SAFETY: all globals and pointers belong to the game; accessed as the original does.
    unsafe {
        let idx: u32 = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd);
        let b30 = (&raw mut DAT_0071b730) as *mut u8;
        let tab = (&raw mut DAT_0071f2f0) as *mut u32;
        let a656cea = (&raw mut DAT_00656cea) as *mut u16;
        let a655cd0 = (&raw mut DAT_00655cd0) as *mut u16;
        let a655ccc = (&raw mut DAT_00655ccc) as *mut u16;
        let a655cce = (&raw mut DAT_00655cce) as *mut u16;
        let a655cd2 = (&raw mut DAT_00655cd2) as *mut u16;

        let t = ((&raw mut DAT_0071bb37) as *mut u8)
            .add(idx as usize)
            .read();
        if t != 0 {
            if t == 0x13 {
                let mut u7: u16 = 1;
                let mut u6: u16 = 0;
                loop {
                    if u7 < 5 && u7 != 4 && u7 == 1 {
                        let mut iv4: i32 = b30
                            .add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
                            .cast::<i32>()
                            .read_unaligned();
                        let b2 = cls(iv4);
                        let mut skip = false;
                        if b2 != 0 {
                            let f = flags(iv4, 0x1db);
                            if (f & 1) != 0 {
                                skip = true;
                            }
                        }
                        if !skip {
                            let f2 = flags(iv4, 0x1d9);
                            if (f2 & 1) != 0 {
                                let b2 = cls(iv4);
                                if b2 != 0 || DAT_00655184 != 0 {
                                    iv4 = (DAT_00655184 as i32).wrapping_add(0x54);
                                    let pp = ((&raw mut DAT_00656c90) as *mut u8)
                                        .byte_add(iv4.wrapping_mul(2) as usize)
                                        .cast::<u16>();
                                    if pp.read_unaligned() != 0 {
                                        pp.write_unaligned(0);
                                        iv4 = iv4.wrapping_mul(0xc);
                                        let o = iv4 as usize;
                                        let i4 = ((&raw mut DAT_00655ab4) as *mut u8)
                                            .byte_add(o)
                                            .cast::<u16>()
                                            .read_unaligned();
                                        let v0 = ((&raw mut DAT_00655ab0) as *mut u8)
                                            .byte_add(o)
                                            .cast::<u16>()
                                            .read_unaligned();
                                        let v2 = ((&raw mut DAT_00655ab2) as *mut u8)
                                            .byte_add(o)
                                            .cast::<u16>()
                                            .read_unaligned();
                                        let v6 = ((&raw mut DAT_00655ab6) as *mut u8)
                                            .byte_add(o)
                                            .cast::<u16>()
                                            .read_unaligned();
                                        call(tab.add(i4 as usize).read(), v0, v2, v6);
                                    }
                                    let u5 = DAT_00655184 as u32;
                                    let o = (u5.wrapping_mul(2)) as usize;
                                    ((&raw mut DAT_00656d42) as *mut u8)
                                        .byte_add(o)
                                        .cast::<u16>()
                                        .write_unaligned(0xffff);
                                    ((&raw mut DAT_00656d44) as *mut u8)
                                        .byte_add(o)
                                        .cast::<u16>()
                                        .write_unaligned(0xffff);
                                    ((&raw mut DAT_00656d3e) as *mut u8)
                                        .byte_add(o)
                                        .cast::<u16>()
                                        .write_unaligned(0xffff);
                                    ((&raw mut DAT_00656d0c) as *mut u8)
                                        .byte_add(o)
                                        .cast::<u16>()
                                        .write_unaligned(0xffff);
                                    ((&raw mut DAT_00656d0a) as *mut u8)
                                        .byte_add(o)
                                        .cast::<u16>()
                                        .write_unaligned(0xffff);
                                    ((&raw mut DAT_00656d04) as *mut u8)
                                        .byte_add(o)
                                        .cast::<u16>()
                                        .write_unaligned(0xffff);
                                    ((&raw mut DAT_00656d38) as *mut u16)
                                        .add(u5 as usize)
                                        .write(0xffff);
                                } else if a656cea.read() != 0 {
                                    a656cea.write(0);
                                    call(
                                        tab.add(a655cd0.read() as usize).read(),
                                        a655ccc.read(),
                                        a655cce.read(),
                                        a655cd2.read(),
                                    );
                                }
                            }
                        }
                    }
                    u7 = u7.wrapping_mul(2);
                    u6 = u6.wrapping_add(1);
                    if !(u6 < 0xd) {
                        break;
                    }
                }
                DAT_00655a84 = 0xffff;
            }
            return;
        }

        let mut u7: u16 = 1;
        let mut u5: u32 = 0;
        loop {
            let iv4: i32 = b30
                .add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
                .cast::<i32>()
                .read_unaligned();
            let b2 = cls(iv4);
            let f = flags(iv4, 0x1db);
            let mut u6: u16;
            let mut go = true;
            if b2 != 0 {
                if (f & u7) == 0 {
                    u6 = 0;
                } else {
                    u6 = 0;
                    go = false;
                }
            } else {
                u6 = (0i32.wrapping_sub(((f & u7) == 0) as i32) & 0xd) as u16;
            }
            if go {
                let f2 = flags(iv4, 0x1d9);
                if (f2 & u7) != 0 && a656cea.add(u5 as usize).read() != u6 {
                    a656cea.add(u5 as usize).write(u6);
                    let k = (u5.wrapping_mul(6)) as usize;
                    call(
                        tab.add((a655cd0.add(k).read() as u32).wrapping_add(u6 as u32) as usize)
                            .read(),
                        a655ccc.add(k).read(),
                        a655cce.add(k).read(),
                        a655cd2.add(k).read(),
                    );
                }
            }
            u7 = u7.wrapping_mul(2);
            u6 = (u5 as u16).wrapping_add(1);
            u5 = u6 as u32;
            if 0xc < u6 {
                DAT_00655a84 = 0xffff;
                return;
            }
        }
    }
}
