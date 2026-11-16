// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007330a4: i32;
    static mut DAT_0071e6c8: i8;
    static mut DAT_007330f8: i32;
    static mut DAT_00739290: i32;
    static mut DAT_007330ac: i32;
    static mut DAT_007330a0: i32;
    static mut DAT_00733074: *mut i32;
    static mut DAT_00539800: [u32; 1];
    static mut DAT_00733798: [i32; 1];
    static mut DAT_00733120: [i32; 1];
    static mut DAT_00733100: [i8; 1];
    static mut LAB_0042fe30: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
    fn FUN_00438a10() -> i32;
}

unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: addr is a game message field address supplied by the caller.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

unsafe fn fill_packet(param_2: i32) {
    // SAFETY: the globals are the game's own memory; the packet buffer and
    // player table are laid out as the original indexes them.
    unsafe {
        let pk = (&raw mut DAT_00539800).cast::<u32>();
        let b = (&raw mut DAT_00733798).cast::<i32>();
        let mut iv3: i32 = 0;
        if 0 < DAT_00739290 {
            let mut p2 = pk.add(4).cast::<i32>();
            let mut p4 = b;
            loop {
                iv3 = iv3.wrapping_add(1);
                p2.write(p4.read());
                p2.cast::<u8>().add(4).write(p4.cast::<u8>().add(12).read());
                p2.cast::<u8>().add(5).write(p4.cast::<u8>().add(20).read());
                p2.cast::<u8>().add(6).write(p4.cast::<u8>().add(16).read());
                p2.cast::<u8>().add(7).write(p4.cast::<u8>().add(24).read());
                p2 = p2.add(2);
                p4 = p4.add(0x8e);
                if !(iv3 < DAT_00739290) {
                    break;
                }
            }
        }
        FUN_00431110(
            rd(param_2.wrapping_add(8)),
            pk,
            (DAT_00739290 as u32).wrapping_mul(8).wrapping_add(0x10),
        );
    }
}

unsafe fn remove_entry(iv3: i32) {
    // SAFETY: the globals are the game's own memory; indexes follow the original.
    unsafe {
        let b = (&raw mut DAT_00733798).cast::<i32>();
        DAT_00739290 = DAT_00739290.wrapping_sub(1);
        let n = DAT_00739290 as u32;
        let mut p2 = b.offset(n.wrapping_mul(0x8e) as i32 as isize);
        let mut p4 = b.offset((iv3 as u32).wrapping_mul(0x8e) as i32 as isize);
        let mut c: i32 = 0x8e;
        while c != 0 {
            p4.write(p2.read());
            p2 = p2.add(1);
            p4 = p4.add(1);
            c = c.wrapping_sub(1);
        }
        let n = DAT_00739290 as u32;
        let base = n.wrapping_mul(0x8e);
        b.offset(base.wrapping_add(0) as i32 as isize).write(0);
        b.offset(base.wrapping_add(1) as i32 as isize).write(0x7f);
        b.offset(base.wrapping_add(3) as i32 as isize).write(0x7f);
        b.offset(base.wrapping_add(5) as i32 as isize).write(0x7f);
        b.offset(base.wrapping_add(2) as i32 as isize).write(0x7f);
        b.offset(base.wrapping_add(4) as i32 as isize).write(0x7f);
        b.offset(base.wrapping_add(6) as i32 as isize).write(0x7f);
        b.offset(base.wrapping_add(7) as i32 as isize).write(0);
        b.cast::<u8>()
            .offset(0x20u32.wrapping_add(n.wrapping_mul(0x238)) as i32 as isize)
            .write(0x39);
        b.offset(base.wrapping_add(0x88) as i32 as isize).write(1);
        b.offset(base.wrapping_add(0x89) as i32 as isize).write(0);
        b.offset(base.wrapping_add(0x8a) as i32 as isize).write(0);
        b.offset(base.wrapping_add(0x8b) as i32 as isize).write(0);
    }
}

unsafe fn remove_search(x: i32) {
    // SAFETY: the globals are the game's own memory.
    unsafe {
        let mut iv3: i32 = 0;
        if 0 < DAT_00739290 {
            let mut p = (&raw mut DAT_00733798).cast::<i32>();
            loop {
                if p.read() == x {
                    remove_entry(iv3);
                    return;
                }
                p = p.add(0x8e);
                iv3 = iv3.wrapping_add(1);
                if !(iv3 < DAT_00739290) {
                    break;
                }
            }
        }
    }
}

unsafe fn str_count(mut p: *const i8) -> u32 {
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        let mut n: u32 = 0;
        loop {
            let c = p.read();
            p = p.add(1);
            n = n.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        n
    }
}

unsafe fn copy_str(mut src: *const i8, mut dst: *mut i8, n: u32) {
    // SAFETY: src/dst are valid for n bytes as in the original.
    unsafe {
        let mut k = n >> 2;
        while k != 0 {
            (dst as *mut u32).write_unaligned((src as *const u32).read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            k -= 1;
        }
        let mut k = n & 3;
        while k != 0 {
            dst.write(src.read());
            src = src.add(1);
            dst = dst.add(1);
            k -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00430980(param_1: i32, param_2: i32) {
    // SAFETY: the globals and callees are the game's own; param_2 is the
    // caller's message address; the vtable slots are called as the original does.
    unsafe {
        let pk = (&raw mut DAT_00539800).cast::<u32>();
        if param_1 == 3 {
            let mut iv3: i32 = 0;
            if DAT_007330a4 != 0 {
                if DAT_0071e6c8 == 0 {
                    if rd(param_2.wrapping_add(4)) == 1 {
                        if rd(param_2.wrapping_add(8)) != DAT_007330f8 {
                            pk.add(1).write(0);
                            pk.add(2).write(DAT_00739290 as u32);
                            pk.write(0x3b);
                            pk.add(3).write(timeGetTime());
                            fill_packet(param_2);
                            pk.write(0x3a);
                            pk.add(1).write(0);
                            FUN_00431110(rd(param_2.wrapping_add(8)), pk, 8);
                        }
                        iv3 = 0;
                        DAT_007330ac = 0;
                        let vt = DAT_00733074.read() as usize as *const i32;
                        let f30: unsafe extern "stdcall" fn(
                            *mut i32,
                            i32,
                            *mut c_void,
                            i32,
                            i32,
                        ) -> i32 =
                            core::mem::transmute(vt.add(0x30 / 4).read() as usize as *const c_void);
                        f30(
                            DAT_00733074,
                            0,
                            (&raw mut LAB_0042fe30).cast::<c_void>(),
                            0,
                            0,
                        );
                        if 0 < DAT_007330ac {
                            let mut p = (&raw mut DAT_00733120).cast::<i32>();
                            loop {
                                if DAT_007330f8 != p.read() {
                                    let vt = DAT_00733074.read() as usize as *const i32;
                                    let f0c: unsafe extern "stdcall" fn(*mut i32, i32, i32) -> i32 =
                                        core::mem::transmute(
                                            vt.add(0xc / 4).read() as usize as *const c_void
                                        );
                                    f0c(DAT_00733074, DAT_007330a0, p.read());
                                }
                                p = p.add(10);
                                iv3 = iv3.wrapping_add(1);
                                if !(iv3 < DAT_007330ac) {
                                    break;
                                }
                            }
                            return;
                        }
                    }
                } else if rd(param_2.wrapping_add(4)) == 1
                    && rd(param_2.wrapping_add(8)) != DAT_007330f8
                {
                    pk.add(1).write(0);
                    pk.write(0x3b);
                    pk.add(2).write(DAT_00739290 as u32);
                    pk.add(3).write(timeGetTime());
                    fill_packet(param_2);
                    pk.write(0x3a);
                    pk.add(1).write(FUN_00438a10() as u32);
                    pk.add(2).write(2);
                    pk.add(3).write(0);
                    FUN_00431110(rd(param_2.wrapping_add(8)), pk, 0x10);
                    return;
                }
            }
        } else if param_1 == 5 {
            if DAT_007330a4 != 0 {
                if DAT_0071e6c8 == 0 {
                    if rd(param_2.wrapping_add(4)) == 1 {
                        DAT_007330ac = 0;
                        let vt = DAT_00733074.read() as usize as *const i32;
                        let f30: unsafe extern "stdcall" fn(
                            *mut i32,
                            i32,
                            *mut c_void,
                            i32,
                            i32,
                        ) -> i32 =
                            core::mem::transmute(vt.add(0x30 / 4).read() as usize as *const c_void);
                        f30(
                            DAT_00733074,
                            0,
                            (&raw mut LAB_0042fe30).cast::<c_void>(),
                            0,
                            0,
                        );
                        let vt = DAT_00733074.read() as usize as *const i32;
                        let f1c: unsafe extern "stdcall" fn(*mut i32, i32, i32) -> i32 =
                            core::mem::transmute(vt.add(0x1c / 4).read() as usize as *const c_void);
                        f1c(DAT_00733074, DAT_007330a0, rd(param_2.wrapping_add(8)));
                        remove_search(rd(param_2.wrapping_add(8)));
                        return;
                    }
                } else if rd(param_2.wrapping_add(4)) == 1 {
                    let mut iv3: i32 = 0;
                    if 0 < DAT_007330ac {
                        let t = (&raw mut DAT_00733120).cast::<i32>();
                        let mut p = t;
                        loop {
                            if p.read() == rd(param_2.wrapping_add(8)) {
                                let idx = 1u32.wrapping_add((iv3 as u32).wrapping_mul(10)) as i32
                                    as isize;
                                if t.offset(idx).read() != 0 {
                                    pk.write(0xd);
                                    FUN_00431110(DAT_007330f8, pk, 4);
                                }
                                let vt = DAT_00733074.read() as usize as *const i32;
                                let f1c: unsafe extern "stdcall" fn(*mut i32, i32, i32) -> i32 =
                                    core::mem::transmute(
                                        vt.add(0x1c / 4).read() as usize as *const c_void
                                    );
                                f1c(DAT_00733074, DAT_007330a0, rd(param_2.wrapping_add(8)));
                                t.offset(idx).write(0);
                                break;
                            }
                            p = p.add(10);
                            iv3 = iv3.wrapping_add(1);
                            if !(iv3 < DAT_007330ac) {
                                break;
                            }
                        }
                    }
                    remove_search(rd(param_2.wrapping_add(8)));
                    return;
                }
            }
        } else {
            if param_1 != 0x103 {
                return;
            }
            let mut iv3: i32 = 0;
            if rd(param_2.wrapping_add(4)) == 1 && 0 < DAT_007330ac {
                let mut p = (&raw mut DAT_00733120).cast::<i32>();
                while p.read() != rd(param_2.wrapping_add(8)) {
                    p = p.add(10);
                    iv3 = iv3.wrapping_add(1);
                    if DAT_007330ac <= iv3 {
                        return;
                    }
                }
                let base = (&raw mut DAT_00733100).cast::<i8>();
                let src = param_2.wrapping_add(0x1c) as usize as *const i8;
                let n1 = str_count(src);
                let d1 = base
                    .offset(0x10u32.wrapping_add((iv3 as u32).wrapping_mul(0x28)) as i32 as isize);
                copy_str(src, d1, n1);
                let n2 = str_count(d1);
                let src2 =
                    (n2.wrapping_add(0x1c).wrapping_add(param_2 as u32)) as usize as *const i8;
                let n3 = str_count(src2);
                let d2 = base.offset((iv3 as u32).wrapping_mul(0x28) as i32 as isize);
                copy_str(src2, d2, n3);
                base.offset((iv3 as u32).wrapping_mul(0x28).wrapping_add(0x1c) as i32 as isize)
                    .write(0);
                base.offset((iv3 as u32).wrapping_mul(0x28).wrapping_add(0xc) as i32 as isize)
                    .write(0);
            }
        }
    }
}
